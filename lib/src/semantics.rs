use {
    crate::{
        semantics::{
            ctx::Ctx,
            ctx_access::mutable::MutableCtx,
            eval::Evaluator,
            eval_mode::eval::Eval,
            prelude::initial_ctx,
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
    ctx: Ctx,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { ctx: initial_ctx() }
    }

    pub fn interpret(&mut self, src: Val) -> Val {
        Eval.eval(&mut MutableCtx(&mut self.ctx), src)
    }

    pub fn reset(&mut self) {
        self.ctx = initial_ctx();
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
