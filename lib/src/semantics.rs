use {
    crate::{
        repr::Repr,
        semantics::{
            eval::{
                Ctx,
                NameMap,
            },
            prelude::prelude,
            val::Val,
        },
        traits::TryClone,
        types::ImRef,
    },
    thiserror::Error,
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
    prelude: ImRef<NameMap>,
    ctx: Ctx,
}

impl Interpreter {
    pub fn new() -> Self {
        let prelude = prelude();
        let ctx = Self::default_ctx(&prelude);
        Interpreter { prelude, ctx }
    }

    pub fn interpret(&mut self, src: &Repr) -> Result<Repr, ReprError> {
        let input = Val::from(src);
        let output = self.ctx.eval(&input);
        output.try_into()
    }

    pub fn reset(&mut self) {
        self.ctx = Self::default_ctx(&self.prelude);
    }

    fn default_ctx(prelude: &ImRef<NameMap>) -> Ctx {
        let constants = prelude.try_clone().unwrap_or_default();
        Ctx {
            constants,
            variables: Default::default(),
            call_interpreter: Default::default(),
            reverse_interpreter: None,
        }
    }
}
