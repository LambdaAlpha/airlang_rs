use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::CtxFn;
use crate::semantics::func::FreeFn;
use crate::semantics::val::Val;
use crate::type_::Key;

pub(crate) struct KeyEval;

pub const PREFIX_ID: char = '_';
pub const PREFIX_SHIFT: char = '.';
pub const PREFIX_CTX: char = ':';

enum KeyMode {
    Id,
    Shift,
    Ctx,
}

impl KeyEval {
    fn recognize(&self, key: Key) -> (KeyMode, Key) {
        match key.chars().next() {
            Some(PREFIX_ID) => (KeyMode::Id, key),
            Some(PREFIX_SHIFT) => (KeyMode::Shift, Key::from_str_unchecked(&key[1 ..])),
            Some(PREFIX_CTX) => (KeyMode::Ctx, Key::from_str_unchecked(&key[1 ..])),
            _ => (KeyMode::Ctx, key),
        }
    }
}

impl FreeFn<Cfg, Key, Val> for KeyEval {
    fn free_call(&self, cfg: &mut Cfg, key: Key) -> Val {
        let (mode, key) = self.recognize(key);
        match mode {
            KeyMode::Id => return Val::Key(key),
            KeyMode::Shift => return Val::Key(key),
            KeyMode::Ctx => {}
        }
        let msg = format!("eval: no context for key {key}");
        abort_by_bug_with_msg(cfg, msg.into())
    }
}

impl CtxFn<Cfg, Val, Key, Val> for KeyEval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        let (mode, key) = self.recognize(key);
        match mode {
            KeyMode::Id => return Val::Key(key),
            KeyMode::Shift => return Val::Key(key),
            KeyMode::Ctx => {}
        }
        let Some(val) = ctx.ref_(cfg, key) else {
            return Val::default();
        };
        val.clone()
    }
}
