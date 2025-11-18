use log::error;

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

pub(crate) struct SymbolEval;

pub(crate) const PREFIX_ID: char = '_';
pub(crate) const PREFIX_SHIFT: char = '.';
pub(crate) const PREFIX_CTX: char = ':';

enum SymbolMode {
    Id,
    Shift,
    Ctx,
}

impl SymbolEval {
    fn recognize(&self, input: Symbol) -> (SymbolMode, Symbol) {
        match input.chars().next() {
            Some(PREFIX_ID) => (SymbolMode::Id, input),
            Some(PREFIX_SHIFT) => (SymbolMode::Shift, Symbol::from_str_unchecked(&input[1 ..])),
            Some(PREFIX_CTX) => (SymbolMode::Ctx, Symbol::from_str_unchecked(&input[1 ..])),
            _ => (SymbolMode::Ctx, input),
        }
    }
}

impl FreeFn<Cfg, Symbol, Val> for SymbolEval {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input.clone());
        match mode {
            SymbolMode::Id => Val::Symbol(s),
            SymbolMode::Shift => Val::Symbol(s),
            SymbolMode::Ctx => {
                error!("symbol {input:?} should be evaluated in a ctx");
                Val::default()
            }
        }
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for SymbolEval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input);
        match mode {
            SymbolMode::Id => Val::Symbol(s),
            SymbolMode::Shift => Val::Symbol(s),
            SymbolMode::Ctx => {
                let Some(val) = ctx.unwrap().ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
        }
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for SymbolEval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        cfg.step();
        let (mode, s) = self.recognize(input);
        match mode {
            SymbolMode::Id => Val::Symbol(s),
            SymbolMode::Shift => Val::Symbol(s),
            SymbolMode::Ctx => {
                let Some(val) = ctx.ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
        }
    }
}
