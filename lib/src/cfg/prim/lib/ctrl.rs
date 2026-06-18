use std::collections::HashMap;
use std::ops::Deref;

use const_format::concatcp;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::cfg::prim::lib::ctx::pattern::PatternAssign;
use crate::cfg::prim::lib::ctx::pattern::PatternMatch;
use crate::cfg::prim::lib::ctx::pattern::PatternParse;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::DefaultFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Copy, Clone)]
pub struct CtrlLib {
    pub do_: PrimFuncVal,
    pub then: PrimFuncVal,
    pub branch: PrimFuncVal,
    pub match_: PrimFuncVal,
    pub loop_: PrimFuncVal,
    pub each: PrimFuncVal,
}

const CTRL: &str = "control";

pub const DO: &str = concatcp!(PREFIX_CELL, CTRL, ".do");
pub const THEN: &str = concatcp!(PREFIX_CELL, CTRL, ".then");
pub const BRANCH: &str = concatcp!(PREFIX_CELL, CTRL, ".branch");
pub const MATCH: &str = concatcp!(PREFIX_CELL, CTRL, ".match");
pub const LOOP: &str = concatcp!(PREFIX_CELL, CTRL, ".loop");
pub const EACH: &str = concatcp!(PREFIX_CELL, CTRL, ".each");

impl Default for CtrlLib {
    fn default() -> Self {
        Self {
            do_: DefaultFunc { fn_: do_ }.build(),
            then: DefaultFunc { fn_: then }.build(),
            branch: DefaultFunc { fn_: branch }.build(),
            match_: DefaultFunc { fn_: match_ }.build(),
            loop_: DefaultFunc { fn_: loop_ }.build(),
            each: DefaultFunc { fn_: each }.build(),
        }
    }
}

impl CfgMod for CtrlLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, DO, self.do_);
        extend_func(cfg, THEN, self.then);
        extend_func(cfg, BRANCH, self.branch);
        extend_func(cfg, MATCH, self.match_);
        extend_func(cfg, LOOP, self.loop_);
        extend_func(cfg, EACH, self.each);
    }
}

const TRY: &str = concatcp!(PREFIX_CELL, "try");

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

    fn flow(self, cfg: &mut Cfg, tag: &str, mut ctx: Ctx<Val>) -> Option<Val> {
        let mut output = Val::default();
        for statement in self.statements {
            output = Eval.call(cfg, ctx.reborrow(), statement.body);
            if cfg.is_aborted() {
                return None;
            }
            if !statement.try_ {
                continue;
            }
            match output {
                Val::Cell(cell) => return Some(Cell::from(cell).value),
                Val::Unit(_) => {},
                output => {
                    bug!(cfg, "{tag}: expected body of {TRY} to be a cell or unit, \
                        but got {output}");
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
        if !s.starts_with(PREFIX_CELL) {
            return Ok(Statement { try_, body: Val::Call(call) });
        }
        if s.deref() != TRY {
            return Err(bug!(cfg, "{tag}: expect {TRY}, but got {s}",));
        }
        let call = Call::from(call);
        Ok(Statement { try_: true, body: call.input })
    }
}

pub fn do_(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Ok(block) = Block::parse(cfg, DO, input) else {
        return Val::default();
    };
    block.flow(cfg, DO, ctx).unwrap_or_default()
}

pub fn then(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Ok(then) = Then::parse(cfg, input) else {
        return Val::default();
    };
    then.eval(cfg, ctx)
}

struct Then {
    condition: Bit,
    body: Block,
    default: Option<Block>,
}

impl Then {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{THEN}: expect input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let condition = pair.left;
        let Val::Bit(condition) = condition else {
            return Err(bug!(cfg, "{THEN}: expected condition to be a bit, but got {condition}"));
        };
        match pair.right {
            Val::Pair(branches) => {
                let branches = Pair::from(branches);
                let body = Block::parse(cfg, THEN, branches.left)?;
                let default = Block::parse(cfg, THEN, branches.right)?;
                Ok(Then { condition, body, default: Some(default) })
            },
            body => {
                let body = Block::parse(cfg, THEN, body)?;
                Ok(Then { condition, body, default: None })
            },
        }
    }

    fn eval(self, cfg: &mut Cfg, ctx: Ctx<Val>) -> Val {
        if *self.condition {
            return self.body.flow(cfg, THEN, ctx).unwrap_or_default();
        }
        let Some(default) = self.default else {
            return Val::default();
        };
        default.flow(cfg, THEN, ctx).unwrap_or_default()
    }
}

pub fn branch(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Ok(branch) = Branch::parse(cfg, input) else {
        return Val::default();
    };
    branch.eval(cfg, ctx)
}

struct Branch {
    val: Key,
    map: HashMap<Key, Block>,
    default: Option<Block>,
}

impl Branch {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{BRANCH}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::Key(val) = val else {
            return Err(bug!(cfg, "{BRANCH}: expected input.left to be a key, but got {val}"));
        };
        match pair.right {
            Val::Map(map) => {
                let map = Self::parse_block_map(cfg, map)?;
                Ok(Self { val, map, default: None })
            },
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let Val::Map(map) = pair.left else {
                    return Err(bug!(cfg, "{BRANCH}: expected input.right.left to be a map, \
                        but got {}", pair.left));
                };
                let map = Self::parse_block_map(cfg, map)?;
                let default = Some(Block::parse(cfg, BRANCH, pair.right)?);
                Ok(Self { val, map, default })
            },
            v => {
                Err(bug!(cfg, "{BRANCH}: expected input.right to be a map or a pair, but got {v}"))
            },
        }
    }

    fn parse_block_map(cfg: &mut Cfg, map: MapVal) -> Result<HashMap<Key, Block>, Val> {
        let mut block_map = HashMap::<Key, Block>::new();
        for (k, v) in Map::from(map) {
            block_map.insert(k, Block::parse(cfg, BRANCH, v)?);
        }
        Ok(block_map)
    }

    fn eval(mut self, cfg: &mut Cfg, ctx: Ctx<Val>) -> Val {
        let Some(body) = self.map.remove(&self.val).or(self.default) else {
            return Val::default();
        };
        body.flow(cfg, BRANCH, ctx).unwrap_or_default()
    }
}

pub fn match_(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
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

    fn eval(self, cfg: &mut Cfg, mut ctx: Ctx<Val>) -> Val {
        for (pattern, block) in self.arms {
            let pattern = Eval.call(cfg, ctx.reborrow(), pattern);
            if cfg.is_aborted() {
                return Val::default();
            }
            let Some(pattern) = pattern.parse(cfg, MATCH) else {
                return Val::default();
            };
            if !pattern.match_(cfg, false, MATCH, &self.val) {
                continue;
            }
            let result = pattern.assign(cfg, MATCH, ctx.reborrow(), self.val);
            if result.is_none() {
                return Val::default();
            }
            return block.flow(cfg, MATCH, ctx).unwrap_or_default();
        }
        Val::default()
    }
}

pub fn loop_(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
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

    fn eval(self, cfg: &mut Cfg, mut ctx: Ctx<Val>) -> Val {
        loop {
            let cond = Eval.call(cfg, ctx.reborrow(), self.condition.clone());
            let Val::Bit(bit) = cond else {
                return bug!(cfg, "{LOOP}: expected condition to be a bit, but got {cond}");
            };
            if !*bit {
                break;
            }
            let Some(output) = self.body.clone().flow(cfg, LOOP, ctx.reborrow()) else {
                return Val::default();
            };
            match output {
                Val::Cell(cell) => return Cell::from(cell).value,
                Val::Unit(_) => {},
                output => {
                    bug!(cfg, "{LOOP}: expected return value of body to be a cell or unit, \
                        but got {output}");
                    return Val::default();
                },
            }
        }
        Val::default()
    }
}

pub fn each(cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
    let Ok(each) = Each::parse(cfg, input) else {
        return Val::default();
    };
    each.eval(cfg, ctx)
}

struct Each {
    val: Val,
    name: Option<Key>,
    body: Block,
}

impl Each {
    fn parse(cfg: &mut Cfg, input: Val) -> Result<Self, Val> {
        let Val::Pair(pair) = input else {
            return Err(bug!(cfg, "{EACH}: expected input to be a pair, but got {input}"));
        };
        let pair = Pair::from(pair);
        let val = pair.left;
        let Val::Pair(name_body) = pair.right else {
            return Err(bug!(cfg, "{EACH}: expected input.right to be a pair, \
                but got {}", pair.right));
        };
        let name_body = Pair::from(name_body);
        let name = match name_body.left {
            Val::Key(key) => Some(key),
            Val::Unit(_) => None,
            v => {
                return Err(
                    bug!(cfg, "{EACH}: expected input.right.left to be a key, but got {v}"),
                );
            },
        };
        let body = Block::parse(cfg, EACH, name_body.right)?;
        Ok(Self { val, name, body })
    }

    fn eval(self, cfg: &mut Cfg, ctx: Ctx<Val>) -> Val {
        match self.val {
            Val::Int(i) => {
                let i = Int::from(i);
                if i.is_negative() {
                    return bug!(cfg, "{EACH}: expected integer to be non-negative, \
                        but got {i}");
                }
                let Some(i) = i.to_u128() else {
                    return bug!(cfg, "{EACH}: unable to each {i}");
                };
                let iter = (0 .. i).map(|i| {
                    let i = Int::from(i);
                    Val::Int(i.into())
                });
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Byte(byte) => {
                let iter = byte.iter().map(|byte| {
                    let byte = Byte::from(vec![*byte]);
                    Val::Byte(byte.into())
                });
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Key(key) => {
                let iter = key.char_indices().map(|(start, c)| {
                    let key = Key::from_str_unchecked(&key[start .. start + c.len_utf8()]);
                    Val::Key(key)
                });
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Text(t) => {
                let iter = t.chars().map(|c| {
                    let text = Text::from(c.to_string());
                    Val::Text(text.into())
                });
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::List(list) => {
                let list = List::from(list);
                let iter = list.into_iter();
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            Val::Map(map) => {
                let map = Map::from(map);
                let iter = map.into_iter().map(|pair| {
                    let pair = Pair::new(Val::Key(pair.0), pair.1);
                    Val::Pair(pair.into())
                });
                each_val(cfg, ctx, self.body, self.name, iter)
            },
            v => bug!(cfg, "{EACH}: expected input.left to be iterable, but got {v}"),
        }
    }
}

fn each_val<ValIter>(
    cfg: &mut Cfg, mut ctx: Ctx<Val>, body: Block, name: Option<Key>, values: ValIter,
) -> Val
where ValIter: Iterator<Item = Val> {
    for val in values {
        if let Some(name) = name.clone() {
            if ctx.const_ {
                return bug!(cfg, "{EACH}: expected name to be a unit in constant context, \
                    but got {name}");
            }
            if ctx.val.set(cfg, name, val).is_none() {
                return Val::default();
            }
        }
        let Some(output) = body.clone().flow(cfg, EACH, ctx.reborrow()) else {
            return Val::default();
        };
        match output {
            Val::Cell(cell) => return Cell::from(cell).value,
            Val::Unit(_) => {},
            output => {
                bug!(cfg, "{EACH}: expected return value of body to be a cell or unit, \
                    but got {output}");
                return Val::default();
            },
        }
    }
    Val::default()
}
