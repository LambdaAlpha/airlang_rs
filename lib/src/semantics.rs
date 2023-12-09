use {
    crate::{
        semantics::{
            ctx::Ctx,
            ctx_access::mutable::MutableCtx,
            eval::Evaluator,
            eval_mode::more::More,
            prelude::initial_ctx,
        },
        syntax::ParseError,
    },
    thiserror::Error,
};

pub use self::val::{
    CallVal,
    CtxVal,
    FuncVal,
    ListVal,
    MapVal,
    PairVal,
    PropVal,
    ReverseVal,
    Val,
};

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}

#[derive(Debug)]
pub struct Interpreter {
    ctx: Ctx,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { ctx: initial_ctx() }
    }

    pub fn interpret(&mut self, src: Val) -> Val {
        More.eval(&mut MutableCtx(&mut self.ctx), src)
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

pub(crate) mod io_mode;

pub(crate) mod logic;

pub(crate) mod nondeterministic;

pub(crate) mod problem;

pub(crate) mod prelude;

#[cfg(test)]
mod test;
