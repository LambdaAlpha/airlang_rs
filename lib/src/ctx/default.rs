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
                    return f(Either::That(Self::remove_prefix(s)));
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
                let Ok(ctx) = ctx.get_variables_dyn() else {
                    return f(Either::That(Val::default()));
                };
                let Ok(mut dyn_ref) = ctx.ref1.get_ref_dyn(s) else {
                    return f(Either::That(Val::default()));
                };
                dyn_ref.is_const |= ctx.is_const;
                f(Either::This(dyn_ref))
            }
            val => f(Either::That(val)),
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
                    return f(&Self::remove_prefix(s));
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
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
                    let val = Self::remove_prefix(s);
                    let result = f(&val);
                    return Val::Pair(Pair::new(val, result).into());
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
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
                    return f(&mut Self::remove_prefix(s));
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
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
                    let mut val = Self::remove_prefix(s);
                    let result = f(&mut val);
                    return Val::Pair(Pair::new(val, result).into());
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
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
                    let mut val = Self::remove_prefix(s);
                    f(&mut val);
                    return val;
                }
                let s = Self::remove_prefix_expect(s, prefix, SYMBOL_REF_PREFIX);
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

    fn remove_prefix_expect(s: Symbol, real_prefix: Option<char>, expect_prefix: char) -> Symbol {
        let Some(prefix) = real_prefix else {
            return s;
        };
        if prefix == expect_prefix {
            Symbol::from_str(&s[1..])
        } else {
            s
        }
    }

    fn remove_prefix(s: Symbol) -> Val {
        Val::Symbol(Symbol::from_str(&s[1..]))
    }
}
