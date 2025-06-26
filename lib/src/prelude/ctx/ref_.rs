use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::val::Val;
use crate::type_::Either;
use crate::type_::Symbol;

pub(in crate::prelude) struct RefCtx;

impl RefCtx {
    pub(in crate::prelude) fn escape_symbol(val: Val) -> Val {
        if let Val::Symbol(s) = val {
            Val::Symbol(Symbol::from_string_unchecked(format!("{}{}", SYMBOL_LITERAL_CHAR, &*s)))
        } else {
            val
        }
    }

    // todo design
    pub(in crate::prelude) fn ref_or_val(val: Val) -> Either<Symbol, Val> {
        let Val::Symbol(s) = val else {
            return Either::That(val);
        };
        let prefix = s.chars().next();
        match prefix {
            Some(SYMBOL_LITERAL_CHAR) => {
                Either::That(Val::Symbol(Symbol::from_str_unchecked(&s[1 ..])))
            }
            Some(SYMBOL_REF_CHAR) => Either::This(Symbol::from_str_unchecked(&s[1 ..])),
            _ => Either::This(s),
        }
    }
}
