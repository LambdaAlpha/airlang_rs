use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_MOVE_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::core::SymbolEval;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::SetupFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Literal,
    Ref,
    Move,
    Eval,
}

impl SetupFn for SymbolMode {}

impl SymbolMode {
    pub(crate) fn into_char(self) -> char {
        match self {
            SymbolMode::Literal => SYMBOL_LITERAL_CHAR,
            SymbolMode::Ref => SYMBOL_REF_CHAR,
            SymbolMode::Move => SYMBOL_MOVE_CHAR,
            SymbolMode::Eval => SYMBOL_EVAL_CHAR,
        }
    }
}

impl FreeStaticFn<Symbol, Val> for SymbolMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        SymbolEval { default: self.into_char(), f: &Eval }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for SymbolMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolEval { default: self.into_char(), f: &Eval }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for SymbolMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        SymbolEval { default: self.into_char(), f: &Eval }.mut_static_call(ctx, input)
    }
}
