use crate::semantics::core::Eval;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::core::SymbolEval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
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

impl SymbolMode {
    fn try_into_char(self) -> Option<char> {
        let c = match self {
            SymbolMode::Id => return None,
            SymbolMode::Literal => SYMBOL_LITERAL_CHAR,
            SymbolMode::Ref => SYMBOL_REF_CHAR,
            SymbolMode::Eval => SYMBOL_EVAL_CHAR,
        };
        Some(c)
    }
}

impl FreeFn<Symbol, Val> for SymbolMode {
    fn free_call(&self, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.free_call(input)
    }
}

impl ConstFn<Val, Symbol, Val> for SymbolMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.const_call(ctx, input)
    }
}

impl MutFn<Val, Symbol, Val> for SymbolMode {
    fn mut_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.mut_call(ctx, input)
    }
}
