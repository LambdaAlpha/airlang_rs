use std::ops::Deref;

use super::func::ConstFn;
use super::func::FreeFn;
use super::func::MutFn;
use super::val::FuncVal;
use super::val::Val;
use crate::cfg::CoreCfg;
use crate::semantics::cfg::Cfg;
use crate::type_::ConstRef;
use crate::type_::Pair;
use crate::type_::Symbol;

pub(super) struct Solve {
    pub(super) func: FuncVal,
}

impl FreeFn<Cfg, Val, Val> for Solve {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        if let Some(reverse) = cfg.import(self.reverse_name()) {
            let Val::Func(reverse) = reverse else {
                return Val::default();
            };
            return reverse.free_call(cfg, input);
        }
        if let Some(solver) = cfg.import(Self::solver_name()) {
            let Val::Func(solver) = solver else {
                return Val::default();
            };
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            return solver.free_call(cfg, func_input);
        }
        Val::default()
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Solve {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        if let Some(reverse) = cfg.import(self.reverse_name()) {
            let Val::Func(reverse) = reverse else {
                return Val::default();
            };
            return reverse.const_call(cfg, ctx, input);
        }
        if let Some(solver) = cfg.import(Self::solver_name()) {
            let Val::Func(solver) = solver else {
                return Val::default();
            };
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            return solver.const_call(cfg, ctx, func_input);
        }
        Val::default()
    }
}

impl MutFn<Cfg, Val, Val, Val> for Solve {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        if let Some(reverse) = cfg.import(self.reverse_name()) {
            let Val::Func(reverse) = reverse else {
                return Val::default();
            };
            return reverse.mut_call(cfg, ctx, input);
        }
        if let Some(solver) = cfg.import(Self::solver_name()) {
            let Val::Func(solver) = solver else {
                return Val::default();
            };
            let func_input = Val::Pair(Pair::new(Val::Func(self.func.clone()), input).into());
            return solver.mut_call(cfg, ctx, func_input);
        }
        Val::default()
    }
}

impl Solve {
    fn reverse_name(&self) -> Symbol {
        let reverse_name = format!("{}.{}", CoreCfg::REVERSE, self.func.id().deref());
        Symbol::from_string_unchecked(reverse_name)
    }

    fn solver_name() -> Symbol {
        Symbol::from_str_unchecked(CoreCfg::SOLVER)
    }
}
