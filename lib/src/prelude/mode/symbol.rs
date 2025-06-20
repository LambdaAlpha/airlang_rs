use super::ModeFn;
use crate::semantics::core::LITERAL_CHAR;
use crate::semantics::core::MOVE_CHAR;
use crate::semantics::core::REF_CHAR;
use crate::semantics::core::SymbolForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Literal,
    Ref,
    Move,
}

impl ModeFn for SymbolMode {}

impl FreeStaticFn<Symbol, Val> for SymbolMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        match self {
            SymbolMode::Literal => SymbolForm::<LITERAL_CHAR>.free_static_call(input),
            SymbolMode::Ref => SymbolForm::<REF_CHAR>.free_static_call(input),
            SymbolMode::Move => SymbolForm::<MOVE_CHAR>.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, Symbol, Val> for SymbolMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        match self {
            SymbolMode::Literal => SymbolForm::<LITERAL_CHAR>.const_static_call(ctx, input),
            SymbolMode::Ref => SymbolForm::<REF_CHAR>.const_static_call(ctx, input),
            SymbolMode::Move => SymbolForm::<MOVE_CHAR>.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Symbol, Val> for SymbolMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        match self {
            SymbolMode::Literal => SymbolForm::<LITERAL_CHAR>.mut_static_call(ctx, input),
            SymbolMode::Ref => SymbolForm::<REF_CHAR>.mut_static_call(ctx, input),
            SymbolMode::Move => SymbolForm::<MOVE_CHAR>.mut_static_call(ctx, input),
        }
    }
}
