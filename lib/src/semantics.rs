use {
    crate::{
        semantics::{
            eval::NameMap,
            prelude::prelude,
        },
        syntax::ParseError,
        types::Reader,
    },
    thiserror::Error,
};
pub use {
    eval::{
        Ctx,
        Func,
    },
    val::Val,
};

pub(crate) mod val;

pub(crate) mod eval;

pub(crate) mod prelude;

#[cfg(test)]
mod test;

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
        self.ctx.eval(src)
    }

    pub fn reset(&mut self) {
        self.ctx = Self::default_ctx(&self.prelude);
    }

    fn default_ctx(prelude: &NameMap) -> Ctx {
        let constants = prelude.clone();
        Ctx {
            constants: Reader::new(constants),
            variables: Default::default(),
            reverse_interpreter: None,
        }
    }
}

pub fn parse(src: &str) -> Result<Val, ParseError> {
    crate::syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    crate::syntax::generator::generate_pretty(src)
}
