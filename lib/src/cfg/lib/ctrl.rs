use std::collections::HashMap;

use const_format::concatcp;
use log::error;
use num_traits::Signed;
use num_traits::ToPrimitive;

use super::DynPrimFn;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::adapter::CallPrimAdapter;
use crate::cfg::adapter::PrimAdapter;
use crate::cfg::adapter::SymbolAdapter;
use crate::cfg::adapter::id_adapter;
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_input;
use crate::cfg::lib::ctx::pattern::PatternAssign;
use crate::cfg::lib::ctx::pattern::PatternMatch;
use crate::cfg::lib::ctx::pattern::PatternParse;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::MutFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
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
        CoreCfg::extend_adapter(cfg, &self.do_.id, id_adapter());
        self.do_.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.test.id, id_adapter());
        self.test.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.switch.id, id_adapter());
        self.switch.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.match_.id, id_adapter());
        self.match_.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.loop_.id, id_adapter());
        self.loop_.extend(cfg);
        CoreCfg::extend_adapter(cfg, &self.iterate.id, id_adapter());
        self.iterate.extend(cfg);
    }
}

const CONTINUE: &str = "continue";
const RETURN: &str = "return";
const CTRL_FLOW_CONTINUE: &str = concatcp!(SYMBOL_LITERAL_CHAR, CONTINUE);
const CTRL_FLOW_RETURN: &str = concatcp!(SYMBOL_LITERAL_CHAR, RETURN);

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

    fn flow(self, cfg: &mut Cfg, ctx: &mut Val) -> (Val, Result<CtrlFlow, ()>) {
        let mut output = Val::default();
        for statement in self.statements {
            match statement {
                Statement::Normal(val) => {
                    output = Eval.mut_call(cfg, ctx, val);
                }
                Statement::Condition { ctrl_flow, condition, body } => {
                    let condition = Eval.mut_call(cfg, ctx, condition);
                    let Val::Bit(condition) = condition else {
                        error!("condition {condition:?} should be a bit");
                        return (Val::default(), Err(()));
                    };
                    if *condition {
                        let output = Eval.mut_call(cfg, ctx, body);
                        return (output, Ok(ctrl_flow));
                    }
                    output = Val::default();
                }
            }
        }
        (output, Ok(CtrlFlow::Continue))
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        self.flow(cfg, ctx).0
    }
}

impl Statement {
    fn parse(val: Val) -> Option<Self> {
        let Val::Call(call) = val else {
            return Some(Statement::Normal(val));
        };
        let Val::Symbol(s) = &call.func else {
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
        let condition = pair.first;
        let body = pair.second;
        let statement = Statement::Condition { ctrl_flow, condition, body };
        Some(statement)
    }
}

impl CtrlFlow {
    fn parse(str: &str) -> Option<Self> {
        let ctrl_flow = match str {
            CTRL_FLOW_RETURN => Self::Return,
            CTRL_FLOW_CONTINUE => Self::Continue,
            _ => return None,
        };
        Some(ctrl_flow)
    }
}

pub fn do_() -> MutPrimFuncVal {
    DynPrimFn { id: "control.do", f: mut_impl(fn_do) }.mut_()
}

fn fn_do(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Some(block) = Block::parse(input) else {
        return illegal_input(cfg);
    };
    block.eval(cfg, ctx)
}

pub fn test() -> MutPrimFuncVal {
    DynPrimFn { id: "control.test", f: mut_impl(fn_test) }.mut_()
}

fn fn_test(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
        let condition = pair.first;
        let Val::Pair(branches) = pair.second else {
            error!("input.second {:?} should be a pair", pair.second);
            return None;
        };
        let branches = Pair::from(branches);
        let branch_then = Block::parse(branches.first)?;
        let branch_else = Block::parse(branches.second)?;
        Some(Test { condition, branch_then, branch_else })
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let condition = Eval.mut_call(cfg, ctx, self.condition);
        let Val::Bit(b) = condition else {
            error!("condition {condition:?} should be a bit");
            return illegal_input(cfg);
        };
        let branch = if *b { self.branch_then } else { self.branch_else };
        branch.eval(cfg, ctx)
    }
}

pub fn switch() -> MutPrimFuncVal {
    DynPrimFn { id: "control.switch", f: mut_impl(fn_switch) }.mut_()
}

fn fn_switch(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Some(switch) = Switch::parse(input) else {
        return illegal_input(cfg);
    };
    switch.eval(cfg, ctx)
}

struct Switch {
    val: Val,
    map: HashMap<Val, Block>,
    default: Option<Block>,
}

impl Switch {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let val = pair.first;
        match pair.second {
            Val::Map(map) => {
                let map = Self::parse_block_map(map)?;
                Some(Self { val, map, default: None })
            }
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let Val::Map(map) = pair.first else {
                    error!("input.second.first {:?} should be a map", pair.first);
                    return None;
                };
                let map = Self::parse_block_map(map)?;
                let default = Some(Block::parse(pair.second)?);
                Some(Self { val, map, default })
            }
            v => {
                error!("input.second {v:?} should be a map or a pair");
                None
            }
        }
    }

    fn parse_block_map(map: MapVal) -> Option<HashMap<Val, Block>> {
        Map::from(map).into_iter().map(|(k, v)| Some((k, Block::parse(v)?))).collect()
    }

    fn eval(mut self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.mut_call(cfg, ctx, self.val);
        let Some(body) = self.map.remove(&val).or(self.default) else {
            return illegal_input(cfg);
        };
        body.eval(cfg, ctx)
    }
}

pub fn match_() -> MutPrimFuncVal {
    DynPrimFn { id: "control.match", f: mut_impl(fn_match) }.mut_()
}

fn fn_match(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
        let val = pair.first;
        let Val::List(list) = pair.second else {
            error!("input.second {:?} should be a list", pair.second);
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
                let block = Block::parse(pair.second)?;
                Some((pair.first, block))
            })
            .collect()
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.mut_call(cfg, ctx, self.val);
        let adapter = PrimAdapter::new(SymbolAdapter::Literal, CallPrimAdapter::Form);
        for (pattern, block) in self.arms {
            let pattern = adapter.mut_call(cfg, ctx, pattern);
            let Some(pattern) = pattern.parse() else {
                error!("parse pattern failed");
                return fail(cfg);
            };
            if pattern.match_(&val) {
                pattern.assign(ctx, val);
                return block.eval(cfg, ctx);
            }
        }
        Val::default()
    }
}

pub fn loop_() -> MutPrimFuncVal {
    DynPrimFn { id: "control.loop", f: mut_impl(fn_loop) }.mut_()
}

fn fn_loop(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
        let condition = pair.first;
        let body = Block::parse(pair.second)?;
        Some(Self { condition, body })
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        loop {
            let cond = Eval.mut_call(cfg, ctx, self.condition.clone());
            let Val::Bit(bit) = cond else {
                error!("condition {cond:?} should be a bit");
                return fail(cfg);
            };
            if !*bit {
                break;
            }
            let (output, ctrl_flow) = self.body.clone().flow(cfg, ctx);
            match ctrl_flow {
                Ok(CtrlFlow::Continue) => {}
                Ok(CtrlFlow::Return) => return output,
                Err(()) => return Val::default(),
            }
        }
        Val::default()
    }
}

pub fn iterate() -> MutPrimFuncVal {
    DynPrimFn { id: "control.iterate", f: mut_impl(fn_iterate) }.mut_()
}

fn fn_iterate(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Some(iterate) = Iterate::parse(input) else {
        return illegal_input(cfg);
    };
    iterate.eval(cfg, ctx)
}

struct Iterate {
    val: Val,
    name: Symbol,
    body: Block,
}

impl Iterate {
    fn parse(input: Val) -> Option<Self> {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            return None;
        };
        let pair = Pair::from(pair);
        let val = pair.first;
        let Val::Pair(name_body) = pair.second else {
            error!("input.second {:?} should be a pair", pair.second);
            return None;
        };
        let name_body = Pair::from(name_body);
        let Val::Symbol(name) = name_body.first else {
            error!("input.first {:?} should be a symbol", name_body.first);
            return None;
        };
        let body = Block::parse(name_body.second)?;
        Some(Self { val, name, body })
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.mut_call(cfg, ctx, self.val);
        match val {
            Val::Int(i) => {
                let i = Int::from(i);
                if i.is_negative() {
                    error!("{i:?} should be positive");
                    return fail(cfg);
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
            Val::Symbol(s) => {
                let iter = s.char_indices().map(|(start, c)| {
                    let symbol = Symbol::from_str_unchecked(&s[start .. start + c.len_utf8()]);
                    Val::Symbol(symbol)
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
                    let pair = Pair::new(pair.0, pair.1);
                    Val::Pair(pair.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            }
            _ => fail(cfg),
        }
    }
}

fn iterate_val<ValIter>(
    cfg: &mut Cfg, ctx: &mut Val, body: Block, name: Symbol, values: ValIter,
) -> Val
where ValIter: Iterator<Item = Val> {
    for val in values {
        let _ = ctx.set(name.clone(), val);
        let (output, ctrl_flow) = body.clone().flow(cfg, ctx);
        match ctrl_flow {
            Ok(CtrlFlow::Continue) => {}
            Ok(CtrlFlow::Return) => return output,
            Err(()) => return Val::default(),
        }
    }
    Val::default()
}
