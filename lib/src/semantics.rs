use {
    crate::{
        semantics::{
            ctx::{
                Ctx,
                NameMap,
            },
            ctx_access::mutable::MutableCtx,
            eval::Evaluator,
            eval_mode::eval::Eval,
            prelude::prelude,
        },
        syntax::ParseError,
    },
    thiserror::Error,
};
pub use {
    func::Func,
    val::Val,
};

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}

pub struct Interpreter {
    prelude: NameMap,
    ctx: Ctx,
}

impl Interpreter {
    pub fn new() -> Self {
        let prelude = prelude();
        let ctx = Self::default_ctx(&prelude);
        Interpreter { prelude, ctx }
    }

    pub fn interpret(&mut self, src: Val) -> Val {
        Eval.eval(&mut MutableCtx(&mut self.ctx), src)
    }

    pub fn reset(&mut self) {
        self.ctx = Self::default_ctx(&self.prelude);
    }

    fn default_ctx(prelude: &NameMap) -> Ctx {
        let name_map = prelude.clone();
        Ctx {
            name_map,
            super_ctx: None,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse(src: &str) -> Result<Val, ParseError> {
    crate::syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    crate::syntax::generator::generate_pretty(src)
}

pub(crate) mod val;

pub(crate) mod eval;

pub(crate) mod ctx;

pub(crate) mod ctx_access;

pub(crate) mod func;

pub(crate) mod eval_mode;

pub(crate) mod input_mode;

pub(crate) mod logic;

pub(crate) mod prelude;

#[cfg(test)]
mod test;
