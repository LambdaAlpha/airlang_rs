use {
    crate::{
        semantics::val::{
            CtxVal,
            RefVal,
            Val,
        },
        types::{
            Bool,
            Either,
            Keeper,
            Map,
            Pair,
            Symbol,
        },
    },
    std::{
        fmt::Debug,
        hash::Hash,
    },
};

#[allow(clippy::wrong_self_convention)]
pub(crate) trait CtxTrait {
    fn get(&mut self, name: &str) -> Val;

    fn is_null(&mut self, name: &str) -> Val;

    fn remove(&mut self, name: &str) -> Val;

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val;

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val;

    fn set_final(&mut self, name: &str);

    fn set_const(&mut self, name: &str);

    fn is_final(&mut self, name: &str) -> Val;

    fn is_const(&mut self, name: &str) -> Val;

    fn set_super(&mut self, super_ctx: Option<Either<Symbol, RefVal>>);

    fn get_ref<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    pub(crate) super_ctx: Option<Either<Symbol, RefVal>>,
}

impl Ctx {
    pub(crate) fn get(&mut self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_super_ctx(true, |super_ctx, _| {
                let Some(TaggedRef { val_ref: ctx, .. }) = super_ctx else {
                    return Val::default();
                };
                ctx.get(name)
            });
        };
        tagged_val.val.clone()
    }

    pub(crate) fn remove(&mut self, is_const: bool, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_super_ctx(is_const, |ctx, _| {
                let Some(TaggedRef {
                    val_ref: ctx,
                    is_const: super_const,
                }) = ctx
                else {
                    return Val::default();
                };
                ctx.remove(super_const, name)
            });
        };
        if is_const {
            return Val::default();
        }
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, is_const: bool, name: Symbol, val: TaggedVal) -> Val {
        let Some(tagged_val) = self.name_map.get(&name) else {
            return self.get_super_ctx(is_const, |ctx, self_ctx| match ctx {
                None => {
                    if is_const {
                        return Val::default();
                    }
                    let Some(self_ctx) = self_ctx else {
                        return Val::default();
                    };
                    self_ctx.put_unchecked(name, val)
                }
                Some(ctx) => {
                    let TaggedRef {
                        val_ref: ctx,
                        is_const: super_const,
                    } = ctx;
                    ctx.put_val(super_const, name, val)
                }
            });
        };
        if is_const {
            return Val::default();
        }
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.put_unchecked(name, val)
    }

    pub(crate) fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        let (None
        | Some(TaggedVal {
            tag: InvariantTag::None,
            ..
        })) = self.name_map.get(&name)
        else {
            return Val::default();
        };
        self.put_unchecked(name, val)
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.name_map
            .insert(name, val)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn set_final(&mut self, is_const: bool, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_super_ctx(is_const, |ctx, _| {
                let Some(TaggedRef {
                    val_ref: ctx,
                    is_const: super_const,
                }) = ctx
                else {
                    return;
                };
                ctx.set_final(super_const, name);
            });
            return;
        };
        if is_const {
            return;
        }
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return;
        }
        tagged_val.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const(&mut self, is_const: bool, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_super_ctx(is_const, |ctx, _| {
                let Some(TaggedRef {
                    val_ref: ctx,
                    is_const: super_const,
                }) = ctx
                else {
                    return;
                };
                ctx.set_const(super_const, name);
            });
            return;
        };
        if is_const {
            return;
        }
        tagged_val.tag = InvariantTag::Const;
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn is_final(&mut self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_super_ctx(true, |ctx, _| {
                let Some(TaggedRef { val_ref: ctx, .. }) = ctx else {
                    return false;
                };
                ctx.is_final(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const)
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn is_const(&mut self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_super_ctx(true, |ctx, _| {
                let Some(TaggedRef { val_ref: ctx, .. }) = ctx else {
                    return false;
                };
                ctx.is_const(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Const)
    }

    pub(crate) fn get_ref<T, F>(&mut self, is_const: bool, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>, Option<&mut Ctx>) -> T,
    {
        if self.name_map.get(name).is_none() {
            return self.get_super_ctx(is_const, |ctx, self_ctx| {
                let Some(TaggedRef {
                    val_ref: ctx,
                    is_const: super_const,
                }) = ctx
                else {
                    return f(None, self_ctx);
                };
                ctx.get_ref(super_const, name, f)
            });
        }
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return f(None, Some(self));
        };
        let is_const = matches!(tagged_val.tag, InvariantTag::Const) || is_const;
        f(Some(TaggedRef::new(&mut tagged_val.val, is_const)), None)
    }

    #[allow(unused)]
    pub(crate) fn get_ref_or_default<T, F>(&mut self, is_const: bool, name: &str, f: F) -> T
    where
        T: Default,
        F: FnOnce(TaggedRef<Val>) -> T,
    {
        self.get_ref(is_const, name, |val, _| {
            let Some(val) = val else {
                return T::default();
            };
            f(val)
        })
    }

    fn get_super_ctx<T, F>(&mut self, is_const: bool, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Ctx>>, Option<&mut Ctx>) -> T,
    {
        let Some(name_or_ref) = &self.super_ctx else {
            return f(None, Some(self));
        };
        match name_or_ref {
            Either::Left(name) => {
                let Some(TaggedVal {
                    val: Val::Ctx(CtxVal(super_ctx)),
                    tag,
                }) = self.name_map.get_mut(name)
                else {
                    return f(None, Some(self));
                };
                let is_const = matches!(tag, InvariantTag::Const) || is_const;
                f(Some(TaggedRef::new(super_ctx, is_const)), None)
            }
            Either::Right(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(r) else {
                    return f(None, Some(self));
                };
                let TaggedVal {
                    val: Val::Ctx(CtxVal(super_ctx)),
                    tag,
                } = &mut *o
                else {
                    return f(None, Some(self));
                };
                let is_const = matches!(tag, InvariantTag::Const);
                f(Some(TaggedRef::new(super_ctx, is_const)), Some(self))
            }
        }
    }
}

pub(crate) struct DefaultCtx;

impl DefaultCtx {
    #[allow(unused)]
    pub(crate) fn get<Ctx: CtxTrait>(&self, ctx: &mut Ctx, name: &str) -> Val {
        ctx.get_ref(name, |val| {
            val.map(|val| val.val_ref.clone()).unwrap_or_default()
        })
    }

    pub(crate) fn is_null<Ctx: CtxTrait>(&self, ctx: &mut Ctx, name: &str) -> Val {
        let is_null = ctx.get_ref(name, |val| val.is_none());
        Val::Bool(Bool::new(is_null))
    }

    #[allow(unused)]
    pub(crate) fn get_ref_or_default<Ctx: CtxTrait, T, F>(
        &self,
        ctx: &mut Ctx,
        name: &str,
        f: F,
    ) -> T
    where
        T: Default,
        F: FnOnce(TaggedRef<Val>) -> T,
    {
        ctx.get_ref(name, |val| {
            let Some(val) = val else {
                return T::default();
            };
            f(val)
        })
    }

    pub(crate) fn get_ref_or_val<Ctx: CtxTrait, T, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> T
    where
        F: FnOnce(Either<TaggedRef<Val>, Option<Val>>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => ctx.get_ref(&s, |ref_or_val| {
                let Some(TaggedRef {
                    val_ref: val,
                    is_const: val_const,
                }) = ref_or_val
                else {
                    return f(Either::Right(None));
                };
                let Val::Ref(RefVal(r)) = val else {
                    return f(Either::Left(TaggedRef::new(val, val_const)));
                };
                let Ok(mut o) = Keeper::owner(r) else {
                    return f(Either::Right(None));
                };
                let is_const = matches!(o.tag, InvariantTag::Const);
                f(Either::Left(TaggedRef::new(&mut o.val, is_const)))
            }),
            Val::Ref(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(&r) else {
                    return f(Either::Right(None));
                };
                let is_const = matches!(o.tag, InvariantTag::Const);
                f(Either::Left(TaggedRef::new(&mut o.val, is_const)))
            }
            val => f(Either::Right(Some(val))),
        }
    }

    pub(crate) fn get_ref_val_or_default<Ctx: CtxTrait, T, F>(
        &self,
        ctx: &mut Ctx,
        name: Val,
        f: F,
    ) -> T
    where
        T: Default,
        F: FnOnce(Either<TaggedRef<Val>, Val>) -> T,
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => f(Either::Left(tagged_ref)),
            Either::Right(Some(val)) => f(Either::Right(val)),
            _ => T::default(),
        })
    }

    pub(crate) fn get_tagged_ref<Ctx: CtxTrait, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(TaggedRef<Val>) -> Val,
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => f(tagged_ref),
            Either::Right(Some(mut val)) => {
                let tagged_ref = TaggedRef::new(&mut val, false);
                let result = f(tagged_ref);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
            _ => Val::default(),
        })
    }

    pub(crate) fn get_const_ref<Ctx: CtxTrait, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&Val) -> Val,
        Self: Sized,
    {
        self.get_ref_or_val(ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(tagged_ref) => f(tagged_ref.val_ref),
            Either::Right(Some(val)) => {
                let result = f(&val);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
            _ => Val::default(),
        })
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
            Either::Right(Some(mut val)) => {
                let result = f(&mut val);
                Val::Pair(Box::new(Pair::new(val, result)))
            }
            _ => Val::default(),
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
            Either::Right(Some(mut val)) => {
                f(&mut val);
                val
            }
            _ => Val::default(),
        })
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
