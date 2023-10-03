use {
    crate::{
        semantics::val::{
            CtxVal,
            Val,
        },
        types::{
            Bool,
            Either,
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

pub(crate) trait CtxTrait {
    fn get(&self, name: &str) -> Val;

    fn remove(&mut self, name: &str) -> Val;

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val;

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val;

    fn set_final(&mut self, name: &str);

    fn set_const(&mut self, name: &str);

    fn is_final(&self, name: &str) -> Val;

    fn is_const(&self, name: &str) -> Val;

    fn is_null(&self, name: &str) -> Val;

    fn is_local(&self, name: &str) -> Val;

    fn set_super(&mut self, super_ctx: Option<Symbol>);

    fn get_tagged_ref(&mut self, name: &str) -> Option<TaggedRef<Val>>;

    fn get_const_ref(&self, name: &str) -> Option<&Val>;

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N]
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
    pub(crate) super_ctx: Option<Symbol>,
}

impl Ctx {
    pub(crate) fn get(&self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            let Some(super_ctx) = self.get_const_super_ctx() else {
                return Val::default();
            };
            return super_ctx.get(name);
        };
        tagged_val.val.clone()
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            let Either::Left(Some(TaggedRef {
                val_ref: ctx,
                is_const: false,
            })) = self.get_tagged_super_ctx()
            else {
                return Val::default();
            };
            return ctx.remove(name);
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        let Some(tagged_val) = self.name_map.get(&name) else {
            return match self.get_tagged_super_ctx() {
                Either::Right(self_ctx) => self_ctx.put_unchecked(name, val),
                Either::Left(Some(TaggedRef {
                    val_ref: ctx,
                    is_const,
                })) => {
                    if is_const {
                        return Val::default();
                    };
                    ctx.put_val(name, val)
                }
                _ => Val::default(),
            };
        };
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

    pub(crate) fn set_final(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            let Either::Left(Some(TaggedRef {
                val_ref: ctx,
                is_const: false,
            })) = self.get_tagged_super_ctx()
            else {
                return;
            };
            ctx.set_final(name);
            return;
        };
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return;
        }
        tagged_val.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            let Either::Left(Some(TaggedRef {
                val_ref: ctx,
                is_const: false,
            })) = self.get_tagged_super_ctx()
            else {
                return;
            };
            ctx.set_const(name);
            return;
        };
        tagged_val.tag = InvariantTag::Const;
    }

    pub(crate) fn is_final(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            let Some(super_ctx) = self.get_const_super_ctx() else {
                return false;
            };
            return super_ctx.is_final(name);
        };
        matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const)
    }

    pub(crate) fn is_const(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            let Some(super_ctx) = self.get_const_super_ctx() else {
                return false;
            };
            return super_ctx.is_const(name);
        };
        matches!(&tagged_val.tag, InvariantTag::Const)
    }

    pub(crate) fn is_local(&self, name: &str) -> bool {
        self.name_map.get(name).is_some()
    }

    pub(crate) fn get_tagged_ref(&mut self, is_const: bool, name: &str) -> Option<TaggedRef<Val>> {
        if self.name_map.get(name).is_none() {
            return match self.get_tagged_super_ctx() {
                Either::Left(Some(TaggedRef {
                    val_ref: ctx,
                    is_const: super_const,
                })) => ctx.get_tagged_ref(is_const || super_const, name),
                _ => None,
            };
        }
        let tagged_val = self.name_map.get_mut(name).unwrap();
        let is_const = matches!(tagged_val.tag, InvariantTag::Const) || is_const;
        Some(TaggedRef::new(&mut tagged_val.val, is_const))
    }

    pub(crate) fn get_const_ref(&self, name: &str) -> Option<&Val> {
        if self.name_map.get(name).is_none() {
            let super_ctx = self.get_const_super_ctx()?;
            return super_ctx.get_const_ref(name);
        }
        let tagged_val = self.name_map.get(name)?;
        Some(&tagged_val.val)
    }

    pub(crate) fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N] {
        match self.get_const_super_ctx() {
            None => names.map(|name| {
                let tagged_val = self.name_map.get(name)?;
                Some(&tagged_val.val)
            }),
            Some(super_ctx) => names.map(|name| {
                self.name_map
                    .get(name)
                    .map(|tagged_val| &tagged_val.val)
                    .or_else(|| super_ctx.get_const_ref(name))
            }),
        }
    }

    fn get_tagged_super_ctx(&mut self) -> Either<Option<TaggedRef<Ctx>>, &mut Ctx> {
        let Some(name) = &self.super_ctx else {
            return Either::Right(self);
        };
        if self.name_map.get(name).is_none() {
            return Either::Right(self);
        }
        let Some(TaggedVal {
            val: Val::Ctx(CtxVal(super_ctx)),
            tag,
        }) = self.name_map.get_mut(name)
        else {
            return Either::Left(None);
        };
        let is_const = matches!(tag, InvariantTag::Const);
        Either::Left(Some(TaggedRef::new(super_ctx, is_const)))
    }

    fn get_const_super_ctx(&self) -> Option<&Ctx> {
        let name = self.super_ctx.as_ref()?;
        let TaggedVal {
            val: Val::Ctx(CtxVal(super_ctx)),
            tag: _tag,
        } = self.name_map.get(name)?
        else {
            return None;
        };
        Some(super_ctx)
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
    pub(crate) fn get<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Val {
        let val = ctx.get_const_ref(name);
        val.cloned().unwrap_or_default()
    }

    pub(crate) fn is_null<Ctx: CtxTrait>(&self, ctx: &Ctx, name: &str) -> Val {
        let is_null = ctx.get_const_ref(name).is_none();
        Val::Bool(Bool::new(is_null))
    }

    pub(crate) fn get_ref_or_val<Ctx: CtxTrait, T, F>(&self, ctx: &mut Ctx, name: Val, f: F) -> T
    where
        T: Default,
        F: FnOnce(Either<TaggedRef<Val>, Val>) -> T,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Some(tagged_ref) = ctx.get_tagged_ref(&s) else {
                    return T::default();
                };
                f(Either::Left(tagged_ref))
            }
            val => f(Either::Right(val)),
        }
    }

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
                let Some(val) = ctx.get_const_ref(&s) else {
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

    pub(crate) fn get_const_ref_no_ret<Ctx: CtxTrait, F>(&self, ctx: &Ctx, name: Val, f: F) -> Val
    where
        F: FnOnce(&Val) -> Val,
        Self: Sized,
    {
        match name {
            Val::Symbol(s) => {
                let Some(val) = ctx.get_const_ref(&s) else {
                    return Val::default();
                };
                f(val)
            }
            val => f(&val),
        }
    }

    pub(crate) fn get_many_const_ref<Ctx: CtxTrait, F, const N: usize>(
        &self,
        ctx: &Ctx,
        names: [Val; N],
        f: F,
    ) -> Val
    where
        F: FnOnce([Option<&Val>; N]) -> Val,
        Self: Sized,
    {
        let vals = names.each_ref().map(|name| match name {
            Val::Symbol(s) => ctx.get_const_ref(s),
            val => Some(val),
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
