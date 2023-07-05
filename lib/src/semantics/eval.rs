use {
    crate::{
        semantics::{
            eval::strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultStrategy,
                },
                inline::InlineStrategy,
                interpolate::InterpolateStrategy,
                val::ValStrategy,
                ByRefStrategy,
                EvalStrategy,
            },
            val::{
                CtxVal,
                RefVal,
                Val,
            },
        },
        types::{
            Either,
            Keeper,
            Map,
            Owner,
            Reader,
        },
    },
    smartstring::alias::CompactString,
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::{
            Hash,
            Hasher,
        },
        mem::swap,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Func {
    pub(crate) func_trait: FuncTrait,
    pub(crate) func_impl: FuncImpl,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct FuncTrait {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FuncImpl {
    Primitive(Primitive),
    Composed(Composed),
}

#[derive(Debug, Clone)]
pub(crate) struct Primitive {
    pub(crate) id: Name,
    pub(crate) eval_mode: EvalMode,
    pub(crate) ctx_fn: PrimitiveCtxFn,
}

#[derive(Clone)]
pub(crate) enum PrimitiveCtxFn {
    Free(CtxFreeFn),
    Const(CtxConstFn),
    Aware(CtxAwareFn),
}

type CtxFreeFn = Reader<dyn Fn(Val) -> Val>;

type CtxConstFn = Reader<dyn Fn(&Ctx, Val) -> Val>;

type CtxAwareFn = Reader<dyn Fn(&mut Ctx, Val) -> Val>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Name,
    pub(crate) eval_mode: EvalMode,
    pub(crate) ctx_fn: ComposedCtxFn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ComposedCtxFn {
    Free,
    Const { caller_name: Name },
    Aware { caller_name: Name },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Value,
    Eval,
    Interpolate,
    Inline,
}

pub(crate) type Name = CompactString;

pub(crate) type NameMap = Map<Name, TaggedVal>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InvariantTag {
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) name_map: NameMap,
    pub(crate) super_ctx: Option<Either<Name, RefVal>>,
    pub(crate) reverse_interpreter: Option<Reader<Func>>,
}

impl Func {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.func_impl.eval(ctx, input)
    }
}

impl FuncImpl {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }
}

impl Primitive {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let val = self.eval_mode.eval(ctx, input);
        match &self.ctx_fn {
            PrimitiveCtxFn::Free(evaluator) => evaluator(val),
            PrimitiveCtxFn::Const(evaluator) => evaluator(ctx, val),
            PrimitiveCtxFn::Aware(evaluator) => evaluator(ctx, val),
        }
    }
}

impl Composed {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match &self.ctx_fn {
            ComposedCtxFn::Free => self.eval_common(ctx, None, input),
            ComposedCtxFn::Const { caller_name } => {
                self.eval_common(ctx, Some((caller_name, InvariantTag::Const)), input)
            }
            ComposedCtxFn::Aware { caller_name } => {
                self.eval_common(ctx, Some((caller_name, InvariantTag::Final)), input)
            }
        }
    }

    fn eval_common(
        &self,
        caller_ctx: &mut Ctx,
        caller_name_tag: Option<(&Name, InvariantTag)>,
        input: Val,
    ) -> Val {
        let mut new_ctx = self.ctx.clone();
        new_ctx.reverse_interpreter = caller_ctx.reverse_interpreter.clone();

        let input = self.eval_mode.eval(caller_ctx, input);
        new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(input));

        let Some((name, tag)) = caller_name_tag else {
            return DefaultByRefStrategy::eval(&mut new_ctx, &self.body);
        };

        let mut caller_ctx_owned = Ctx::default();
        swap(caller_ctx, &mut caller_ctx_owned);
        let keeper = Keeper::new(TaggedVal {
            val: Val::Ctx(Box::new(caller_ctx_owned).into()),
            tag,
        });
        let caller_ctx_ref = Val::Ref(RefVal(keeper.clone()));
        new_ctx.put_val_local(name.clone(), TaggedVal::new(caller_ctx_ref));

        let output = DefaultByRefStrategy::eval(&mut new_ctx, &self.body);

        if let Ok(o) = Keeper::owner(&keeper) {
            if let Val::Ctx(CtxVal(caller)) = Owner::move_data(o).val {
                *caller_ctx = *caller;
            }
        }

        output
    }
}

impl EvalMode {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Value => ValStrategy::eval(ctx, input),
            EvalMode::Eval => DefaultStrategy::eval(ctx, input),
            EvalMode::Interpolate => InterpolateStrategy::eval(ctx, input),
            EvalMode::Inline => InlineStrategy::eval(ctx, input),
        }
    }
}

impl Ctx {
    pub(crate) fn get(&self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return Val::default();
                };
                ctx.get(name)
            });
        };
        tagged_val.val.clone()
    }

    pub(crate) fn get_ref<T, F>(&self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<&Val>) -> T,
    {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return f(None);
                };
                ctx.get_ref(name, f)
            });
        };
        f(Some(&tagged_val.val))
    }

    pub(crate) fn get_mut<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<&mut Val>) -> T,
    {
        if self.name_map.get(name).is_none() {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.get_mut(name, f)
                } else {
                    f(None)
                }
            });
        }
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            return f(None);
        };

        if matches!(tagged_val.tag, InvariantTag::Const) {
            return f(None);
        }

        f(Some(&mut tagged_val.val))
    }

    pub(crate) fn remove(&mut self, name: &str) -> Val {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.remove(name)
                } else {
                    Val::default()
                }
            });
        };
        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.name_map
            .remove(name)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn put_val(&mut self, name: Name, val: TaggedVal) -> Val {
        let Some(tagged_val) = self.name_map.get(&name) else {
            return self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.put_val(name, val)
                } else {
                    ctx.put_unchecked(name, val)
                }
            });
        };

        if !matches!(&tagged_val.tag, InvariantTag::None) {
            return Val::default();
        }
        self.put_unchecked(name, val)
    }

    pub(crate) fn put_val_local(&mut self, name: Name, val: TaggedVal) -> Val {
        let (None | Some(TaggedVal { tag: InvariantTag::None, .. })) = self.name_map.get(&name) else {
            return Val::default();
        };
        self.put_unchecked(name, val)
    }

    fn put_unchecked(&mut self, name: Name, val: TaggedVal) -> Val {
        self.name_map
            .insert(name, val)
            .map(|tagged_val| tagged_val.val)
            .unwrap_or_default()
    }

    pub(crate) fn set_final(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.set_final(name);
                }
            });
            return;
        };
        if !(matches!(&tagged_val.tag, InvariantTag::None)) {
            return;
        }
        tagged_val.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const(&mut self, name: &str) {
        let Some(tagged_val) = self.name_map.get_mut(name) else {
            self.get_mut_super_ctx(|ctx, is_super| {
                if is_super {
                    ctx.set_const(name);
                }
            });
            return;
        };
        tagged_val.tag = InvariantTag::Const;
    }

    pub(crate) fn is_final(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return false;
                };
                ctx.is_final(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Final | InvariantTag::Const)
    }

    pub(crate) fn is_const(&self, name: &str) -> bool {
        let Some(tagged_val) = self.name_map.get(name) else {
            return self.get_ref_super_ctx(|op_ctx| {
                let Some(ctx) = op_ctx else {
                    return false;
                };
                ctx.is_const(name)
            });
        };
        matches!(&tagged_val.tag, InvariantTag::Const)
    }

    fn get_ref_super_ctx<T, F>(&self, f: F) -> T
    where
        F: FnOnce(Option<&Ctx>) -> T,
    {
        let Some(name_or_ref) = &self.super_ctx else {
            return f(None);
        };
        match name_or_ref {
            Either::Left(name) => {
                let Some(TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), .. }) = self.name_map.get(name) else {
                    return f(None);
                };
                f(Some(super_ctx))
            }
            Either::Right(RefVal(r)) => {
                let Ok(r) = Keeper::reader(r) else {
                    return f(None);
                };
                let TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), .. } = &*r else {
                    return f(None);
                };
                f(Some(super_ctx))
            }
        }
    }

    fn get_mut_super_ctx<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Ctx, bool) -> T,
    {
        let Some(name_or_ref) = &self.super_ctx else {
            return f(self, false);
        };
        match name_or_ref {
            Either::Left(name) => {
                let Some(TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), tag }) = self.name_map.get_mut(name) else {
                    return f(self, false);
                };
                if matches!(tag, InvariantTag::Const) {
                    return f(self, false);
                }
                f(super_ctx, true)
            }
            Either::Right(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(r) else {
                    return f(self, false);
                };
                let TaggedVal { val: Val::Ctx(CtxVal(super_ctx)), tag } = &mut *o else {
                    return f(self, false);
                };
                if matches!(tag, InvariantTag::Const) {
                    return f(self, false);
                }
                f(super_ctx, true)
            }
        }
    }

    pub(crate) fn get_ref_or_val<F>(&self, name: Val, f: F) -> Val
    where
        F: FnOnce(Either<&Val, Val>) -> Val,
    {
        match name {
            Val::Symbol(s) => self.get_ref(&s, |op_val| {
                let Some(val) = op_val else {
                    return Val::default();
                };
                if let Val::Ref(RefVal(r)) = val {
                    let Ok(r) = Keeper::reader(r) else {
                        return Val::default();
                    };
                    f(Either::Left(&r.val))
                } else {
                    f(Either::Left(val))
                }
            }),
            Val::Ref(RefVal(r)) => {
                let Ok(r) = Keeper::reader(&r) else {
                    return Val::default();
                };
                f(Either::Left(&r.val))
            }
            val => f(Either::Right(val)),
        }
    }

    pub(crate) fn get_mut_or_val<F>(&mut self, name: Val, f: F) -> Val
    where
        F: FnOnce(Either<&mut Val, Val>) -> Val,
    {
        match name {
            Val::Symbol(s) => self.get_mut(&s, |op_val| {
                let Some(val) = op_val else {
                    return Val::default();
                };
                if let Val::Ref(RefVal(r)) = val {
                    let Ok(mut o) = Keeper::owner(r) else {
                        return Val::default();
                    };
                    if matches!(o.tag, InvariantTag::Const) {
                        return Val::default();
                    }
                    f(Either::Left(&mut o.val))
                } else {
                    f(Either::Left(val))
                }
            }),
            Val::Ref(RefVal(r)) => {
                let Ok(mut o) = Keeper::owner(&r) else {
                    return Val::default();
                };
                if matches!(o.tag, InvariantTag::Const) {
                    return Val::default();
                }
                f(Either::Left(&mut o.val))
            }
            val => f(Either::Right(val)),
        }
    }
}

impl Debug for PrimitiveCtxFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveCtxFn::Free(evaluator) => {
                let ptr: *const dyn Fn(Val) -> Val = &**evaluator;
                f.debug_tuple("PrimitiveCtxFn::Free")
                    .field(&format!("{ptr:p}"))
                    .finish()
            }
            PrimitiveCtxFn::Const(evaluator) => {
                let ptr: *const dyn Fn(&Ctx, Val) -> Val = &**evaluator;
                f.debug_tuple("PrimitiveCtxFn::Const")
                    .field(&format!("{ptr:p}"))
                    .finish()
            }
            PrimitiveCtxFn::Aware(evaluator) => {
                let ptr: *const dyn Fn(&mut Ctx, Val) -> Val = &**evaluator;
                f.debug_tuple("PrimitiveCtxFn::Aware")
                    .field(&format!("{ptr:p}"))
                    .finish()
            }
        }
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Primitive {}

impl Hash for Primitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TaggedVal {
    pub(crate) fn new(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::None,
            val,
        }
    }
    pub(crate) fn new_final(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::Final,
            val,
        }
    }
    pub(crate) fn new_const(val: Val) -> TaggedVal {
        TaggedVal {
            tag: InvariantTag::Const,
            val,
        }
    }
}

impl Func {
    pub(crate) fn new_primitive(primitive: Primitive) -> Func {
        Func {
            func_trait: FuncTrait {},
            func_impl: FuncImpl::Primitive(primitive),
        }
    }

    pub(crate) fn new_composed(composed: Composed) -> Func {
        Func {
            func_trait: FuncTrait {},
            func_impl: FuncImpl::Composed(composed),
        }
    }
}

impl Primitive {
    pub(crate) fn new_ctx_free(
        id: &str,
        eval_mode: EvalMode,
        evaluator: impl Fn(Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Free(Reader::new(evaluator)),
            eval_mode,
        }
    }

    pub(crate) fn new_ctx_const(
        id: &str,
        eval_mode: EvalMode,
        evaluator: impl Fn(&Ctx, Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Const(Reader::new(evaluator)),
            eval_mode,
        }
    }

    pub(crate) fn new_ctx_aware(
        id: &str,
        eval_mode: EvalMode,
        evaluator: impl Fn(&mut Ctx, Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Aware(Reader::new(evaluator)),
            eval_mode,
        }
    }
}

pub(crate) mod strategy;
