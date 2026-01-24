use std::collections::HashMap;

use const_format::concatcp;
use log::error;
use num_traits::Signed;
use num_traits::ToPrimitive;

use super::DynPrimFn;
use super::dyn_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::ctx::pattern::PatternAssign;
use crate::cfg::lib::ctx::pattern::PatternMatch;
use crate::cfg::lib::ctx::pattern::PatternParse;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::DynRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Clone)]
pub struct CtrlLib {
    pub do_: MutPrimFuncVal,
    pub test: MutPrimFuncVal,
    pub switch: MutPrimFuncVal,
    pub match_: MutPrimFuncVal,
    pub loop_: MutPrimFuncVal,
    pub iterate: MutPrimFuncVal,
}

const CTRL: &str = "control";

pub const DO: &str = concatcp!(PREFIX_ID, CTRL, ".do");
pub const TEST: &str = concatcp!(PREFIX_ID, CTRL, ".test");
pub const SWITCH: &str = concatcp!(PREFIX_ID, CTRL, ".switch");
pub const MATCH: &str = concatcp!(PREFIX_ID, CTRL, ".match");
pub const LOOP: &str = concatcp!(PREFIX_ID, CTRL, ".loop");
pub const ITERATE: &str = concatcp!(PREFIX_ID, CTRL, ".iterate");

impl Default for CtrlLib {
    fn default() -> Self {
        CtrlLib {
            do_: do_(),
            test: test(),
            switch: switch(),
            match_: match_(),
            loop_: loop_(),
            iterate: iterate(),
        }
    }
}

impl CfgMod for CtrlLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, DO, self.do_);
        extend_func(cfg, TEST, self.test);
        extend_func(cfg, SWITCH, self.switch);
        extend_func(cfg, MATCH, self.match_);
        extend_func(cfg, LOOP, self.loop_);
        extend_func(cfg, ITERATE, self.iterate);
    }
}

const CONTINUE: &str = concatcp!(PREFIX_ID, "continue");
const RETURN: &str = concatcp!(PREFIX_ID, "return");

#[derive(Clone)]
struct Block {
    statements: Vec<Statement>,
}

#[derive(Clone)]
enum Statement {
    Normal(Val),
    Condition { ctrl_flow: CtrlFlow, condition: Val, body: Val },
}

#[derive(Copy, Clone)]
enum CtrlFlow {
    Continue,
    Return,
}

impl Block {
    fn parse(val: Val) -> Option<Self> {
        let Val::List(list) = val else {
            error!("input {val:?} should be a list");
            return None;
        };
        let list = List::from(list);
        let items = list.into_iter().map(Statement::parse).collect::<Option<_>>()?;
        Some(Block { statements: items })
    }

    fn flow(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> (Val, CtrlFlow) {
        let mut output = Val::default();
        for statement in self.statements {
            if cfg.is_aborted() {
                error!("aborted");
                return (Val::default(), CtrlFlow::Return);
            }
            match statement {
                Statement::Normal(val) => {
                    output = Eval.dyn_call(cfg, ctx.reborrow(), val);
                }
                Statement::Condition { ctrl_flow, condition, body } => {
                    let condition = Eval.dyn_call(cfg, ctx.reborrow(), condition);
                    let Val::Bit(condition) = condition else {
                        error!("condition {condition:?} should be a bit");
                        abort_bug_with_msg(cfg, "block condition should be a bit");
                        return (Val::default(), CtrlFlow::Return);
                    };
                    if *condition {
                        let output = Eval.dyn_call(cfg, ctx, body);
                        return (output, ctrl_flow);
                    }
                    output = Val::default();
                }
            }
        }
        (output, CtrlFlow::Continue)
    }

    fn eval(self, cfg: &mut Cfg, ctx: DynRef<Val>) -> Val {
        self.flow(cfg, ctx).0
    }
}

impl Statement {
    fn parse(val: Val) -> Option<Self> {
        let Val::Call(call) = val else {
            return Some(Statement::Normal(val));
        };
        let Val::Key(s) = &call.func else {
            return Some(Statement::Normal(Val::Call(call)));
        };
        let Some(ctrl_flow) = CtrlFlow::parse(s) else {
            return Some(Statement::Normal(Val::Call(call)));
        };
        let call = Call::from(call);
        let Val::Pair(pair) = call.input else {
            error!("call.input {:?} should be a pair", call.input);
            return None;
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let body = pair.right;
        let statement = Statement::Condition { ctrl_flow, condition, body };
        Some(statement)
    }
}

impl CtrlFlow {
    fn parse(str: &str) -> Option<Self> {
        let ctrl_flow = match str {
            RETURN => Self::Return,
            CONTINUE => Self::Continue,
            _ => return None,
        };
        Some(ctrl_flow)
    }
}

pub fn do_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_do) }.mut_()
}

fn fn_do(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(block) = Block::parse(input) else {
        return illegal_input(cfg);
    };
    block.eval(cfg, ctx)
}

pub fn test() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_test) }.mut_()
}

fn fn_test(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(test) = Test::parse(input) else {
        return illegal_input(cfg);
    };
    test.eval(cfg, ctx)
}

struct Test {
    condition: Val,
    branch_then: Block,
    branch_else: Block,
}

impl Test {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let Val::Pair(branches) = pair.right else {
            error!("input.right {:?} should be a pair", pair.right);
            return None;
        };
        let branches = Pair::from(branches);
        let branch_then = Block::parse(branches.left)?;
        let branch_else = Block::parse(branches.right)?;
        Some(Test { condition, branch_then, branch_else })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let condition = Eval.dyn_call(cfg, ctx.reborrow(), self.condition);
        let Val::Bit(b) = condition else {
            error!("condition {condition:?} should be a bit");
            return illegal_input(cfg);
        };
        let branch = if *b { self.branch_then } else { self.branch_else };
        branch.eval(cfg, ctx)
    }
}

pub fn switch() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_switch) }.mut_()
}

fn fn_switch(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(switch) = Switch::parse(input) else {
        return illegal_input(cfg);
    };
    switch.eval(cfg, ctx)
}

struct Switch {
    val: Val,
    map: HashMap<Key, Block>,
    default: Option<Block>,
}

impl Switch {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        match pair.right {
            Val::Map(map) => {
                let map = Self::parse_block_map(map)?;
                Some(Self { val, map, default: None })
            }
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let Val::Map(map) = pair.left else {
                    error!("input.right.left {:?} should be a map", pair.left);
                    return None;
                };
                let map = Self::parse_block_map(map)?;
                let default = Some(Block::parse(pair.right)?);
                Some(Self { val, map, default })
            }
            v => {
                error!("input.right {v:?} should be a map or a pair");
                None
            }
        }
    }

    fn parse_block_map(map: MapVal) -> Option<HashMap<Key, Block>> {
        Map::from(map).into_iter().map(|(k, v)| Some((k, Block::parse(v)?))).collect()
    }

    fn eval(mut self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        let Val::Key(key) = val else {
            error!("input.left {val:?} should be a key");
            return illegal_input(cfg);
        };
        let Some(body) = self.map.remove(&key).or(self.default) else {
            return Val::default();
        };
        body.eval(cfg, ctx)
    }
}

pub fn match_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_match) }.mut_()
}

fn fn_match(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(match_) = Match::parse(input) else {
        return illegal_input(cfg);
    };
    match_.eval(cfg, ctx)
}

struct Match {
    val: Val,
    arms: Vec<(Val, Block)>,
}

impl Match {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::List(list) = pair.right else {
            error!("input.right {:?} should be a list", pair.right);
            return None;
        };
        let arms = Self::parse_arms(list)?;
        Some(Self { val, arms })
    }

    fn parse_arms(list: ListVal) -> Option<Vec<(Val, Block)>> {
        List::from(list)
            .into_iter()
            .map(|arm| {
                let Val::Pair(pair) = arm else {
                    error!("match arm {arm:?} should be a pair");
                    return None;
                };
                let pair = Pair::from(pair);
                let block = Block::parse(pair.right)?;
                Some((pair.left, block))
            })
            .collect()
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        for (pattern, block) in self.arms {
            if cfg.is_aborted() {
                error!("aborted");
                return Val::default();
            }
            let pattern = Eval.dyn_call(cfg, ctx.reborrow(), pattern);
            let Some(pattern) = pattern.parse() else {
                error!("parse pattern failed");
                return abort_bug_with_msg(cfg, "match parsing pattern failed");
            };
            if pattern.match_(&val) {
                // todo design
                if !ctx.is_const() {
                    pattern.assign(ctx.reborrow().unwrap(), val);
                }
                return block.eval(cfg, ctx);
            }
        }
        Val::default()
    }
}

pub fn loop_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_loop) }.mut_()
}

fn fn_loop(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(loop_) = Loop::parse(input) else {
        return illegal_input(cfg);
    };
    loop_.eval(cfg, ctx)
}

struct Loop {
    condition: Val,
    body: Block,
}

impl Loop {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let body = Block::parse(pair.right)?;
        Some(Self { condition, body })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        loop {
            let cond = Eval.dyn_call(cfg, ctx.reborrow(), self.condition.clone());
            let Val::Bit(bit) = cond else {
                error!("condition {cond:?} should be a bit");
                return abort_bug_with_msg(cfg, "loop condition should be a bit");
            };
            if !*bit {
                break;
            }
            let (output, ctrl_flow) = self.body.clone().flow(cfg, ctx.reborrow());
            match ctrl_flow {
                CtrlFlow::Continue => {}
                CtrlFlow::Return => return output,
            }
        }
        Val::default()
    }
}

pub fn iterate() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_iterate) }.mut_()
}

fn fn_iterate(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Some(iterate) = Iterate::parse(input) else {
        return illegal_input(cfg);
    };
    iterate.eval(cfg, ctx)
}

struct Iterate {
    val: Val,
    name: Key,
    body: Block,
}

impl Iterate {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::Pair(name_body) = pair.right else {
            error!("input.right {:?} should be a pair", pair.right);
            return None;
        };
        let name_body = Pair::from(name_body);
        let Val::Key(name) = name_body.left else {
            error!("input.left {:?} should be a key", name_body.left);
            return None;
        };
        let body = Block::parse(name_body.right)?;
        Some(Self { val, name, body })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        match val {
            Val::Int(i) => {
                let i = Int::from(i);
                if i.is_negative() {
                    error!("{i:?} should be non-negative");
                    return abort_bug_with_msg(cfg, "iterate int should be non-negative");
                }
                let Some(i) = i.to_u128() else { panic!("iterate on super big int {i:?}!!!") };
                let iter = (0 .. i).map(|i| {
                    let i = Int::from(i);
                    Val::Int(i.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            Val::Byte(byte) => {
                let iter = byte.iter().map(|byte| {
                    let byte = Byte::from(vec![*byte]);
                    Val::Byte(byte.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            Val::Key(key) => {
                let iter = key.char_indices().map(|(start, c)| {
                    let key = Key::from_str_unchecked(&key[start .. start + c.len_utf8()]);
                    Val::Key(key)
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            Val::Text(t) => {
                let iter = t.chars().map(|c| {
                    let text = Text::from(c.to_string());
                    Val::Text(text.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            Val::List(list) => {
                let list = List::from(list);
                let iter = list.into_iter();
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            Val::Map(map) => {
                let map = Map::from(map);
                let iter = map.into_iter().map(|pair| {
                    let pair = Pair::new(Val::Key(pair.0), pair.1);
                    Val::Pair(pair.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            _ => abort_bug_with_msg(cfg, "iterate on unsupported type"),
        }
    }
}

fn iterate_val<ValIter>(
    cfg: &mut Cfg, mut ctx: DynRef<Val>, body: Block, name: Key, values: ValIter,
) -> Val
where ValIter: Iterator<Item = Val> {
    for val in values {
        if cfg.is_aborted() {
            error!("aborted");
            return Val::default();
        }
        if !ctx.is_const() {
            let _ = ctx.reborrow().unwrap().set(name.clone(), val);
        }
        let (output, ctrl_flow) = body.clone().flow(cfg, ctx.reborrow());
        match ctrl_flow {
            CtrlFlow::Continue => {}
            CtrlFlow::Return => return output,
        }
    }
    Val::default()
}
