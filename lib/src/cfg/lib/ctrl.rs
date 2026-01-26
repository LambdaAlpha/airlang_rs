use std::collections::HashMap;

use const_format::concatcp;
use num_traits::Signed;
use num_traits::ToPrimitive;

use super::DynImpl;
use super::abort_free;
use crate::bug;
use crate::cfg::CfgMod;
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

struct OutputCtrlFlow {
    output: Val,
    ctrl_flow: CtrlFlow,
}

impl Block {
    fn parse(tag: &str, cfg: &mut Cfg, val: Val) -> Result<Self, Val> {
        let Val::List(list) = val else {
            return Err(bug!(cfg, "{tag}: expected block to be a list, but got {val:?}"));
        };
        let list = List::from(list);
        let mut statements = Vec::with_capacity(list.len());
        for statement in list {
            statements.push(Statement::parse(tag, cfg, statement)?);
        }
        Ok(Block { statements })
    }

    fn flow(self, tag: &str, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> OutputCtrlFlow {
        let mut output = Val::default();
        for statement in self.statements {
            if cfg.is_aborted() {
                return OutputCtrlFlow { output: Val::default(), ctrl_flow: CtrlFlow::Return };
            }
            match statement {
                Statement::Normal(val) => {
                    output = Eval.dyn_call(cfg, ctx.reborrow(), val);
                }
                Statement::Condition { ctrl_flow, condition, body } => {
                    let condition = Eval.dyn_call(cfg, ctx.reborrow(), condition);
                    let Val::Bit(condition) = condition else {
                        bug!(
                            cfg,
                            "{tag}: expected block condition to be a bit, but got {condition:?}"
                        );
                        return OutputCtrlFlow {
                            output: Val::default(),
                            ctrl_flow: CtrlFlow::Return,
                        };
                    };
                    if *condition {
                        let output = Eval.dyn_call(cfg, ctx, body);
                        return OutputCtrlFlow { output, ctrl_flow };
                    }
                    output = Val::default();
                }
            }
        }
        OutputCtrlFlow { output, ctrl_flow: CtrlFlow::Continue }
    }

    fn eval(self, tag: &str, cfg: &mut Cfg, ctx: DynRef<Val>) -> Val {
        self.flow(tag, cfg, ctx).output
    }
}

impl Statement {
    fn parse(tag: &str, cfg: &mut Cfg, val: Val) -> Result<Self, Val> {
        let Val::Call(call) = val else {
            return Ok(Statement::Normal(val));
        };
        let Val::Key(s) = &call.func else {
            return Ok(Statement::Normal(Val::Call(call)));
        };
        let Some(ctrl_flow) = CtrlFlow::parse(s) else {
            return Ok(Statement::Normal(Val::Call(call)));
        };
        let call = Call::from(call);
        let Val::Pair(pair) = call.input else {
            return Err(bug!(
                cfg,
                "{tag}: expect statement condition input to be a pair, but got {:?}",
                call.input
            ));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let body = pair.right;
        let statement = Statement::Condition { ctrl_flow, condition, body };
        Ok(statement)
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
    DynImpl { free: abort_free(DO), dyn_: fn_do }.build_with(true)
}

fn fn_do(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(block) = Block::parse(DO, cfg, input) else {
        return Val::default();
    };
    block.eval(DO, cfg, ctx)
}

pub fn test() -> MutPrimFuncVal {
    DynImpl { free: abort_free(TEST), dyn_: fn_test }.build_with(true)
}

fn fn_test(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(test) = Test::parse(cfg, input) else {
        return Val::default();
    };
    test.eval(cfg, ctx)
}

struct Test {
    condition: Val,
    branch_then: Block,
    branch_else: Block,
}

impl Test {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{TEST}: expect input to be a pair, but got {input:?}"));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let Val::Pair(branches) = pair.right else {
            return Err(bug!(
                cfg,
                "{TEST}: expect input.right to be a pair, but got {:?}",
                pair.right
            ));
        };
        let branches = Pair::from(branches);
        let branch_then = Block::parse(TEST, cfg, branches.left)?;
        let branch_else = Block::parse(TEST, cfg, branches.right)?;
        Ok(Test { condition, branch_then, branch_else })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let condition = Eval.dyn_call(cfg, ctx.reborrow(), self.condition);
        let Val::Bit(b) = condition else {
            return bug!(cfg, "{TEST}: expected condition to be a bit, but got {condition:?}");
        };
        let branch = if *b { self.branch_then } else { self.branch_else };
        branch.eval(TEST, cfg, ctx)
    }
}

pub fn switch() -> MutPrimFuncVal {
    DynImpl { free: abort_free(SWITCH), dyn_: fn_switch }.build_with(true)
}

fn fn_switch(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(switch) = Switch::parse(cfg, input) else {
        return Val::default();
    };
    switch.eval(cfg, ctx)
}

struct Switch {
    val: Val,
    map: HashMap<Key, Block>,
    default: Option<Block>,
}

impl Switch {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{SWITCH}: expected input to be a pair, but got {input:?}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        match pair.right {
            Val::Map(map) => {
                let map = Self::parse_block_map(cfg, map)?;
                Ok(Self { val, map, default: None })
            }
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let Val::Map(map) = pair.left else {
                    return Err(bug!(
                        cfg,
                        "{SWITCH}: expected input.right.left to be a map, but got {:?}",
                        pair.left
                    ));
                };
                let map = Self::parse_block_map(cfg, map)?;
                let default = Some(Block::parse(SWITCH, cfg, pair.right)?);
                Ok(Self { val, map, default })
            }
            v => Err(bug!(
                cfg,
                "{SWITCH}: expected input.right to be a map or a pair, but got {v:?}"
            )),
        }
    }

    fn parse_block_map(cfg: &mut Cfg, map: MapVal) -> Result<HashMap<Key, Block>, Val> {
        let mut block_map = HashMap::<Key, Block>::new();
        for (k, v) in Map::from(map) {
            block_map.insert(k, Block::parse(SWITCH, cfg, v)?);
        }
        Ok(block_map)
    }

    fn eval(mut self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        let Val::Key(key) = val else {
            return bug!(cfg, "{SWITCH}: expected input.left to be a key, but got {val:?}");
        };
        let Some(body) = self.map.remove(&key).or(self.default) else {
            return Val::default();
        };
        body.eval(SWITCH, cfg, ctx)
    }
}

pub fn match_() -> MutPrimFuncVal {
    DynImpl { free: abort_free(MATCH), dyn_: fn_match }.build_with(true)
}

fn fn_match(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(match_) = Match::parse(cfg, input) else {
        return Val::default();
    };
    match_.eval(cfg, ctx)
}

struct Match {
    val: Val,
    arms: Vec<(Val, Block)>,
}

impl Match {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{MATCH}: expected input to be a pair, but got {input:?}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::List(list) = pair.right else {
            return Err(bug!(
                cfg,
                "{MATCH}: expected input.right to be a list, but got {:?}",
                pair.right
            ));
        };
        let arms = Self::parse_arms(cfg, list)?;
        Ok(Self { val, arms })
    }

    fn parse_arms(cfg: &mut Cfg, list: ListVal) -> Result<Vec<(Val, Block)>, Val> {
        let mut arms = Vec::with_capacity(list.len());
        for arm in List::from(list) {
            let Val::Pair(pair) = arm else {
                return Err(bug!(cfg, "{MATCH}: expected arm to be a pair, but got {arm:?}"));
            };
            let pair = Pair::from(pair);
            let block = Block::parse(MATCH, cfg, pair.right)?;
            arms.push((pair.left, block));
        }
        Ok(arms)
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        for (pattern, block) in self.arms {
            if cfg.is_aborted() {
                return Val::default();
            }
            let pattern = Eval.dyn_call(cfg, ctx.reborrow(), pattern);
            let Some(pattern) = pattern.parse(cfg, MATCH) else {
                return Val::default();
            };
            if !pattern.match_(cfg, false, MATCH, &val) {
                continue;
            }
            // todo design
            if !ctx.is_const() {
                let result = pattern.assign(cfg, MATCH, ctx.reborrow().unwrap(), val);
                if result.is_none() {
                    return Val::default();
                }
            }
            return block.eval(MATCH, cfg, ctx);
        }
        Val::default()
    }
}

pub fn loop_() -> MutPrimFuncVal {
    DynImpl { free: abort_free(LOOP), dyn_: fn_loop }.build_with(true)
}

fn fn_loop(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(loop_) = Loop::parse(cfg, input) else {
        return Val::default();
    };
    loop_.eval(cfg, ctx)
}

struct Loop {
    condition: Val,
    body: Block,
}

impl Loop {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{LOOP}: expected input to be a pair, but got {input:?}"));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let body = Block::parse(LOOP, cfg, pair.right)?;
        Ok(Self { condition, body })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        loop {
            let cond = Eval.dyn_call(cfg, ctx.reborrow(), self.condition.clone());
            let Val::Bit(bit) = cond else {
                return bug!(cfg, "{LOOP}: expected condition to be a bit, but got {cond:?}");
            };
            if !*bit {
                break;
            }
            let v = self.body.clone().flow(LOOP, cfg, ctx.reborrow());
            match v.ctrl_flow {
                CtrlFlow::Continue => {}
                CtrlFlow::Return => return v.output,
            }
        }
        Val::default()
    }
}

pub fn iterate() -> MutPrimFuncVal {
    DynImpl { free: abort_free(ITERATE), dyn_: fn_iterate }.build_with(true)
}

fn fn_iterate(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Ok(iterate) = Iterate::parse(cfg, input) else {
        return Val::default();
    };
    iterate.eval(cfg, ctx)
}

struct Iterate {
    val: Val,
    name: Key,
    body: Block,
}

impl Iterate {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{ITERATE}: expected input to be a pair, but got {input:?}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::Pair(name_body) = pair.right else {
            return Err(bug!(
                cfg,
                "{ITERATE}: expected input.right to be a pair, but got {:?}",
                pair.right
            ));
        };
        let name_body = Pair::from(name_body);
        let Val::Key(name) = name_body.left else {
            return Err(bug!(
                cfg,
                "{ITERATE}: expected input.right.left to be a key, but got {:?}",
                name_body.left
            ));
        };
        let body = Block::parse(ITERATE, cfg, name_body.right)?;
        Ok(Self { val, name, body })
    }

    fn eval(self, cfg: &mut Cfg, mut ctx: DynRef<Val>) -> Val {
        let val = Eval.dyn_call(cfg, ctx.reborrow(), self.val);
        match val {
            Val::Int(i) => {
                let i = Int::from(i);
                if i.is_negative() {
                    return bug!(
                        cfg,
                        "{ITERATE}: expected integer to be non-negative, but got {i:?}"
                    );
                }
                let Some(i) = i.to_u128() else {
                    return bug!(cfg, "{ITERATE}: unable to iterate {i:?}");
                };
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
            v => bug!(cfg, "{ITERATE}: expected input.left to be iterable, but got {v:?}"),
        }
    }
}

fn iterate_val<ValIter>(
    cfg: &mut Cfg, mut ctx: DynRef<Val>, body: Block, name: Key, values: ValIter,
) -> Val
where ValIter: Iterator<Item = Val> {
    for val in values {
        if cfg.is_aborted() {
            return Val::default();
        }
        if !ctx.is_const() {
            let _ = ctx.reborrow().unwrap().set(cfg, name.clone(), val);
        }
        let v = body.clone().flow(ITERATE, cfg, ctx.reborrow());
        match v.ctrl_flow {
            CtrlFlow::Continue => {}
            CtrlFlow::Return => return v.output,
        }
    }
    Val::default()
}
