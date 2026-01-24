use log::error;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_bug_with_msg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
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
        let (mode, s) = self.recognize(key.clone());
        match mode {
            KeyMode::Id => Val::Key(s),
            KeyMode::Shift => Val::Key(s),
            KeyMode::Ctx => {
                error!("key {key:?} should be evaluated in a ctx");
                abort_bug_with_msg(cfg, "key should exist in a context")
            }
        }
    }
}

impl ConstFn<Cfg, Val, Key, Val> for KeyEval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, key: Key) -> Val {
        let (mode, s) = self.recognize(key);
        match mode {
            KeyMode::Id => Val::Key(s),
            KeyMode::Shift => Val::Key(s),
            KeyMode::Ctx => {
                let Some(val) = ctx.unwrap().ref_(s) else {
                    return abort_bug_with_msg(cfg, "key should exist in the context");
                };
                val.clone()
            }
        }
    }
}

impl MutFn<Cfg, Val, Key, Val> for KeyEval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        self.const_call(cfg, ConstRef::new(ctx), key)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, key: Key) -> Val {
        self.const_call(cfg, ctx.into_const(), key)
    }
}
