use crate::{
    PrimitiveMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    mode::{
        form::Form,
        id::Id,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Id,
    Form(Form),
}

impl Transformer<Symbol, Val> for SymbolMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            SymbolMode::Id => Id.transform_symbol(ctx, symbol),
            SymbolMode::Form(mode) => mode.transform(ctx, symbol),
        }
    }
}

impl From<PrimitiveMode> for SymbolMode {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => SymbolMode::Id,
            PrimitiveMode::Form(mode) => SymbolMode::Form(mode),
            PrimitiveMode::Eval(mode) => SymbolMode::Form(Form::from(mode)),
        }
    }
}

impl From<SymbolMode> for PrimitiveMode {
    fn from(mode: SymbolMode) -> Self {
        match mode {
            SymbolMode::Id => PrimitiveMode::Id,
            SymbolMode::Form(mode) => PrimitiveMode::Form(mode),
        }
    }
}

impl Default for SymbolMode {
    fn default() -> Self {
        SymbolMode::Form(Form::default())
    }
}
