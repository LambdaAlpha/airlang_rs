use std::collections::HashMap;
use std::ops::Deref;

use const_format::concatcp;
use num_traits::Signed;
use num_traits::ToPrimitive;

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
use crate::semantics::func::CtxMutInputRawFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Clone)]
pub struct CtrlLib {
    pub do_: PrimFuncVal,
    pub test: PrimFuncVal,
    pub switch: PrimFuncVal,
    pub match_: PrimFuncVal,
    pub loop_: PrimFuncVal,
    pub iterate: PrimFuncVal,
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
            do_: CtxMutInputRawFunc { fn_: do_ }.build(),
            test: CtxMutInputRawFunc { fn_: test }.build(),
            switch: CtxMutInputRawFunc { fn_: switch }.build(),
            match_: CtxMutInputRawFunc { fn_: match_ }.build(),
            loop_: CtxMutInputRawFunc { fn_: loop_ }.build(),
            iterate: CtxMutInputRawFunc { fn_: iterate }.build(),
        }
    }
}

impl CfgMod for CtrlLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, DO, self.do_);
        extend_func(cfg, TEST, self.test);
        extend_func(cfg, SWITCH, self.switch);
        extend_func(cfg, MATCH, self.match_);
        extend_func(cfg, LOOP, self.loop_);
        extend_func(cfg, ITERATE, self.iterate);
    }
}

const TRY: &str = concatcp!(PREFIX_ID, "try");

#[derive(Clone)]
struct Block {
    statements: Vec<Statement>,
}

#[derive(Clone)]
struct Statement {
    try_: bool,
    body: Val,
}

impl Block {
    fn parse(cfg: &mut Cfg, tag: &str, val: Val) -> Result<Self, Val> {
        let Val::List(list) = val else {
            return Err(bug!(cfg, "{tag}: expected block to be a list, but got {val}"));
        };
        let list = List::from(list);
        let mut statements = Vec::with_capacity(list.len());
        for statement in list {
            statements.push(Statement::parse(cfg, tag, statement)?);
        }
        Ok(Block { statements })
    }

    fn flow(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val) -> Option<Val> {
        let mut output = Val::default();
        for statement in self.statements {
            if cfg.is_aborted() {
                return None;
            }
            output = Eval.call(cfg, ctx, statement.body);
            if !statement.try_ {
                continue;
            }
            match output {
                Val::Cell(cell) => return Some(Cell::from(cell).value),
                Val::Unit(_) => {},
                output => {
                    bug!(cfg, "{tag}: expected body of {TRY} to be a cell or unit, \
                        but got {output:?}");
                    return None;
                },
            }
        }
        Some(output)
    }
}

impl Statement {
    fn parse(cfg: &mut Cfg, tag: &str, val: Val) -> Result<Self, Val> {
        let try_ = false;
        let Val::Call(call) = val else {
            return Ok(Statement { try_, body: val });
        };
        let Val::Key(s) = &call.func else {
            return Ok(Statement { try_, body: Val::Call(call) });
        };
        if !s.starts_with(PREFIX_ID) {
            return Ok(Statement { try_, body: Val::Call(call) });
        }
        if s.deref() != TRY {
            return Err(bug!(cfg, "{tag}: expect {TRY}, but got {s}",));
        }
        let call = Call::from(call);
        Ok(Statement { try_: true, body: call.input })
    }
}

pub fn do_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Ok(block) = Block::parse(cfg, DO, input) else {
        return Val::default();
    };
    block.flow(cfg, DO, ctx).unwrap_or_default()
}

pub fn test(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Ok(test) = Test::parse(cfg, input) else {
        return Val::default();
    };
    test.eval(cfg, ctx)
}

struct Test {
    condition: Val,
    body: Block,
    default: Option<Block>,
}

impl Test {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{TEST}: expect input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        match pair.right {
            Val::Pair(branches) => {
                let branches = Pair::from(branches);
                let body = Block::parse(cfg, TEST, branches.left)?;
                let default = Block::parse(cfg, TEST, branches.right)?;
                Ok(Test { condition, body, default: Some(default) })
            },
            body => {
                let body = Block::parse(cfg, TEST, body)?;
                Ok(Test { condition, body, default: None })
            },
        }
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let condition = Eval.call(cfg, ctx, self.condition);
        let Val::Bit(b) = condition else {
            return bug!(cfg, "{TEST}: expected condition to be a bit, but got {condition}");
        };
        if *b {
            return self.body.flow(cfg, TEST, ctx).unwrap_or_default();
        }
        let Some(default) = self.default else {
            return Val::default();
        };
        default.flow(cfg, TEST, ctx).unwrap_or_default()
    }
}

pub fn switch(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
            return Err(bug!(cfg, "{SWITCH}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        match pair.right {
            Val::Map(map) => {
                let map = Self::parse_block_map(cfg, map)?;
                Ok(Self { val, map, default: None })
            },
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let Val::Map(map) = pair.left else {
                    return Err(bug!(cfg, "{SWITCH}: expected input.right.left to be a map, \
                        but got {}", pair.left));
                };
                let map = Self::parse_block_map(cfg, map)?;
                let default = Some(Block::parse(cfg, SWITCH, pair.right)?);
                Ok(Self { val, map, default })
            },
            v => {
                Err(bug!(cfg, "{SWITCH}: expected input.right to be a map or a pair, but got {v}"))
            },
        }
    }

    fn parse_block_map(cfg: &mut Cfg, map: MapVal) -> Result<HashMap<Key, Block>, Val> {
        let mut block_map = HashMap::<Key, Block>::new();
        for (k, v) in Map::from(map) {
            block_map.insert(k, Block::parse(cfg, SWITCH, v)?);
        }
        Ok(block_map)
    }

    fn eval(mut self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.call(cfg, ctx, self.val);
        let Val::Key(key) = val else {
            return bug!(cfg, "{SWITCH}: expected input.left to be a key, but got {val}");
        };
        let Some(body) = self.map.remove(&key).or(self.default) else {
            return Val::default();
        };
        body.flow(cfg, SWITCH, ctx).unwrap_or_default()
    }
}

pub fn match_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
            return Err(bug!(cfg, "{MATCH}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::List(list) = pair.right else {
            return Err(bug!(cfg, "{MATCH}: expected input.right to be a list, \
                but got {}", pair.right));
        };
        let arms = Self::parse_arms(cfg, list)?;
        Ok(Self { val, arms })
    }

    fn parse_arms(cfg: &mut Cfg, list: ListVal) -> Result<Vec<(Val, Block)>, Val> {
        let mut arms = Vec::with_capacity(list.len());
        for arm in List::from(list) {
            let Val::Pair(pair) = arm else {
                return Err(bug!(cfg, "{MATCH}: expected arm to be a pair, but got {arm}"));
            };
            let pair = Pair::from(pair);
            let block = Block::parse(cfg, MATCH, pair.right)?;
            arms.push((pair.left, block));
        }
        Ok(arms)
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.call(cfg, ctx, self.val);
        for (pattern, block) in self.arms {
            if cfg.is_aborted() {
                return Val::default();
            }
            let pattern = Eval.call(cfg, ctx, pattern);
            let Some(pattern) = pattern.parse(cfg, MATCH) else {
                return Val::default();
            };
            if !pattern.match_(cfg, false, MATCH, &val) {
                continue;
            }
            // todo design
            let result = pattern.assign(cfg, MATCH, ctx, val);
            if result.is_none() {
                return Val::default();
            }
            return block.flow(cfg, MATCH, ctx).unwrap_or_default();
        }
        Val::default()
    }
}

pub fn loop_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
            return Err(bug!(cfg, "{LOOP}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let body = Block::parse(cfg, LOOP, pair.right)?;
        Ok(Self { condition, body })
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        loop {
            let cond = Eval.call(cfg, ctx, self.condition.clone());
            let Val::Bit(bit) = cond else {
                return bug!(cfg, "{LOOP}: expected condition to be a bit, but got {cond}");
            };
            if !*bit {
                break;
            }
            let Some(output) = self.body.clone().flow(cfg, LOOP, ctx) else {
                return Val::default();
            };
            match output {
                Val::Cell(cell) => return Cell::from(cell).value,
                Val::Unit(_) => {},
                output => {
                    bug!(cfg, "{LOOP}: expected return value of body to be a cell or unit, \
                        but got {output:?}");
                    return Val::default();
                },
            }
        }
        Val::default()
    }
}

pub fn iterate(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
            return Err(bug!(cfg, "{ITERATE}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::Pair(name_body) = pair.right else {
            return Err(bug!(cfg, "{ITERATE}: expected input.right to be a pair, \
                but got {}", pair.right));
        };
        let name_body = Pair::from(name_body);
        let Val::Key(name) = name_body.left else {
            return Err(bug!(cfg, "{ITERATE}: expected input.right.left to be a key, \
                but got {}", name_body.left));
        };
        let body = Block::parse(cfg, ITERATE, name_body.right)?;
        Ok(Self { val, name, body })
    }

    fn eval(self, cfg: &mut Cfg, ctx: &mut Val) -> Val {
        let val = Eval.call(cfg, ctx, self.val);
        match val {
            Val::Int(i) => {
                let i = Int::from(i);
                if i.is_negative() {
                    return bug!(cfg, "{ITERATE}: expected integer to be non-negative, \
                        but got {i}");
                }
                let Some(i) = i.to_u128() else {
                    return bug!(cfg, "{ITERATE}: unable to iterate {i}");
                };
                let iter = (0 .. i).map(|i| {
                    let i = Int::from(i);
                    Val::Int(i.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Byte(byte) => {
                let iter = byte.iter().map(|byte| {
                    let byte = Byte::from(vec![*byte]);
                    Val::Byte(byte.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Key(key) => {
                let iter = key.char_indices().map(|(start, c)| {
                    let key = Key::from_str_unchecked(&key[start .. start + c.len_utf8()]);
                    Val::Key(key)
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Text(t) => {
                let iter = t.chars().map(|c| {
                    let text = Text::from(c.to_string());
                    Val::Text(text.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::List(list) => {
                let list = List::from(list);
                let iter = list.into_iter();
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Map(map) => {
                let map = Map::from(map);
                let iter = map.into_iter().map(|pair| {
                    let pair = Pair::new(Val::Key(pair.0), pair.1);
                    Val::Pair(pair.into())
                });
                iterate_val(cfg, ctx, self.body, self.name, iter)
            },
            v => bug!(cfg, "{ITERATE}: expected input.left to be iterable, but got {v}"),
        }
    }
}

fn iterate_val<ValIter>(
    cfg: &mut Cfg, ctx: &mut Val, body: Block, name: Key, values: ValIter,
) -> Val
where ValIter: Iterator<Item = Val> {
    for val in values {
        if cfg.is_aborted() {
            return Val::default();
        }
        if ctx.set(cfg, name.clone(), val).is_none() {
            return Val::default();
        }
        let Some(output) = body.clone().flow(cfg, ITERATE, ctx) else {
            return Val::default();
        };
        match output {
            Val::Cell(cell) => return Cell::from(cell).value,
            Val::Unit(_) => {},
            output => {
                bug!(cfg, "{ITERATE}: expected return value of body to be a cell or unit, \
                    but got {output:?}");
                return Val::default();
            },
        }
    }
    Val::default()
}
