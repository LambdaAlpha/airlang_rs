use crate::{
    CtxError,
    Pair,
    Symbol,
    Val,
    core::{
        SYMBOL_ID_PREFIX,
        SYMBOL_REF_PREFIX,
    },
    ctx::{
        DynRef,
        map::CtxMapRef,
        ref1::CtxRef,
    },
    types::either::Either,
};

pub(crate) struct DefaultCtx;

impl DefaultCtx {
    pub(crate) fn get_or_default<'a, Ctx>(&self, ctx: Ctx, name: Symbol) -> Val
    where
        Ctx: CtxRef<'a>,
    {
        let Ok(ctx) = ctx.get_variables() else {
            return Val::default();
        };
        let Ok(val) = ctx.get_ref(name) else {
            return Val::default();
        };
        val.clone()
    }

    pub(crate) fn remove_or_default<'a, Ctx>(&self, ctx: Ctx, name: Symbol) -> Val
    where
        Ctx: CtxRef<'a>,
    {
        let Ok(variables) = ctx.get_variables_mut() else {
            return Val::default();
        };
        variables.remove(name).unwrap_or_default()
    }

    pub(crate) fn is_null<'a, Ctx>(&self, ctx: Ctx, name: Symbol) -> Result<bool, CtxError>
    where
        Ctx: CtxRef<'a>,
    {
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

    pub(crate) fn with_dyn<'a, Ctx, T, F>(&self, ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(Either<DynRef<Val>, Val>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    return f(Either::Right(Val::Symbol(s)));
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
                let Ok(ctx) = ctx.get_variables_dyn() else {
                    return f(Either::Right(Val::default()));
                };
                let Ok(mut dyn_ref) = ctx.ref1.get_ref_dyn(s) else {
                    return f(Either::Right(Val::default()));
                };
                dyn_ref.is_const |= ctx.is_const;
                f(Either::Left(dyn_ref))
            }
            val => f(Either::Right(val)),
        }
    }

    #[allow(unused)]
    pub(crate) fn with_ref<'a, Ctx, T, F>(&self, ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(&Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    return f(&Val::Symbol(s));
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
                let Ok(ctx) = ctx.get_variables() else {
                    return f(&Val::default());
                };
                let Ok(val) = ctx.get_ref(s) else {
                    return f(&Val::default());
                };
                f(val)
            }
            val => f(&val),
        }
    }

    pub(crate) fn with_ref_lossless<'a, Ctx, F>(&self, ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(&Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    let val = Val::Symbol(s);
                    let result = f(&val);
                    return Val::Pair(Pair::new(val, result).into());
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
                let Ok(ctx) = ctx.get_variables() else {
                    return f(&Val::default());
                };
                let Ok(val) = ctx.get_ref(s) else {
                    return f(&Val::default());
                };
                f(val)
            }
            val => {
                let result = f(&val);
                Val::Pair(Pair::new(val, result).into())
            }
        }
    }

    #[allow(unused)]
    pub(crate) fn with_ref_mut<'a, Ctx, T, F>(&self, ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(&mut Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    return f(&mut Val::Symbol(s));
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
                let Ok(ctx) = ctx.get_variables_mut() else {
                    return f(&mut Val::default());
                };
                let Ok(val) = ctx.get_ref_mut(s) else {
                    return f(&mut Val::default());
                };
                f(val)
            }
            mut val => f(&mut val),
        }
    }

    pub(crate) fn with_ref_mut_lossless<'a, Ctx, F>(&self, ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(&mut Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    let mut val = Val::Symbol(s);
                    let result = f(&mut val);
                    return Val::Pair(Pair::new(val, result).into());
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
                let Ok(ctx) = ctx.get_variables_mut() else {
                    return f(&mut Val::default());
                };
                let Ok(val) = ctx.get_ref_mut(s) else {
                    return f(&mut Val::default());
                };
                f(val)
            }
            mut val => {
                let result = f(&mut val);
                Val::Pair(Pair::new(val, result).into())
            }
        }
    }

    pub(crate) fn with_ref_mut_no_ret<'a, Ctx, F>(&self, ctx: Ctx, name: Val, f: F) -> Val
    where
        Ctx: CtxRef<'a>,
        F: FnOnce(&mut Val),
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let prefix = s.chars().next();
                if let Some(SYMBOL_ID_PREFIX) = prefix {
                    let s = Symbol::from_str(&s[1..]);
                    let mut val = Val::Symbol(s);
                    f(&mut val);
                    return val;
                }
                let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                    Symbol::from_str(&s[1..])
                } else {
                    s
                };
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
            mut val => {
                f(&mut val);
                val
            }
        }
    }
}
