use const_format::concatcp;

use crate::{
    Symbol,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    transformer::Transformer,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Literal,
    Ref,
    Move,
}

pub(crate) const LITERAL_CHAR: char = '.';
pub(crate) const LITERAL: &str = concatcp!(LITERAL_CHAR);
pub(crate) const REF_CHAR: char = '*';
pub(crate) const REF: &str = concatcp!(REF_CHAR);
pub(crate) const MOVE_CHAR: char = '^';
pub(crate) const MOVE: &str = concatcp!(MOVE_CHAR);

impl Transformer<Symbol, Val> for SymbolMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            SymbolMode::Literal => FormCore::transform_symbol::<LITERAL_CHAR, _>(ctx, symbol),
            SymbolMode::Ref => FormCore::transform_symbol::<REF_CHAR, _>(ctx, symbol),
            SymbolMode::Move => FormCore::transform_symbol::<MOVE_CHAR, _>(ctx, symbol),
        }
    }
}

impl From<UniMode> for SymbolMode {
    fn from(mode: UniMode) -> Self {
        mode.symbol
    }
}
