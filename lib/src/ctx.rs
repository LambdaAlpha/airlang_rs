use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::Hash,
};

use ref1::CtxRef;

use crate::{
    map::Map,
    pair::Pair,
    symbol::Symbol,
    transform::SYMBOL_READ_PREFIX,
    types::either::Either,
    val::Val,
};

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
    Unexpected,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) map: CtxMap,
    pub(crate) meta: Option<Box<Ctx>>,
}

pub(crate) type CtxMap = Map<Symbol, CtxValue>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Invariant {
    // no limit
    None,
    // can't be assigned
    Final,
    // can't be modified
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) invariant: Invariant,
    pub(crate) val: Val,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct DynRef<'a, T> {
    pub(crate) ref1: &'a mut T,
    pub(crate) is_const: bool,
}

impl<'l> CtxRef<'l> for &'l mut Ctx {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        (&*self).get_ref(name)
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        if self.map.get(&name).is_none() {
            return Err(CtxError::NotFound);
        }
        let ctx_value = self.map.get_mut(&name).unwrap();
        let is_const = ctx_value.invariant == Invariant::Const;
        Ok(DynRef::new(&mut ctx_value.val, is_const))
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.map.remove(&name).unwrap().val)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        (&*self).is_assignable(name)
    }

    fn put_value(self, name: Symbol, val: CtxValue) -> Result<Option<Val>, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Ok(self.put_unchecked(name, val));
        };
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.put_unchecked(name, val))
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        value.invariant = Invariant::Final;
        Ok(())
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        (&*self).is_final(name)
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        value.invariant = Invariant::Const;
        Ok(())
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        (&*self).is_const(name)
    }

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        (&*self).get_meta()
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        let Some(meta) = &mut self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(meta)
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        let Some(meta) = &mut self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(DynRef::new(meta, false))
    }

    fn set_meta(self, meta: Option<Ctx>) -> Result<(), CtxError> {
        self.meta = meta.map(Box::new);
        Ok(())
    }
}

impl<'l> CtxRef<'l> for &'l Ctx {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        let Some(tagged_val) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(&tagged_val.val)
    }

    fn get_ref_mut(self, _name: Symbol) -> Result<&'l mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(self, _name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn remove(self, _name: Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        let Some(value) = self.map.get(&name) else {
            return true;
        };
        value.invariant == Invariant::None
    }

    fn put_value(self, _name: Symbol, _val: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant != Invariant::None)
    }

    fn set_const(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant == Invariant::Const)
    }

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        let Some(meta) = &self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(meta)
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_meta(self, _meta: Option<Ctx>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl Ctx {
    fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn into_val(mut self, name: Symbol) -> Result<Val, CtxError> {
        let Some(ctx_value) = self.map.remove(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(ctx_value.val)
    }
}

impl Ctx {
    pub(crate) fn new(map: CtxMap, meta: Option<Box<Ctx>>) -> Self {
        Self { map, meta }
    }
}

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

#[allow(unused)]
impl CtxValue {
    pub(crate) fn new(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::None,
            val,
        }
    }

    pub(crate) fn new_final(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Final,
            val,
        }
    }

    pub(crate) fn new_const(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Const,
            val,
        }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.invariant)
            .field(&self.val)
            .finish()
    }
}

impl<'a, T> DynRef<'a, T> {
    pub(crate) fn new(ref1: &'a mut T, is_const: bool) -> Self {
        DynRef { ref1, is_const }
    }

    pub(crate) fn as_const(&'a self) -> &'a T {
        self.ref1
    }

    pub(crate) fn as_mut(&'a mut self) -> Option<&'a mut T> {
        if self.is_const {
            None
        } else {
            Some(&mut self.ref1)
        }
    }
}

impl Display for CtxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CtxError::NotFound => {
                write!(f, "not found")
            }
            CtxError::AccessDenied => {
                write!(f, "access denied")
            }
            CtxError::Unexpected => {
                write!(f, "unexpected")
            }
        }
    }
}

impl Error for CtxError {}

pub(crate) mod ref1;

pub(crate) mod free;

pub(crate) mod constant;

pub(crate) mod mutable;
