use crate::bug;
use crate::cfg::utils::key;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

pub(in crate::cfg) enum Pattern {
    Any(Key),
    Val(Val),
    Cell(Box<Cell<Pattern>>),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Key, Pattern>),
}

pub(in crate::cfg) trait PatternParse {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern>;
}

impl PatternParse for Val {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        match self {
            Val::Key(key) => key.parse(cfg, tag),
            Val::Cell(cell) => cell.parse(cfg, tag),
            Val::Pair(pair) => pair.parse(cfg, tag),
            Val::List(list) => list.parse(cfg, tag),
            Val::Map(map) => map.parse(cfg, tag),
            Val::Call(call) => call.parse(cfg, tag),
            val => Some(Pattern::Val(val)),
        }
    }
}

const KEY_LITERAL_CHAR: char = '-';
const KEY_REF_CHAR: char = '*';

impl PatternParse for Key {
    fn parse(self, _cfg: &mut Cfg, _tag: &str) -> Option<Pattern> {
        let pattern = match self.chars().next() {
            Some(KEY_LITERAL_CHAR) => Pattern::Val(key(&self[1 ..])),
            Some(KEY_REF_CHAR) => Pattern::Any(Key::from_str_unchecked(&self[1 ..])),
            _ => Pattern::Any(self),
        };
        Some(pattern)
    }
}

impl PatternParse for CellVal {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        let cell = Cell::from(self);
        let value = cell.value.parse(cfg, tag)?;
        Some(Pattern::Cell(Box::new(Cell::new(value))))
    }
}

impl PatternParse for PairVal {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        let pair = Pair::from(self);
        let left = pair.left.parse(cfg, tag)?;
        let right = pair.right.parse(cfg, tag)?;
        Some(Pattern::Pair(Box::new(Pair::new(left, right))))
    }
}

impl PatternParse for CallVal {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        let call = Call::from(self);
        let func = call.func.parse(cfg, tag)?;
        let input = call.input.parse(cfg, tag)?;
        Some(Pattern::Call(Box::new(Call { func, input })))
    }
}

impl PatternParse for ListVal {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        let list = List::from(self);
        let mut patterns = Vec::with_capacity(list.len());
        for val in list {
            patterns.push(val.parse(cfg, tag)?);
        }
        Some(Pattern::List(patterns.into()))
    }
}

impl PatternParse for MapVal {
    fn parse(self, cfg: &mut Cfg, tag: &str) -> Option<Pattern> {
        let map = Map::from(self);
        let mut pattern_map = Map::with_capacity(map.len());
        for (k, val) in map {
            pattern_map.insert(k, val.parse(cfg, tag)?);
        }
        Some(Pattern::Map(pattern_map))
    }
}

pub(in crate::cfg) trait PatternMatch<T> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &T) -> bool;
}

impl PatternMatch<Val> for Pattern {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        match self {
            Pattern::Any(name) => name.match_(cfg, force, tag, val),
            Pattern::Val(expected) => expected.match_(cfg, force, tag, val),
            Pattern::Cell(cell) => cell.match_(cfg, force, tag, val),
            Pattern::Pair(pair) => pair.match_(cfg, force, tag, val),
            Pattern::Call(call) => call.match_(cfg, force, tag, val),
            Pattern::List(list) => list.match_(cfg, force, tag, val),
            Pattern::Map(map) => map.match_(cfg, force, tag, val),
        }
    }
}

impl PatternMatch<Val> for Key {
    fn match_(&self, _cfg: &mut Cfg, _force: bool, _tag: &str, _val: &Val) -> bool {
        true
    }
}

impl PatternMatch<Val> for Val {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let match_ = *self == *val;
        if !match_ && force {
            bug!(cfg, "{tag}: expected {self}, but got {val}");
        }
        match_
    }
}

impl PatternMatch<Val> for Cell<Pattern> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let Val::Cell(val) = val else {
            if force {
                bug!(cfg, "{tag}: expected a cell, but got {val}");
            }
            return false;
        };
        self.value.match_(cfg, force, tag, &val.value)
    }
}

impl PatternMatch<Val> for Pair<Pattern, Pattern> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let Val::Pair(val) = val else {
            if force {
                bug!(cfg, "{tag}: expected a pair, but got {val}");
            }
            return false;
        };
        let left = self.left.match_(cfg, force, tag, &val.left);
        let right = self.right.match_(cfg, force, tag, &val.right);
        left && right
    }
}

impl PatternMatch<Val> for Call<Pattern, Pattern> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let Val::Call(val) = val else {
            if force {
                bug!(cfg, "{tag}: expected a call, but got {val}");
            }
            return false;
        };
        let func = self.func.match_(cfg, force, tag, &val.func);
        let input = self.input.match_(cfg, force, tag, &val.input);
        func && input
    }
}

impl PatternMatch<Val> for List<Pattern> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let Val::List(val) = val else {
            if force {
                bug!(cfg, "{tag}: expected a list, but got {val}");
            }
            return false;
        };
        if val.len() < self.len() {
            if force {
                bug!(cfg, "{tag}: expected length of list to be at least {}, \
                    but got {val}", self.len());
            }
            return false;
        }
        let mut val_iter = val.iter();
        for pattern in self {
            let val = val_iter.next().unwrap();
            if !pattern.match_(cfg, force, tag, val) {
                return false;
            }
        }
        true
    }
}

impl PatternMatch<Val> for Map<Key, Pattern> {
    fn match_(&self, cfg: &mut Cfg, force: bool, tag: &str, val: &Val) -> bool {
        let Val::Map(val) = val else {
            if force {
                bug!(cfg, "{tag}: expected a map, but got {val}");
            }
            return false;
        };
        for (k, pattern) in self {
            let Some(val) = val.get(k) else {
                if force {
                    bug!(cfg, "{tag}: value not found for key {k} in map {val}");
                }
                return false;
            };
            if !pattern.match_(cfg, force, tag, val) {
                return false;
            }
        }
        true
    }
}

pub(in crate::cfg) trait PatternAssign<Ctx, Val> {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Ctx, val: Val) -> Option<()>;
}

impl PatternAssign<Val, Val> for Pattern {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        match self {
            Pattern::Any(name) => name.assign(cfg, tag, ctx, val),
            Pattern::Val(expected) => expected.assign(cfg, tag, ctx, val),
            Pattern::Cell(cell) => cell.assign(cfg, tag, ctx, val),
            Pattern::Pair(pair) => pair.assign(cfg, tag, ctx, val),
            Pattern::Call(call) => call.assign(cfg, tag, ctx, val),
            Pattern::List(list) => list.assign(cfg, tag, ctx, val),
            Pattern::Map(map) => map.assign(cfg, tag, ctx, val),
        }
    }
}

impl PatternAssign<Val, Val> for Key {
    fn assign(self, cfg: &mut Cfg, _tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        ctx.set(cfg, self, val);
        if cfg.is_aborted() {
            return None;
        }
        Some(())
    }
}

impl PatternAssign<Val, Val> for Val {
    fn assign(self, _cfg: &mut Cfg, _tag: &str, _ctx: &mut Val, _val: Val) -> Option<()> {
        Some(())
    }
}

impl PatternAssign<Val, Val> for Cell<Pattern> {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Cell(val) = val else {
            bug!(cfg, "{tag}: expected a cell, but got {val}");
            return None;
        };
        let val = Cell::from(val);
        self.value.assign(cfg, tag, ctx, val.value)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for Pair<Pattern, Pattern> {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Pair(val) = val else {
            bug!(cfg, "{tag}: expected a pair, but got {val}");
            return None;
        };
        let val = Pair::from(val);
        self.left.assign(cfg, tag, ctx, val.left)?;
        self.right.assign(cfg, tag, ctx, val.right)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for Call<Pattern, Pattern> {
    fn assign(self, cfg: &mut Cfg, tag: &str, c: &mut Val, val: Val) -> Option<()> {
        let Val::Call(val) = val else {
            bug!(cfg, "{tag}: expected a call, but got {val}");
            return None;
        };
        let val = Call::from(val);
        self.func.assign(cfg, tag, c, val.func)?;
        self.input.assign(cfg, tag, c, val.input)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for List<Pattern> {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::List(val) = val else {
            bug!(cfg, "{tag}: expected a list, but got {val}");
            return None;
        };
        if val.len() < self.len() {
            bug!(cfg, "{tag}: expected length of list to be at least {}, \
                but got {val}", self.len());
            return None;
        }
        let mut val_iter = List::from(val).into_iter();
        for p in self {
            p.assign(cfg, tag, ctx, val_iter.next().unwrap())?;
        }
        Some(())
    }
}

impl PatternAssign<Val, Val> for Map<Key, Pattern> {
    fn assign(self, cfg: &mut Cfg, tag: &str, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Map(mut val) = val else {
            bug!(cfg, "{tag}: expected a map, but got {val}");
            return None;
        };
        for (k, pattern) in self {
            let Some(val) = val.remove(&k) else {
                bug!(cfg, "{tag}: value not found for key {k} in map {val}");
                return None;
            };
            pattern.assign(cfg, tag, ctx, val)?;
        }
        Some(())
    }
}
