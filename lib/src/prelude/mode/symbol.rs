use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::core::SymbolEval;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SymbolMode {
    Id,
    Literal,
    Ref,
    Eval,
}

impl FreeStaticFn<Symbol, Val> for SymbolMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        let default = match self {
            SymbolMode::Id => return Val::Symbol(input),
            SymbolMode::Literal => SYMBOL_LITERAL_CHAR,
            SymbolMode::Ref => SYMBOL_REF_CHAR,
            SymbolMode::Eval => SYMBOL_EVAL_CHAR,
        };
        SymbolEval { default, f: &Eval }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for SymbolMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        let default = match self {
            SymbolMode::Id => return Val::Symbol(input),
            SymbolMode::Literal => SYMBOL_LITERAL_CHAR,
            SymbolMode::Ref => SYMBOL_REF_CHAR,
            SymbolMode::Eval => SYMBOL_EVAL_CHAR,
        };
        SymbolEval { default, f: &Eval }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for SymbolMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        let default = match self {
            SymbolMode::Id => return Val::Symbol(input),
            SymbolMode::Literal => SYMBOL_LITERAL_CHAR,
            SymbolMode::Ref => SYMBOL_REF_CHAR,
            SymbolMode::Eval => SYMBOL_EVAL_CHAR,
        };
        SymbolEval { default, f: &Eval }.mut_static_call(ctx, input)
    }
}
