use crate::{
    PrimitiveMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    mode::{
        eval::Eval,
        form::Form,
        id::Id,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Id,
    Form,
    #[default]
    Eval,
}

impl Transformer<Symbol, Val> for SymbolMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            SymbolMode::Id => Id.transform_symbol(ctx, symbol),
            SymbolMode::Form => Form.transform_symbol(ctx, symbol),
            SymbolMode::Eval => Eval.transform_symbol(ctx, symbol),
        }
    }
}

impl From<PrimitiveMode> for SymbolMode {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => SymbolMode::Id,
            PrimitiveMode::Form => SymbolMode::Form,
            PrimitiveMode::Eval => SymbolMode::Eval,
        }
    }
}

impl From<SymbolMode> for PrimitiveMode {
    fn from(mode: SymbolMode) -> Self {
        match mode {
            SymbolMode::Id => PrimitiveMode::Id,
            SymbolMode::Form => PrimitiveMode::Form,
            SymbolMode::Eval => PrimitiveMode::Eval,
        }
    }
}
