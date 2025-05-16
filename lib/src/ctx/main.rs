use crate::Ctx;
use crate::CtxError;
use crate::DynRef;
use crate::Pair;
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

    pub(crate) fn is_null(ctx: &Ctx, name: Symbol) -> Result<bool, CtxError> {
        match ctx.variables().get_ref(name) {
            Ok(_) => Ok(false),
            Err(err) => {
                if let CtxError::NotFound = err {
                    Ok(true)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub(crate) fn with_ref_or_val<T, F>(ctx: &Ctx, name: Val, f: F) -> T
    where F: FnOnce(Either<&Val, Val>) -> T {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(ref1) = ctx.variables().get_ref(name) else {
                    return f(Either::That(Val::default()));
                };
                f(Either::This(ref1))
            }
            Either::That(val) => f(Either::That(val)),
        }
    }

    pub(crate) fn with_ref_mut_or_val<T, F>(ctx: &mut Ctx, name: Val, f: F) -> T
    where F: FnOnce(Either<&mut Val, Val>) -> T {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(ref1) = ctx.variables_mut().get_ref_mut(name) else {
                    return f(Either::That(Val::default()));
                };
                f(Either::This(ref1))
            }
            Either::That(val) => f(Either::That(val)),
        }
    }

    pub(crate) fn with_ref_dyn_or_val<T, F>(mut ctx: DynRef<Ctx>, name: Val, f: F) -> T
    where F: FnOnce(Either<DynRef<Val>, Val>) -> T {
        match Self::ref_or_val(name) {
            Either::This(s) => {
                let Ok(ref1) = ctx.map_res(|ctx| ctx.variables_mut().get_ref_dyn(s)) else {
                    return f(Either::That(Val::default()));
                };
                f(Either::This(ref1))
            }
            Either::That(val) => f(Either::That(val)),
        }
    }

    #[expect(unused)]
    pub(crate) fn with_ref<T, F>(ctx: &Ctx, name: Val, f: F) -> T
    where F: FnOnce(&Val) -> T {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(val) = ctx.variables().get_ref(name) else {
                    return f(&Val::default());
                };
                f(val)
            }
            Either::That(val) => f(&val),
        }
    }

    pub(crate) fn with_ref_lossless<F>(ctx: &Ctx, name: Val, f: F) -> Val
    where F: FnOnce(&Val) -> Val {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(val) = ctx.variables().get_ref(name) else {
                    return f(&Val::default());
                };
                f(val)
            }
            Either::That(val) => {
                let result = f(&val);
                Val::Pair(Pair::new(val, result).into())
            }
        }
    }

    #[expect(dead_code)]
    pub(crate) fn with_ref_mut<T, F>(ctx: &mut Ctx, name: Val, f: F) -> T
    where F: FnOnce(&mut Val) -> T {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(val) = ctx.variables_mut().get_ref_mut(name) else {
                    return f(&mut Val::default());
                };
                f(val)
            }
            Either::That(mut val) => f(&mut val),
        }
    }

    pub(crate) fn with_ref_mut_lossless<F>(ctx: &mut Ctx, name: Val, f: F) -> Val
    where F: FnOnce(&mut Val) -> Val {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(val) = ctx.variables_mut().get_ref_mut(name) else {
                    return f(&mut Val::default());
                };
                f(val)
            }
            Either::That(mut val) => {
                let result = f(&mut val);
                Val::Pair(Pair::new(val, result).into())
            }
        }
    }

    pub(crate) fn with_ref_mut_no_ret<F>(ctx: &mut Ctx, name: Val, f: F) -> Val
    where F: FnOnce(&mut Val) {
        match Self::ref_or_val(name) {
            Either::This(name) => {
                let Ok(val) = ctx.variables_mut().get_ref_mut(name) else {
                    f(&mut Val::default());
                    return Val::default();
                };
                f(val);
                Val::default()
            }
            Either::That(mut val) => {
                f(&mut val);
                Val::Pair(Pair::new(val, Val::default()).into())
            }
        }
    }

    pub(crate) fn escape_symbol(val: Val) -> Val {
        if let Val::Symbol(s) = val {
            Val::Symbol(Symbol::from_string(format!("{}{}", LITERAL_CHAR, &*s)))
        } else {
            val
        }
    }

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
