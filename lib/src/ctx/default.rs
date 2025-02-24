use crate::{
    CtxError,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    ctx::{
        map::{
            CtxMapRef,
            DynRef,
        },
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    mode::{
        eval::EVAL,
        symbol::{
            LITERAL_CHAR,
            MOVE_CHAR,
            REF_CHAR,
        },
    },
    transformer::Transformer,
    types::either::Either,
};

pub(crate) struct DefaultCtx;

impl DefaultCtx {
    pub(crate) fn get_or_default<'a, Ctx>(ctx: Ctx, name: Symbol) -> Val
    where Ctx: CtxRef<'a> {
        let Ok(ctx) = ctx.get_variables() else {
            return Val::default();
        };
        let Ok(val) = ctx.get_ref(name) else {
            return Val::default();
        };
        val.clone()
    }

    pub(crate) fn remove_or_default<'a, Ctx>(ctx: Ctx, name: Symbol) -> Val
    where Ctx: CtxRef<'a> {
        let Ok(variables) = ctx.get_variables_mut() else {
            return Val::default();
        };
        variables.remove(name).unwrap_or_default()
    }

    pub(crate) fn is_null<'a, Ctx>(ctx: Ctx, name: Symbol) -> Result<bool, CtxError>
    where Ctx: CtxRef<'a> {
        let ctx = ctx.get_variables()?;
        match ctx.get_ref(name) {
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

    pub(crate) fn with_dyn<'a, Ctx, T, F>(ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(Either<DynRef<Val>, Val>) -> T, {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables_dyn() else {
                    return f(Either::That(Val::default()));
                };
                let Ok(mut dyn_ref) = ctx.ref1.get_ref_dyn(s) else {
                    return f(Either::That(Val::default()));
                };
                dyn_ref.is_const |= ctx.is_const;
                f(Either::This(dyn_ref))
            }
            Either::That(val) => f(Either::That(val)),
        }
    }

    #[expect(dead_code)]
    pub(crate) fn with_ref<'a, Ctx, T, F>(ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(&Val) -> T, {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables() else {
                    return f(&Val::default());
                };
                let Ok(val) = ctx.get_ref(s) else {
                    return f(&Val::default());
                };
                f(val)
            }
            Either::That(val) => f(&val),
        }
    }

    pub(crate) fn with_ref_lossless<'a, Ctx, F>(ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(&Val) -> Val, {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables() else {
                    return f(&Val::default());
                };
                let Ok(val) = ctx.get_ref(s) else {
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
    pub(crate) fn with_ref_mut<'a, Ctx, T, F>(ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(&mut Val) -> T, {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables_mut() else {
                    return f(&mut Val::default());
                };
                let Ok(val) = ctx.get_ref_mut(s) else {
                    return f(&mut Val::default());
                };
                f(val)
            }
            Either::That(mut val) => f(&mut val),
        }
    }

    pub(crate) fn with_ref_mut_lossless<'a, Ctx, F>(ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(&mut Val) -> Val, {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables_mut() else {
                    return f(&mut Val::default());
                };
                let Ok(val) = ctx.get_ref_mut(s) else {
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

    pub(crate) fn with_ref_mut_no_ret<'a, Ctx, F>(ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxMeta<'a>,
        F: FnOnce(&mut Val), {
        let val = Self::ref_or_val(name);
        match val {
            Either::This(s) => {
                let Ok(ctx) = ctx.get_variables_mut() else {
                    f(&mut Val::default());
                    return Val::default();
                };
                let Ok(val) = ctx.get_ref_mut(s) else {
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

    pub(crate) fn eval_escape_symbol(ctx: MutFnCtx, input: Val) -> Val {
        let Val::Symbol(s) = &input else {
            let val = EVAL.transform(ctx, input);
            return Self::escape_symbol(val);
        };
        let prefix = s.chars().next();
        if let Some(MOVE_CHAR) = prefix {
            let val = DefaultCtx::remove_or_default(ctx, Symbol::from_str(&s[1 ..]));
            return Self::escape_symbol(val);
        }
        input
    }

    fn escape_symbol(val: Val) -> Val {
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
