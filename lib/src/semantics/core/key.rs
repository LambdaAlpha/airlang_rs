use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::Val;
use crate::type_::Key;

pub(crate) struct KeyEval;

pub const PREFIX_QUOTE: char = '_';
pub const PREFIX_CELL: char = '.';

enum KeyMode {
    Quote,
    Cell,
    Ctx,
}

impl KeyEval {
    fn recognize(&self, key: Key) -> (KeyMode, Key) {
        match key.chars().next() {
            Some(PREFIX_QUOTE) => (KeyMode::Quote, Key::from_str_unchecked(&key[1 ..])),
            Some(PREFIX_CELL) => (KeyMode::Cell, key),
            _ => (KeyMode::Ctx, key),
        }
    }
}

impl DynFunc<Val, Key, Val> for KeyEval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, key: Key) -> Val {
        let (mode, key) = self.recognize(key);
        if matches!(mode, KeyMode::Quote | KeyMode::Cell) {
            return Val::Key(key);
        }
        let Some(val) = ctx.val.ref_(cfg, key) else {
            return Val::default();
        };
        val.clone()
    }
}
