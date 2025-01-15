use const_format::concatcp;

use crate::{
    Symbol,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Id(Id),
    Form(PrefixMode),
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrefixMode {
    Literal,
    #[default]
    Ref,
    Move,
}

pub(crate) const LITERAL: char = '.';
pub(crate) const LITERAL_STR: &str = concatcp!(LITERAL);
pub(crate) const REF: char = '*';
pub(crate) const REF_STR: &str = concatcp!(REF);
pub(crate) const MOVE: char = '^';
pub(crate) const MOVE_STR: &str = concatcp!(MOVE);

impl Transformer<Symbol, Val> for PrefixMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrefixMode::Literal => FormCore::transform_symbol::<LITERAL, _>(ctx, symbol),
            PrefixMode::Ref => FormCore::transform_symbol::<REF, _>(ctx, symbol),
            PrefixMode::Move => FormCore::transform_symbol::<MOVE, _>(ctx, symbol),
        }
    }
}

impl Transformer<Symbol, Val> for SymbolMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            SymbolMode::Id(mode) => mode.transform_symbol(ctx, symbol),
            SymbolMode::Form(mode) => mode.transform(ctx, symbol),
        }
    }
}

impl From<UniMode> for SymbolMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => SymbolMode::Id(mode),
            UniMode::Form(mode) => SymbolMode::Form(mode.prefix_mode()),
            UniMode::Eval(mode) => SymbolMode::Form(mode.prefix_mode()),
        }
    }
}

impl Default for SymbolMode {
    fn default() -> Self {
        SymbolMode::Form(PrefixMode::default())
    }
}
