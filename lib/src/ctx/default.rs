use crate::{
    ctx::{
        ref1::CtxRef,
        DynRef,
    },
    mode::SYMBOL_READ_PREFIX,
    types::either::Either,
    CtxError,
    Pair,
    Symbol,
    Val,
};

pub(crate) struct DefaultCtx;

impl DefaultCtx {
    pub(crate) fn get_or_default<'a, Ctx>(&self, ctx: Ctx, name: Symbol) -> Val
    where
        Ctx: CtxRef<'a>,
    {
        let Ok(val) = ctx.get_ref(name) else {
            return Val::default();
        };
        val.clone()
    }

    pub(crate) fn is_null<'a, Ctx>(&self, ctx: Ctx, name: Symbol) -> Result<bool, CtxError>
    where
        Ctx: CtxRef<'a>,
    {
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
        T: Default,
        F: FnOnce(Either<DynRef<Val>, Val>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    f(Either::Right(Val::Symbol(s)))
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(dyn_ref) = ctx.get_ref_dyn(s) else {
                        return T::default();
                    };
                    f(Either::Left(dyn_ref))
                }
                _ => {
                    let Ok(dyn_ref) = ctx.get_ref_dyn(s) else {
                        return T::default();
                    };
                    f(Either::Left(dyn_ref))
                }
            },
            val => f(Either::Right(val)),
        }
    }

    #[allow(unused)]
    pub(crate) fn with_ref<'a, Ctx, T, F>(&self, ctx: Ctx, name: Val, f: F) -> T
    where
        Ctx: CtxRef<'a>,
        T: Default,
        F: FnOnce(&Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    f(&Val::Symbol(s))
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(val) = ctx.get_ref(s) else {
                        return T::default();
                    };
                    f(val)
                }
                _ => {
                    let Ok(val) = ctx.get_ref(s) else {
                        return T::default();
                    };
                    f(val)
                }
            },
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
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let val = Val::Symbol(s);
                    let result = f(&val);
                    Val::Pair(Pair::new(val, result).into())
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(val) = ctx.get_ref(s) else {
                        return Val::default();
                    };
                    f(val)
                }
                _ => {
                    let Ok(val) = ctx.get_ref(s) else {
                        return Val::default();
                    };
                    f(val)
                }
            },
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
        T: Default,
        F: FnOnce(&mut Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    f(&mut Val::Symbol(s))
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return T::default();
                    };
                    f(val)
                }
                _ => {
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return T::default();
                    };
                    f(val)
                }
            },
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
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let mut val = Val::Symbol(s);
                    let result = f(&mut val);
                    Val::Pair(Pair::new(val, result).into())
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return Val::default();
                    };
                    f(val)
                }
                _ => {
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return Val::default();
                    };
                    f(val)
                }
            },
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
            Val::Symbol(s) => match s.chars().next() {
                Some(Symbol::ID_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let mut val = Val::Symbol(s);
                    f(&mut val);
                    val
                }
                Some(SYMBOL_READ_PREFIX) => {
                    let s = Symbol::from_str(&s[1..]);
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return Val::default();
                    };
                    f(val);
                    Val::default()
                }
                _ => {
                    let Ok(val) = ctx.get_ref_mut(s) else {
                        return Val::default();
                    };
                    f(val);
                    Val::default()
                }
            },
            mut val => {
                f(&mut val);
                val
            }
        }
    }
}
