use {
    crate::{
        semantics::val::{
            CtxVal,
            Val,
        },
        types::{
            Either,
            Map,
            Pair,
            Symbol,
        },
    },
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::Hash,
    },
};

#[derive(Copy, Clone)]
pub(crate) enum CtxError {
    NotFound,
    AccessDenied,
    NotExpected,
}

pub(crate) trait CtxTrait {
    fn get(&self, name: &str) -> Result<Val, CtxError>;

    fn remove(&mut self, name: &str) -> Result<Val, CtxError>;

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError>;

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError>;

    fn set_final(&mut self, name: &str) -> Result<(), CtxError>;

    fn set_const(&mut self, name: &str) -> Result<(), CtxError>;

    fn is_final(&self, name: &str) -> Result<bool, CtxError>;

    fn is_const(&self, name: &str) -> Result<bool, CtxError>;

    fn is_null(&self, name: &str) -> Result<bool, CtxError>;

    fn is_local(&self, name: &str) -> Result<bool, CtxError>;

    fn get_meta(&self) -> Result<&Ctx, CtxError>;

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError>;

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError>;

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError>;

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError>;

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Result<&Val, CtxError>; N]
    where
        Self: Sized;
}

pub(crate) type NameMap = Map<Symbol, TaggedVal>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InvariantTag {
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
pub(crate) struct Ctx {
    pub(crate) name_map: NameMap,
    pub(crate) meta: Option<Box<Ctx>>,
}

impl Ctx {
    pub(crate) fn get(&self, name: &str) -> Result<Val, CtxError> {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_const_super_ctx()?.get(name);
        };
        Ok(tagged_val.val.clone())
    }

    pub(crate) fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_mut_super_ctx()?.remove(name);
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.name_map.remove(name).unwrap().val)
    }

    pub(crate) fn put_val(
        &mut self,
        name: Symbol,
        val: TaggedVal,
    ) -> Result<Option<Val>, CtxError> {
        let Some(tagged_val) = self.name_map.get(&name) else {
            return match self.get_mut_super_ctx() {
                Ok(super_ctx) => super_ctx.put_val(name, val),
                Err(CtxError::NotFound) => Ok(self.put_unchecked(name, val)),
                Err(err) => Err(err),
            };
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.put_unchecked(name, val))
    }

    pub(crate) fn put_val_local(
        &mut self,
        name: Symbol,
        val: TaggedVal,
    ) -> Result<Option<Val>, CtxError> {
        let (None
        | Some(TaggedVal {
            tag: InvariantTag::None,
            ..
        })) = self.name_map.get(&name)
        else {
            return Err(CtxError::AccessDenied);
        };
        Ok(self.put_unchecked(name, val))
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: TaggedVal) -> Option<Val> {
        self.name_map
            .insert(name, val)
            .map(|tagged_val| tagged_val.val)
    }

    pub(crate) fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return self.get_mut_super_ctx()?.set_final(name);
        };
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return Err(CtxError::AccessDenied);
        }
        tagged_val.tag = InvariantTag::Final;
        Ok(())
    }

    pub(crate) fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return self.get_mut_super_ctx()?.set_const(name);
        };
        tagged_val.tag = InvariantTag::Const;
        Ok(())
    }

    pub(crate) fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_const_super_ctx()?.is_final(name);
        };
        let is_final = matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const);
        Ok(is_final)
    }

    pub(crate) fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_const_super_ctx()?.is_const(name);
        };
        let is_const = matches!(&tagged_val.tag, InvariantTag::Const);
        Ok(is_const)
    }

    pub(crate) fn is_local(&self, name: &str) -> bool {
        self.name_map.get(name).is_some()
    }

    pub(crate) fn get_tagged_ref(
        &mut self,
        is_const: bool,
        name: &str,
    ) -> Result<TaggedRef<Val>, CtxError> {
        if self.name_map.get(name).is_none() {
            let super_ctx = self.get_tagged_super_ctx()?;
            let is_const = is_const || super_ctx.is_const;
            return super_ctx.val_ref.get_tagged_ref(is_const, name);
        }
        let tagged_val = self.name_map.get_mut(name).unwrap();
        let is_const = is_const || matches!(tagged_val.tag, InvariantTag::Const);
        Ok(TaggedRef::new(&mut tagged_val.val, is_const))
    }

    pub(crate) fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_const_super_ctx()?.get_const_ref(name);
        };
        Ok(&tagged_val.val)
    }

    pub(crate) fn get_many_const_ref<const N: usize>(
        &self,
        names: [&str; N],
    ) -> [Result<&Val, CtxError>; N] {
        match self.get_const_super_ctx() {
            Ok(super_ctx) => names.map(|name| {
                let Some(tagged_val) = self.name_map.get(name) else {
                    return super_ctx.get_const_ref(name);
                };
                Ok(&tagged_val.val)
            }),
            Err(err) => names.map(|name| {
                let Some(tagged_val) = self.name_map.get(name) else {
                    return Err(err);
                };
                Ok(&tagged_val.val)
            }),
        }
    }

    fn get_tagged_super_ctx(&mut self) -> Result<TaggedRef<Ctx>, CtxError> {
        let Some(meta) = &self.meta else {
            return Err(CtxError::NotFound);
        };
        let Val::Symbol(name) = meta.get(names::SUPER)? else {
            return Err(CtxError::NotExpected);
        };
        let Some(tagged_val) = self.name_map.get_mut(&name) else {
            return Err(CtxError::NotExpected);
        };
        let Val::Ctx(CtxVal(super_ctx)) = &mut tagged_val.val else {
            return Err(CtxError::NotExpected);
        };
        let is_const = matches!(tagged_val.tag, InvariantTag::Const);
        Ok(TaggedRef::new(super_ctx, is_const))
    }

    fn get_mut_super_ctx(&mut self) -> Result<&mut Ctx, CtxError> {
        let Some(meta) = &self.meta else {
            return Err(CtxError::NotFound);
        };
        let Val::Symbol(name) = meta.get(names::SUPER)? else {
            return Err(CtxError::NotExpected);
        };
        let Some(tagged_val) = self.name_map.get_mut(&name) else {
            return Err(CtxError::NotExpected);
        };
        let Val::Ctx(CtxVal(super_ctx)) = &mut tagged_val.val else {
            return Err(CtxError::NotExpected);
        };
        if matches!(tagged_val.tag, InvariantTag::Const) {
            return Err(CtxError::AccessDenied);
        }
        Ok(super_ctx)
    }

    fn get_const_super_ctx(&self) -> Result<&Ctx, CtxError> {
        let Some(meta) = &self.meta else {
            return Err(CtxError::NotFound);
        };
        let Val::Symbol(name) = meta.get(names::SUPER)? else {
            return Err(CtxError::NotExpected);
        };
        let Some(super_ctx) = self.name_map.get(&name) else {
            return Err(CtxError::NotExpected);
        };
        let Val::Ctx(CtxVal(super_ctx)) = &super_ctx.val else {
            return Err(CtxError::NotExpected);
        };
        Ok(super_ctx)
    }

    pub(crate) fn into_val(mut self, name: &str) -> Val {
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
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

    #[allow(unused)]
    pub(crate) fn get_many_const_ref<Ctx: CtxTrait, F, const N: usize>(
        &self,
        ctx: &Ctx,
        names: [Val; N],
        f: F,
    ) -> Val
    where
        F: FnOnce([Result<&Val, CtxError>; N]) -> Val,
        Self: Sized,
    {
        let vals = names.each_ref().map(|name| match name {
            Val::Symbol(s) => ctx.get_const_ref(s),
            val => Ok(val),
        });
        f(vals)
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

pub(crate) mod names;
