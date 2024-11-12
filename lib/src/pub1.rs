use crate::{
    Ctx,
    Mode,
    MutCtx,
    ReprError,
    Val,
    prelude,
    syntax,
    syntax::ParseError,
    transformer::Transformer,
};

pub fn parse(src: &str) -> Result<Val, ParseError> {
    syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    syntax::generator::generate_pretty(src)
}

#[derive(Debug, Clone)]
pub struct AirCell {
    mode: Mode,
    ctx: Ctx,
}

impl AirCell {
    pub fn new(mode: Mode, ctx: Ctx) -> Self {
        Self { mode, ctx }
    }

    pub fn initial_ctx() -> Ctx {
        prelude::initial_ctx()
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        self.mode.transform(MutCtx::new(&mut self.ctx), input)
    }

    pub fn ctx_mut(&mut self) -> MutCtx {
        MutCtx::new(&mut self.ctx)
    }
}

impl Default for AirCell {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            ctx: Self::initial_ctx(),
        }
    }
}
