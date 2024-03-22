use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use thiserror::Error;

use crate::{
    map::Map,
    pair::Pair,
    symbol::Symbol,
    transformer::Transformer,
    types::either::Either,
    val::Val,
    FreeCtx,
};

#[derive(Error, Copy, Clone, Debug)]
pub enum CtxError {
    #[error("not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("unexpected")]
    Unexpected,
}

pub(crate) trait CtxTrait {
    fn get(&self, name: &str) -> Result<Val, CtxError>;

    fn remove(&mut self, name: &str) -> Result<Val, CtxError>;

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError>;

    fn set_final(&mut self, name: &str) -> Result<(), CtxError>;

    fn set_const(&mut self, name: &str) -> Result<(), CtxError>;

    fn is_final(&self, name: &str) -> Result<bool, CtxError>;

    fn is_const(&self, name: &str) -> Result<bool, CtxError>;

    fn is_null(&self, name: &str) -> Result<bool, CtxError>;

    fn get_meta(&self) -> Result<&Ctx, CtxError>;

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError>;

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError>;

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError>;

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError>;
}

pub(crate) type NameMap = Map<Symbol, TaggedVal>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InvariantTag {
    // no limit
    None,
    // can't be assigned
    Final,
    // can't be modified
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct TaggedVal {
    pub(crate) tag: InvariantTag,
    pub(crate) val: Val,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct TaggedRef<'a, T> {
    pub(crate) val_ref: &'a mut T,
    pub(crate) is_const: bool,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) name_map: NameMap,
    pub(crate) meta: Option<Box<Ctx>>,
}

struct DispatchConst<'a> {
    foreword: bool,
    ctx: &'a Ctx,
}

struct DispatchMut<'a> {
    foreword: bool,
    ctx: &'a mut Ctx,
}

struct DispatchTagged<'a> {
    foreword: bool,
    ctx: TaggedRef<'a, Ctx>,
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
        let Ok(dispatcher) = meta.get(DISPATCHER) else {
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
                let Some(val) = self.name_map.get(&s) else {
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
        let Ok(dispatcher) = meta.get(DISPATCHER) else {
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
                let Some(tagged_val) = self.name_map.get_mut(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = &mut tagged_val.val else {
                    return Err(CtxError::Unexpected);
                };
                if matches!(tagged_val.tag, InvariantTag::Const) {
                    return Err(CtxError::AccessDenied);
                }
                Ok(DispatchMut::foreword(ctx))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn dispatch_tagged(&mut self, is_const: bool, name: &str) -> Result<DispatchTagged, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchTagged::this(TaggedRef::new(self, is_const)));
        };
        let Ok(dispatcher) = meta.get(DISPATCHER) else {
            return Ok(DispatchTagged::this(TaggedRef::new(self, is_const)));
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
                    DispatchTagged::foreword(TaggedRef::new(self.meta.as_mut().unwrap(), is_const))
                } else {
                    DispatchTagged::this(TaggedRef::new(self, is_const))
                };
                Ok(dispatch)
            }
            Val::Symbol(s) => {
                let Some(tagged_val) = self.name_map.get_mut(&s) else {
                    return Err(CtxError::Unexpected);
                };
                let Val::Ctx(ctx) = &mut tagged_val.val else {
                    return Err(CtxError::Unexpected);
                };
                let is_const = is_const || matches!(tagged_val.tag, InvariantTag::Const);
                Ok(DispatchTagged::foreword(TaggedRef::new(ctx, is_const)))
            }
            _ => Err(CtxError::Unexpected),
        }
    }

    fn dispatch_owned(mut self, name: &str) -> Result<DispatchOwned, CtxError> {
        let Some(meta) = &self.meta else {
            return Ok(DispatchOwned::this(self));
        };
        let Ok(dispatcher) = meta.get(DISPATCHER) else {
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
                let Some(val) = self.name_map.remove(&s) else {
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

    pub(crate) fn get(&self, name: &str) -> Result<Val, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.get(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(tagged_val.val.clone())
    }

    pub(crate) fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.remove(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(name) else {
            return Err(CtxError::NotFound);
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Err(CtxError::AccessDenied);
        }
        Ok(dispatch.ctx.name_map.remove(name).unwrap().val)
    }

    pub(crate) fn put_val(
        &mut self,
        name: Symbol,
        val: TaggedVal,
    ) -> Result<Option<Val>, CtxError> {
        let dispatch = self.dispatch_mut(&name)?;
        if dispatch.foreword {
            return dispatch.ctx.put_val(name, val);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(&name) else {
            return Ok(dispatch.ctx.put_unchecked(name, val));
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Err(CtxError::AccessDenied);
        }
        Ok(dispatch.ctx.put_unchecked(name, val))
    }

    fn put_unchecked(&mut self, name: Symbol, val: TaggedVal) -> Option<Val> {
        self.name_map
            .insert(name, val)
            .map(|tagged_val| tagged_val.val)
    }

    pub(crate) fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.set_final(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get_mut(name) else {
            return Err(CtxError::NotFound);
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Err(CtxError::AccessDenied);
        }
        tagged_val.tag = InvariantTag::Final;
        Ok(())
    }

    pub(crate) fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        let dispatch = self.dispatch_mut(name)?;
        if dispatch.foreword {
            return dispatch.ctx.set_const(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get_mut(name) else {
            return Err(CtxError::NotFound);
        };
        tagged_val.tag = InvariantTag::Const;
        Ok(())
    }

    pub(crate) fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.is_final(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(name) else {
            return Err(CtxError::NotFound);
        };
        let is_final = matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const);
        Ok(is_final)
    }

    pub(crate) fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.is_const(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(name) else {
            return Err(CtxError::NotFound);
        };
        let is_const = matches!(&tagged_val.tag, InvariantTag::Const);
        Ok(is_const)
    }

    pub(crate) fn get_tagged_ref(
        &mut self,
        is_const: bool,
        name: &str,
    ) -> Result<TaggedRef<Val>, CtxError> {
        let dispatch = self.dispatch_tagged(is_const, name)?;
        if dispatch.foreword {
            return dispatch
                .ctx
                .val_ref
                .get_tagged_ref(dispatch.ctx.is_const, name);
        }
        if dispatch.ctx.val_ref.name_map.get(name).is_none() {
            return Err(CtxError::NotFound);
        }
        let tagged_val = dispatch.ctx.val_ref.name_map.get_mut(name).unwrap();
        let is_const = is_const || matches!(tagged_val.tag, InvariantTag::Const);
        Ok(TaggedRef::new(&mut tagged_val.val, is_const))
    }

    pub(crate) fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        let dispatch = self.dispatch_const(name)?;
        if dispatch.foreword {
            return dispatch.ctx.get_const_ref(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.get(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(&tagged_val.val)
    }

    pub(crate) fn into_val(self, name: &str) -> Result<Val, CtxError> {
        let mut dispatch = self.dispatch_owned(name)?;
        if dispatch.foreword {
            return dispatch.ctx.into_val(name);
        }
        let Some(tagged_val) = dispatch.ctx.name_map.remove(name) else {
            return Err(CtxError::NotFound);
        };
        Ok(tagged_val.val)
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

impl<'a> DispatchTagged<'a> {
    fn this(ctx: TaggedRef<'a, Ctx>) -> Self {
        Self {
            foreword: false,
            ctx,
        }
    }

    fn foreword(ctx: TaggedRef<'a, Ctx>) -> Self {
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
    #[allow(unused)]
    pub(crate) fn get<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Result<Val, CtxError> {
        ctx.get_const_ref(name).cloned()
    }

    pub(crate) fn is_null<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Result<bool, CtxError> {
        match ctx.get_const_ref(name) {
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

    pub(crate) fn get_ref_or_val<Ctx: CtxTrait, T, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> T
    where
        T: Default,
        F: FnOnce(Either<TaggedRef<Val>, Val>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(tagged_ref) = ctx.get_tagged_ref(&s) else {
                    return T::default();
                };
                f(Either::Left(tagged_ref))
            }
            val => f(Either::Right(val)),
        }
    }

    #[allow(unused)]
    pub(crate) fn get_tagged_ref<Ctx: CtxTrait, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(TaggedRef<Val>) -> Val,
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => f(tagged_ref),
            Either::Right(mut val) => {
                let tagged_ref = TaggedRef::new(&mut val, false);
                let result = f(tagged_ref);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
        })
    }

    pub(crate) fn get_const_ref<Ctx: CtxTrait, F>(&self, ctx: &Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Ok(val) = ctx.get_const_ref(&s) else {
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

    pub(crate) fn get_mut_ref<Ctx: CtxTrait, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&mut Val) -> Val,
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => {
                if tagged_ref.is_const {
                    return Val::default();
                }
                f(tagged_ref.val_ref)
            }
            Either::Right(mut val) => {
                let result = f(&mut val);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
        })
    }

    pub(crate) fn get_mut_ref_no_ret<Ctx: CtxTrait, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&mut Val),
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => {
                if tagged_ref.is_const {
                    return Val::default();
                }
                f(tagged_ref.val_ref);
                Val::default()
            }
            Either::Right(mut val) => {
                f(&mut val);
                val
            }
        })
    }
}

impl Debug for TaggedVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.tag).field(&self.val).finish()
    }
}

impl<'a, T> TaggedRef<'a, T> {
    pub(crate) fn new(val_ref: &'a mut T, is_const: bool) -> Self {
        TaggedRef { val_ref, is_const }
    }

    pub(crate) fn as_const(&'a self) -> &'a T {
        self.val_ref
    }

    pub(crate) fn as_mut(&'a mut self) -> Option<&'a mut T> {
        if self.is_const {
            None
        } else {
            Some(&mut self.val_ref)
        }
    }
}

pub(crate) const DISPATCHER: &str = "dispatcher";
