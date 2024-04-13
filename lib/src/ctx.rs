use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    map::Map,
    pair::Pair,
    symbol::Symbol,
    transformer::Transformer,
    types::either::Either,
    val::Val,
    FreeCtx,
};

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
    Unexpected,
}

pub(crate) trait CtxTrait {
    fn get_ref(&self, name: &str) -> Result<&Val, CtxError>;

    fn get_ref_mut(&mut self, name: &str) -> Result<&mut Val, CtxError>;

    fn get_ref_dyn(&mut self, name: &str) -> Result<DynRef<Val>, CtxError>;

    fn remove(&mut self, name: &str) -> Result<Val, CtxError>;

    fn put_value(&mut self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError>;

    fn set_final(&mut self, name: &str) -> Result<(), CtxError>;

    fn is_final(&self, name: &str) -> Result<bool, CtxError>;

    fn set_const(&mut self, name: &str) -> Result<(), CtxError>;

    fn is_const(&self, name: &str) -> Result<bool, CtxError>;

    fn get_meta(&self) -> Result<&Ctx, CtxError>;

    #[allow(unused)]
    fn get_meta_mut(&mut self) -> Result<&mut Ctx, CtxError>;

    fn get_meta_dyn(&mut self) -> Result<DynRef<Ctx>, CtxError>;

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError>;
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

impl CtxTrait for Ctx {
    fn get_ref(&self, name: &str) -> Result<&Val, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.get_ref(name);
        }
        let Some(tagged_val) = dispatch.ctx.map.get(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(&tagged_val.val)
    }

    fn get_ref_mut(&mut self, name: &str) -> Result<&mut Val, CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.get_ref_mut(name);
        }
        let Some(value) = dispatch.ctx.map.get_mut(name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    fn get_ref_dyn(&mut self, name: &str) -> Result<DynRef<Val>, CtxError> {
        let dispatch = self.dispatch_dyn(name)?;
        if dispatch.foreword {
            let mut dyn_ref = dispatch.ctx.ref1.get_ref_dyn(name)?;
            dyn_ref.is_const = dispatch.ctx.is_const || dyn_ref.is_const;
            return Ok(dyn_ref);
        }
        if dispatch.ctx.ref1.map.get(name).is_none() {
            return Err(CtxError::NotFound);
        }
        let ctx_value = dispatch.ctx.ref1.map.get_mut(name).unwrap();
        let is_const = ctx_value.invariant == Invariant::Const;
        Ok(DynRef::new(&mut ctx_value.val, is_const))
    }

    fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.remove(name);
        }
        let Some(value) = dispatch.ctx.map.get(name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(dispatch.ctx.map.remove(name).unwrap().val)
    }

    fn put_value(&mut self, name: Symbol, val: CtxValue) -> Result<Option<Val>, CtxError> {
        let dispatch = self.dispatch_mut(&name)?;
        if dispatch.foreword {
            return dispatch.ctx.put_value(name, val);
        }
        let Some(value) = dispatch.ctx.map.get(&name) else {
            return Ok(dispatch.ctx.put_unchecked(name, val));
        };
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(dispatch.ctx.put_unchecked(name, val))
    }

    fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.set_final(name);
        }
        let Some(value) = dispatch.ctx.map.get_mut(name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        value.invariant = Invariant::Final;
        Ok(())
    }

    fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.is_final(name);
        }
        let Some(value) = dispatch.ctx.map.get(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant != Invariant::None)
    }

    fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.set_final(name);
        }
        let Some(value) = dispatch.ctx.map.get_mut(name) else {
            return Err(CtxError::NotFound);
        };
        value.invariant = Invariant::Const;
        Ok(())
    }

    fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.is_final(name);
        }
        let Some(value) = dispatch.ctx.map.get(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant == Invariant::Const)
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        let Some(meta) = &self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(meta)
    }

    fn get_meta_mut(&mut self) -> Result<&mut Ctx, CtxError> {
        let Some(meta) = &mut self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(meta)
    }

    fn get_meta_dyn(&mut self) -> Result<DynRef<Ctx>, CtxError> {
        let Some(meta) = &mut self.meta else {
            return Err(CtxError::NotFound);
        };
        Ok(DynRef::new(meta, false))
    }

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError> {
        self.meta = meta.map(Box::new);
        Ok(())
    }
}

struct DispatchConst<'a> {
    foreword: bool,
    ctx: &'a Ctx,
}

struct DispatchMut<'a> {
    foreword: bool,
    ctx: &'a mut Ctx,
}

struct DispatchDyn<'a> {
    foreword: bool,
    ctx: DynRef<'a, Ctx>,
}

struct DispatchOwned {
    foreword: bool,
    ctx: Ctx,
}

impl Ctx {
    fn dispatch_const(&self, name: &str) -> Result<DispatchConst, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchConst::this(self));
        };
        let Ok(dispatcher) = meta.get_ref(DISPATCHER) else {
            return Ok(DispatchConst::this(self));
        };
        let Val::Func(dispatcher) = dispatcher else {
            return Err(CtxError::Unexpected);
        };
        if !dispatcher.is_ctx_free() {
            return Err(CtxError::Unexpected);
        }
        let target_name = Val::Symbol(Symbol::from_str(name));
        let ctx_name = dispatcher.transform(&mut FreeCtx, target_name);
        match ctx_name {
            Val::Bool(b) => {
                let dispatch = if b.bool() {
                    DispatchConst::foreword(meta)
                } else {
                    DispatchConst::this(self)
                };
                Ok(dispatch)
            }
            Val::Symbol(s) => {
                let Some(val) = self.map.get(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = &val.val else {
                    return Err(CtxError::Unexpected);
                };
                Ok(DispatchConst::foreword(ctx))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn dispatch_mut(&mut self, name: &str) -> Result<DispatchMut, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchMut::this(self));
        };
        let Ok(dispatcher) = meta.get_ref(DISPATCHER) else {
            return Ok(DispatchMut::this(self));
        };
        let Val::Func(dispatcher) = dispatcher else {
            return Err(CtxError::Unexpected);
        };
        if !dispatcher.is_ctx_free() {
            return Err(CtxError::Unexpected);
        }
        let target_name = Val::Symbol(Symbol::from_str(name));
        let ctx_name = dispatcher.transform(&mut FreeCtx, target_name);
        match ctx_name {
            Val::Bool(b) => {
                let dispatch = if b.bool() {
                    DispatchMut::foreword(self.meta.as_mut().unwrap())
                } else {
                    DispatchMut::this(self)
                };
                Ok(dispatch)
            }
            Val::Symbol(s) => {
                let Some(ctx_value) = self.map.get_mut(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = &mut ctx_value.val else {
                    return Err(CtxError::Unexpected);
                };
                if matches!(ctx_value.invariant, Invariant::Const) {
                    return Err(CtxError::AccessDenied);
                }
                Ok(DispatchMut::foreword(ctx))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn dispatch_dyn(&mut self, name: &str) -> Result<DispatchDyn, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchDyn::this(DynRef::new(self, false)));
        };
        let Ok(dispatcher) = meta.get_ref(DISPATCHER) else {
            return Ok(DispatchDyn::this(DynRef::new(self, false)));
        };
        let Val::Func(dispatcher) = &dispatcher else {
            return Err(CtxError::Unexpected);
        };
        if !dispatcher.is_ctx_free() {
            return Err(CtxError::Unexpected);
        }
        let target_name = Val::Symbol(Symbol::from_str(name));
        let ctx_name = dispatcher.transform(&mut FreeCtx, target_name);
        match ctx_name {
            Val::Bool(b) => {
                let dispatch = if b.bool() {
                    DispatchDyn::foreword(DynRef::new(self.meta.as_mut().unwrap(), false))
                } else {
                    DispatchDyn::this(DynRef::new(self, false))
                };
                Ok(dispatch)
            }
            Val::Symbol(s) => {
                let Some(ctx_value) = self.map.get_mut(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = &mut ctx_value.val else {
                    return Err(CtxError::Unexpected);
                };
                let is_const = ctx_value.invariant == Invariant::Const;
                Ok(DispatchDyn::foreword(DynRef::new(ctx, is_const)))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn dispatch_owned(mut self, name: &str) -> Result<DispatchOwned, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchOwned::this(self));
        };
        let Ok(dispatcher) = meta.get_ref(DISPATCHER) else {
            return Ok(DispatchOwned::this(self));
        };
        let Val::Func(dispatcher) = dispatcher else {
            return Err(CtxError::Unexpected);
        };
        if !dispatcher.is_ctx_free() {
            return Err(CtxError::Unexpected);
        }
        let target_name = Val::Symbol(Symbol::from_str(name));
        let ctx_name = dispatcher.transform(&mut FreeCtx, target_name);
        match ctx_name {
            Val::Bool(b) => {
                let dispatch = if b.bool() {
                    DispatchOwned::foreword(*self.meta.unwrap())
                } else {
                    DispatchOwned::this(self)
                };
                Ok(dispatch)
            }
            Val::Symbol(s) => {
                let Some(val) = self.map.remove(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = val.val else {
                    return Err(CtxError::Unexpected);
                };
                Ok(DispatchOwned::foreword(*ctx.0))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn into_val(self, name: &str) -> Result<Val, CtxError> {
        let mut dispatch = self.dispatch_owned(name)?;
        if dispatch.foreword {
            return dispatch.ctx.into_val(name);
        }
        let Some(ctx_value) = dispatch.ctx.map.remove(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(ctx_value.val)
    }
}

impl<'a> DispatchConst<'a> {
    fn this(ctx: &'a Ctx) -> Self {
        Self {
            foreword: false,
            ctx,
        }
    }

    fn foreword(ctx: &'a Ctx) -> Self {
        Self {
            foreword: true,
            ctx,
        }
    }
}

impl<'a> DispatchMut<'a> {
    fn this(ctx: &'a mut Ctx) -> Self {
        Self {
            foreword: false,
            ctx,
        }
    }

    fn foreword(ctx: &'a mut Ctx) -> Self {
        Self {
            foreword: true,
            ctx,
        }
    }
}

impl<'a> DispatchDyn<'a> {
    fn this(ctx: DynRef<'a, Ctx>) -> Self {
        Self {
            foreword: false,
            ctx,
        }
    }

    fn foreword(ctx: DynRef<'a, Ctx>) -> Self {
        Self {
            foreword: true,
            ctx,
        }
    }
}

impl DispatchOwned {
    fn this(ctx: Ctx) -> Self {
        Self {
            foreword: false,
            ctx,
        }
    }

    fn foreword(ctx: Ctx) -> Self {
        Self {
            foreword: true,
            ctx,
        }
    }
}

pub(crate) struct DefaultCtx;

impl DefaultCtx {
    pub(crate) fn get_or_default<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Val {
        let Ok(val) = ctx.get_ref(name) else {
            return Val::default();
        };
        val.clone()
    }

    pub(crate) fn is_null<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Result<bool, CtxError> {
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

    pub(crate) fn with_dyn<Ctx: CtxTrait, T, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> T
    where
        T: Default,
        F: FnOnce(Either<DynRef<Val>, Val>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(dyn_ref) = ctx.get_ref_dyn(&s) else {
                    return T::default();
                };
                f(Either::Left(dyn_ref))
            }
            val => f(Either::Right(val)),
        }
    }

    #[allow(unused)]
    pub(crate) fn with_ref<Ctx: CtxTrait, T, F>(&self, ctx: &Ctx, name: Val, f: F) -> T
    where
        T: Default,
        F: FnOnce(&Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_ref(&s) else {
                    return T::default();
                };
                f(val)
            }
            val => f(&val),
        }
    }

    pub(crate) fn with_ref_lossless<Ctx: CtxTrait, F>(&self, ctx: &Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_ref(&s) else {
                    return Val::default();
                };
                f(val)
            }
            val => {
                let result = f(&val);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
        }
    }

    #[allow(unused)]
    pub(crate) fn with_ref_mut<Ctx: CtxTrait, T, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> T
    where
        T: Default,
        F: FnOnce(&mut Val) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_ref_mut(&s) else {
                    return T::default();
                };
                f(val)
            }
            mut val => f(&mut val),
        }
    }

    pub(crate) fn with_ref_mut_lossless<Ctx: CtxTrait, F>(
        &self,
        ctx: &mut Ctx,
        name: Val,
        f: F,
    ) -> Val
    where
        F: FnOnce(&mut Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_ref_mut(&s) else {
                    return Val::default();
                };
                f(val)
            }
            mut val => {
                let result = f(&mut val);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
        }
    }

    pub(crate) fn with_ref_mut_no_ret<Ctx: CtxTrait, F>(
        &self,
        ctx: &mut Ctx,
        name: Val,
        f: F,
    ) -> Val
    where
        F: FnOnce(&mut Val),
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_ref_mut(&s) else {
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

pub(crate) const DISPATCHER: &str = "dispatcher";
