use log::error;

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Key;

pub(crate) struct KeyEval;

pub(crate) const PREFIX_ID: char = '_';
pub(crate) const PREFIX_SHIFT: char = '.';
pub(crate) const PREFIX_CTX: char = ':';

enum KeyMode {
    Id,
    Shift,
    Ctx,
}

impl KeyEval {
    fn recognize(&self, input: Key) -> (KeyMode, Key) {
        match input.chars().next() {
            Some(PREFIX_ID) => (KeyMode::Id, input),
            Some(PREFIX_SHIFT) => (KeyMode::Shift, Key::from_str_unchecked(&input[1 ..])),
            Some(PREFIX_CTX) => (KeyMode::Ctx, Key::from_str_unchecked(&input[1 ..])),
            _ => (KeyMode::Ctx, input),
        }
    }
}

impl FreeFn<Cfg, Key, Val> for KeyEval {
    fn free_call(&self, cfg: &mut Cfg, input: Key) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input.clone());
        match mode {
            KeyMode::Id => Val::Key(s),
            KeyMode::Shift => Val::Key(s),
            KeyMode::Ctx => {
                error!("key {input:?} should be evaluated in a ctx");
                Val::default()
            }
        }
    }
}

impl ConstFn<Cfg, Val, Key, Val> for KeyEval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Key) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input);
        match mode {
            KeyMode::Id => Val::Key(s),
            KeyMode::Shift => Val::Key(s),
            KeyMode::Ctx => {
                let Some(val) = ctx.unwrap().ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
        }
    }
}

impl MutFn<Cfg, Val, Key, Val> for KeyEval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Key) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input);
        match mode {
            KeyMode::Id => Val::Key(s),
            KeyMode::Shift => Val::Key(s),
            KeyMode::Ctx => {
                let Some(val) = ctx.ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
        }
    }
}
