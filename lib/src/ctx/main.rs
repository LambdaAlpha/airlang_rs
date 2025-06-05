use crate::Ctx;
use crate::Symbol;
use crate::Val;
use crate::either::Either;
use crate::mode::symbol::LITERAL_CHAR;
use crate::mode::symbol::REF_CHAR;

pub(crate) struct MainCtx;

impl MainCtx {
    pub(crate) fn get_or_default(ctx: &Ctx, name: Symbol) -> Val {
        ctx.variables().get_ref(name).cloned().unwrap_or_default()
    }

    pub(crate) fn remove_or_default(ctx: &mut Ctx, name: Symbol) -> Val {
        ctx.variables_mut().remove(name).unwrap_or_default()
    }

    pub(crate) fn escape_symbol(val: Val) -> Val {
        if let Val::Symbol(s) = val {
            Val::Symbol(Symbol::from_string(format!("{}{}", LITERAL_CHAR, &*s)))
        } else {
            val
        }
    }

    // todo design
    pub(crate) fn ref_or_val(val: Val) -> Either<Symbol, Val> {
        let Val::Symbol(s) = val else {
            return Either::That(val);
        };
        let prefix = s.chars().next();
        match prefix {
            Some(LITERAL_CHAR) => Either::That(Val::Symbol(Symbol::from_str(&s[1 ..]))),
            Some(REF_CHAR) => Either::This(Symbol::from_str(&s[1 ..])),
            _ => Either::This(s),
        }
    }
}
