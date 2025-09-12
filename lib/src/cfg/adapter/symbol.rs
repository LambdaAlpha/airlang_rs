use crate::semantics::cfg::Cfg;
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
pub enum SymbolAdapter {
    Id,
    Literal,
    Ref,
    Eval,
}

impl SymbolAdapter {
    fn try_into_char(self) -> Option<char> {
        let c = match self {
            SymbolAdapter::Id => return None,
            SymbolAdapter::Literal => SYMBOL_LITERAL_CHAR,
            SymbolAdapter::Ref => SYMBOL_REF_CHAR,
            SymbolAdapter::Eval => SYMBOL_EVAL_CHAR,
        };
        Some(c)
    }
}

impl FreeFn<Cfg, Symbol, Val> for SymbolAdapter {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for SymbolAdapter {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for SymbolAdapter {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        let Some(default) = self.try_into_char() else { return Val::Symbol(input) };
        SymbolEval { default, f: &Eval }.mut_call(cfg, ctx, input)
    }
}
