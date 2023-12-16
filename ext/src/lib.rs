#![feature(trait_alias)]

use {
    crate::{
        prelude::{
            AllPrelude,
            Prelude,
        },
        problem::DefaultSolver,
    },
    airlang::{
        CtxForMutableFn,
        Extension,
        Symbol,
        Val,
    },
    std::collections::HashMap,
};
pub use {
    func::{
        ExtCtxConstFn,
        ExtCtxFreeFn,
        ExtCtxMutableFn,
        ExtFn,
        ExtFunc,
    },
    problem::Solver,
};

pub struct AirExt {
    pub(crate) func_map: HashMap<Symbol, ExtFunc>,
    pub(crate) solver: Box<dyn Solver>,
}

impl Extension for AirExt {
    fn call(&mut self, mut ctx: CtxForMutableFn, func: Val, input: Val) -> Val {
        let Val::Symbol(func) = func else {
            return Val::default();
        };
        let Some(func) = self.func_map.get_mut(&func) else {
            return Val::default();
        };
        let input = func.input_mode.apply(ctx.reborrow(), input);
        func.ext_fn.call(ctx, input)
    }

    fn reverse(&mut self, mut ctx: CtxForMutableFn, func: Val, output: Val) -> Val {
        let Val::Symbol(func) = func else {
            return Val::default();
        };
        let Some(func) = self.func_map.get(&func) else {
            return Val::default();
        };
        let output = func.output_mode.apply(ctx.reborrow(), output);
        self.solver.solve(ctx, func, output)
    }
}

impl Default for AirExt {
    fn default() -> Self {
        let all_prelude = AllPrelude::default();
        let mut call_map = HashMap::with_capacity(200);
        all_prelude.put(&mut call_map);
        Self {
            func_map: call_map,
            solver: Box::new(DefaultSolver),
        }
    }
}

impl AirExt {
    pub fn add_func(&mut self, name: Symbol, func: ExtFunc) {
        self.func_map.entry(name).or_insert(func);
    }
}

pub(crate) mod func;

pub(crate) mod problem;

pub(crate) mod prelude;

#[cfg(test)]
mod test;
