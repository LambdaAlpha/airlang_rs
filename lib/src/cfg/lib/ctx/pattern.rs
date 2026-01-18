use log::error;

use crate::cfg::utils::key;
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
use crate::type_::Unit;

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
    fn parse(self) -> Option<Pattern>;
}

impl PatternParse for Val {
    fn parse(self) -> Option<Pattern> {
        match self {
            Val::Key(key) => key.parse(),
            Val::Cell(cell) => cell.parse(),
            Val::Pair(pair) => pair.parse(),
            Val::List(list) => list.parse(),
            Val::Map(map) => map.parse(),
            Val::Call(call) => call.parse(),
            val => Some(Pattern::Val(val)),
        }
    }
}

const KEY_LITERAL_CHAR: char = '-';
const KEY_REF_CHAR: char = '*';

impl PatternParse for Key {
    fn parse(self) -> Option<Pattern> {
        let pattern = match self.chars().next() {
            Some(KEY_LITERAL_CHAR) => Pattern::Val(key(&self[1 ..])),
            Some(KEY_REF_CHAR) => Pattern::Any(Key::from_str_unchecked(&self[1 ..])),
            _ => Pattern::Any(self),
        };
        Some(pattern)
    }
}

impl PatternParse for CellVal {
    fn parse(self) -> Option<Pattern> {
        let cell = Cell::from(self);
        let value = cell.value.parse()?;
        Some(Pattern::Cell(Box::new(Cell::new(value))))
    }
}

impl PatternParse for PairVal {
    fn parse(self) -> Option<Pattern> {
        let pair = Pair::from(self);
        let left = pair.left.parse()?;
        let right = pair.right.parse()?;
        Some(Pattern::Pair(Box::new(Pair::new(left, right))))
    }
}

impl PatternParse for CallVal {
    fn parse(self) -> Option<Pattern> {
        let call = Call::from(self);
        let func = call.func.parse()?;
        let input = call.input.parse()?;
        Some(Pattern::Call(Box::new(Call { func, input })))
    }
}

impl PatternParse for ListVal {
    fn parse(self) -> Option<Pattern> {
        let list = List::from(self);
        let list = list.into_iter().map(PatternParse::parse).collect::<Option<List<_>>>()?;
        Some(Pattern::List(list))
    }
}

impl PatternParse for MapVal {
    fn parse(self) -> Option<Pattern> {
        let map = Map::from(self);
        let map =
            map.into_iter().map(|(k, v)| Some((k, v.parse()?))).collect::<Option<Map<_, _>>>()?;
        Some(Pattern::Map(map))
    }
}

pub(in crate::cfg) trait PatternMatch<T> {
    fn match_(&self, val: &T) -> bool;
}

impl PatternMatch<Val> for Pattern {
    fn match_(&self, val: &Val) -> bool {
        match self {
            Pattern::Any(name) => name.match_(val),
            Pattern::Val(expected) => expected.match_(val),
            Pattern::Cell(cell) => cell.match_(val),
            Pattern::Pair(pair) => pair.match_(val),
            Pattern::Call(call) => call.match_(val),
            Pattern::List(list) => list.match_(val),
            Pattern::Map(map) => map.match_(val),
        }
    }
}

impl PatternMatch<Val> for Key {
    fn match_(&self, _val: &Val) -> bool {
        true
    }
}

impl PatternMatch<Val> for Val {
    fn match_(&self, val: &Val) -> bool {
        *self == *val
    }
}

impl PatternMatch<Val> for Cell<Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Cell(val) = val else {
            error!("{val:?} should be a cell");
            return false;
        };
        self.value.match_(&val.value)
    }
}

impl PatternMatch<Val> for Pair<Pattern, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Pair(val) = val else {
            error!("{val:?} should be a pair");
            return false;
        };
        let left = self.left.match_(&val.left);
        let right = self.right.match_(&val.right);
        left && right
    }
}

impl PatternMatch<Val> for Call<Pattern, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Call(val) = val else {
            error!("{val:?} should be a call");
            return false;
        };
        let func = self.func.match_(&val.func);
        let input = self.input.match_(&val.input);
        func && input
    }
}

impl PatternMatch<Val> for List<Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::List(val) = val else {
            error!("{val:?} should be a list");
            return false;
        };
        let mut val_iter = val.iter();
        self.iter().all(|p| {
            let val = val_iter.next().unwrap_or(&Val::Unit(Unit));
            p.match_(val)
        })
    }
}

impl PatternMatch<Val> for Map<Key, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Map(val) = val else {
            error!("{val:?} should be a map");
            return false;
        };
        self.iter().all(|(k, pattern)| {
            let val = val.get(k).unwrap_or(&Val::Unit(Unit));
            pattern.match_(val)
        })
    }
}

pub(in crate::cfg) trait PatternAssign<Ctx, Val> {
    fn assign(self, ctx: &mut Ctx, val: Val) -> Option<()>;
}

impl PatternAssign<Val, Val> for Pattern {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        match self {
            Pattern::Any(name) => name.assign(ctx, val),
            Pattern::Val(expected) => expected.assign(ctx, val),
            Pattern::Cell(cell) => cell.assign(ctx, val),
            Pattern::Pair(pair) => pair.assign(ctx, val),
            Pattern::Call(call) => call.assign(ctx, val),
            Pattern::List(list) => list.assign(ctx, val),
            Pattern::Map(map) => map.assign(ctx, val),
        }
    }
}

impl PatternAssign<Val, Val> for Key {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        ctx.set(self, val)
    }
}

impl PatternAssign<Val, Val> for Val {
    fn assign(self, _ctx: &mut Val, _val: Val) -> Option<()> {
        Some(())
    }
}

impl PatternAssign<Val, Val> for Cell<Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Cell(val) = val else {
            error!("{val:?} should be a cell");
            return None;
        };
        let val = Cell::from(val);
        self.value.assign(ctx, val.value)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for Pair<Pattern, Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Pair(val) = val else {
            error!("{val:?} should be a pair");
            return None;
        };
        let val = Pair::from(val);
        self.left.assign(ctx, val.left)?;
        self.right.assign(ctx, val.right)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for Call<Pattern, Pattern> {
    fn assign(self, c: &mut Val, val: Val) -> Option<()> {
        let Val::Call(val) = val else {
            error!("{val:?} should be a call");
            return None;
        };
        let val = Call::from(val);
        self.func.assign(c, val.func)?;
        self.input.assign(c, val.input)?;
        Some(())
    }
}

impl PatternAssign<Val, Val> for List<Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::List(val) = val else {
            error!("{val:?} should be a list");
            return None;
        };
        let mut val_iter = List::from(val).into_iter();
        for p in self {
            p.assign(ctx, val_iter.next()?)?;
        }
        Some(())
    }
}

impl PatternAssign<Val, Val> for Map<Key, Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Option<()> {
        let Val::Map(mut val) = val else {
            error!("{val:?} should be a map");
            return None;
        };
        for (k, pattern) in self {
            pattern.assign(ctx, val.remove(&k)?)?;
        }
        Some(())
    }
}
