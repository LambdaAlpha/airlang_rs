use {
    crate::{
        semantics::{
            eval::{
                ctx::{
                    Ctx,
                    InvariantTag,
                    TaggedVal,
                },
                strategy::{
                    eval::{
                        DefaultByRefStrategy,
                        DefaultConstStrategy,
                        DefaultFreeStrategy,
                        DefaultStrategy,
                    },
                    inline::{
                        InlineConstStrategy,
                        InlineFreeStrategy,
                        InlineStrategy,
                    },
                    interpolate::{
                        InterpolateConstStrategy,
                        InterpolateFreeStrategy,
                        InterpolateStrategy,
                    },
                    val::{
                        ValFreeStrategy,
                        ValStrategy,
                    },
                    ByRefStrategy,
                    EvalStrategy,
                    FreeStrategy,
                },
            },
            val::{
                CtxVal,
                RefVal,
                Val,
            },
        },
        types::{
            Keeper,
            Owner,
            Pair,
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

pub(crate) type Name = CompactString;

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
    Mutable(CtxMutableFn),
}

pub(crate) type IsConst = bool;

type CtxFreeFn = Reader<dyn Fn(Val) -> Val>;

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
type CtxConstFn = Reader<dyn Fn(&mut Ctx, Val) -> Val>;

type CtxMutableFn = Reader<dyn Fn(&mut Ctx, IsConst, Val) -> Val>;

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
    Mutable { caller_name: Name },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum BasicEvalMode {
    Value,
    Eval,
    Interpolate,
    Inline,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Basic(BasicEvalMode),
    Pair {
        first: BasicEvalMode,
        second: BasicEvalMode,
        non_pair: BasicEvalMode,
    },
}

impl Func {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.func_impl.eval(ctx, input)
    }

    pub(crate) fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.func_impl.eval_const(ctx, input)
    }

    pub(crate) fn eval_free(&self, input: Val) -> Val {
        self.func_impl.eval_free(input)
    }
}

impl FuncImpl {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }

    pub(crate) fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval_const(ctx, input),
            FuncImpl::Composed(c) => c.eval_const(ctx, input),
        }
    }

    pub(crate) fn eval_free(&self, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval_free(input),
            FuncImpl::Composed(c) => c.eval_free(input),
        }
    }
}

impl Primitive {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let val = self.eval_mode.eval(ctx, input);
        match &self.ctx_fn {
            PrimitiveCtxFn::Free(evaluator) => evaluator(val),
            PrimitiveCtxFn::Const(evaluator) => evaluator(ctx, val),
            PrimitiveCtxFn::Mutable(evaluator) => evaluator(ctx, false, val),
        }
    }

    pub(crate) fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        let val = self.eval_mode.eval_const(ctx, input);
        match &self.ctx_fn {
            PrimitiveCtxFn::Free(evaluator) => evaluator(val),
            PrimitiveCtxFn::Const(evaluator) => evaluator(ctx, val),
            PrimitiveCtxFn::Mutable(evaluator) => evaluator(ctx, true, val),
        }
    }

    pub(crate) fn eval_free(&self, input: Val) -> Val {
        let mut ctx = Ctx::default();
        let val = self.eval_mode.eval(&mut ctx, input);
        match &self.ctx_fn {
            PrimitiveCtxFn::Free(evaluator) => evaluator(val),
            PrimitiveCtxFn::Const(evaluator) => evaluator(&mut ctx, val),
            PrimitiveCtxFn::Mutable(evaluator) => evaluator(&mut ctx, false, val),
        }
    }
}

impl Composed {
    pub(crate) fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let mut new_ctx = self.ctx.clone();
        let input = self.eval_mode.eval(ctx, input);
        new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(input));

        match &self.ctx_fn {
            ComposedCtxFn::Free => DefaultByRefStrategy::eval(&mut new_ctx, &self.body),
            ComposedCtxFn::Const { caller_name } => {
                self.eval_ctx_aware(new_ctx, ctx, caller_name, InvariantTag::Const)
            }
            ComposedCtxFn::Mutable { caller_name } => {
                self.eval_ctx_aware(new_ctx, ctx, caller_name, InvariantTag::Final)
            }
        }
    }

    pub(crate) fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        let mut new_ctx = self.ctx.clone();
        let input = self.eval_mode.eval_const(ctx, input);
        new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(input));

        match &self.ctx_fn {
            ComposedCtxFn::Free => DefaultByRefStrategy::eval(&mut new_ctx, &self.body),
            ComposedCtxFn::Const { caller_name } => {
                self.eval_ctx_aware(new_ctx, ctx, caller_name, InvariantTag::Const)
            }
            ComposedCtxFn::Mutable { caller_name } => {
                self.eval_ctx_aware(new_ctx, ctx, caller_name, InvariantTag::Const)
            }
        }
    }

    pub(crate) fn eval_free(&self, input: Val) -> Val {
        let mut ctx = Ctx::default();
        let mut new_ctx = self.ctx.clone();
        let input = self.eval_mode.eval(&mut ctx, input);
        new_ctx.put_val_local(self.input_name.clone(), TaggedVal::new(input));

        match &self.ctx_fn {
            ComposedCtxFn::Free => {}
            ComposedCtxFn::Const { caller_name } => {
                Self::put_ctx(&mut new_ctx, ctx, caller_name, InvariantTag::Const);
            }
            ComposedCtxFn::Mutable { caller_name } => {
                Self::put_ctx(&mut new_ctx, ctx, caller_name, InvariantTag::Final);
            }
        }
        DefaultByRefStrategy::eval(&mut new_ctx, &self.body)
    }

    fn eval_ctx_aware(
        &self,
        mut new_ctx: Ctx,
        ctx: &mut Ctx,
        caller_name: &Name,
        tag: InvariantTag,
    ) -> Val {
        let caller = Self::own_ctx(ctx);
        let keeper = Self::keep_ctx(&mut new_ctx, caller, caller_name, tag);
        let output = DefaultByRefStrategy::eval(&mut new_ctx, &self.body);
        Self::restore_ctx(ctx, &keeper);
        output
    }

    // here is why we need a `&mut Ctx` for a const eval
    fn own_ctx(ctx: &mut Ctx) -> Ctx {
        let mut owned = Ctx::default();
        swap(ctx, &mut owned);
        owned
    }

    fn put_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: &Name, tag: InvariantTag) {
        let keeper = Keeper::new(TaggedVal {
            val: Val::Ctx(Box::new(ctx).into()),
            tag,
        });
        let caller_ref = Val::Ref(RefVal(keeper));
        new_ctx.put_val_local(name.clone(), TaggedVal::new(caller_ref));
    }

    fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: &Name, tag: InvariantTag) -> Keeper<TaggedVal> {
        let keeper = Keeper::new(TaggedVal {
            val: Val::Ctx(Box::new(ctx).into()),
            tag,
        });
        let caller_ref = Val::Ref(RefVal(keeper.clone()));
        new_ctx.put_val_local(name.clone(), TaggedVal::new(caller_ref));
        keeper
    }

    fn restore_ctx(ctx: &mut Ctx, keeper: &Keeper<TaggedVal>) {
        if let Ok(o) = Keeper::owner(keeper) {
            if let Val::Ctx(CtxVal(c)) = Owner::move_data(o).val {
                *ctx = *c;
            }
        }
    }
}

impl BasicEvalMode {
    fn eval_free(&self, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => ValFreeStrategy::eval(input),
            BasicEvalMode::Eval => DefaultFreeStrategy::eval(input),
            BasicEvalMode::Interpolate => InterpolateFreeStrategy::eval(input),
            BasicEvalMode::Inline => InlineFreeStrategy::eval(input),
        }
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => ValStrategy::eval(ctx, input),
            BasicEvalMode::Eval => DefaultConstStrategy::eval(ctx, input),
            BasicEvalMode::Interpolate => InterpolateConstStrategy::eval(ctx, input),
            BasicEvalMode::Inline => InlineConstStrategy::eval(ctx, input),
        }
    }

    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => ValStrategy::eval(ctx, input),
            BasicEvalMode::Eval => DefaultStrategy::eval(ctx, input),
            BasicEvalMode::Interpolate => InterpolateStrategy::eval(ctx, input),
            BasicEvalMode::Inline => InlineStrategy::eval(ctx, input),
        }
    }
}

impl EvalMode {
    #[allow(unused)]
    fn eval_free(&self, input: Val) -> Val {
        match self {
            EvalMode::Basic(eval_mode) => eval_mode.eval_free(input),
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first = first.eval_free(pair.first);
                    let second = second.eval_free(pair.second);
                    let pair = Pair::new(first, second);
                    Val::Pair(Box::new(pair))
                }
                input => non_pair.eval_free(input),
            },
        }
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Basic(eval_mode) => eval_mode.eval_const(ctx, input),
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first = first.eval_const(ctx, pair.first);
                    let second = second.eval_const(ctx, pair.second);
                    let pair = Pair::new(first, second);
                    Val::Pair(Box::new(pair))
                }
                input => non_pair.eval_const(ctx, input),
            },
        }
    }

    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Basic(eval_mode) => eval_mode.eval(ctx, input),
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first = first.eval(ctx, pair.first);
                    let second = second.eval(ctx, pair.second);
                    let pair = Pair::new(first, second);
                    Val::Pair(Box::new(pair))
                }
                input => non_pair.eval(ctx, input),
            },
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
                let ptr: *const dyn Fn(&mut Ctx, Val) -> Val = &**evaluator;
                f.debug_tuple("PrimitiveCtxFn::Const")
                    .field(&format!("{ptr:p}"))
                    .finish()
            }
            PrimitiveCtxFn::Mutable(evaluator) => {
                let ptr: *const dyn Fn(&mut Ctx, IsConst, Val) -> Val = &**evaluator;
                f.debug_tuple("PrimitiveCtxFn::Mutable")
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
        evaluator: impl Fn(&mut Ctx, Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Const(Reader::new(evaluator)),
            eval_mode,
        }
    }

    pub(crate) fn new_ctx_mutable(
        id: &str,
        eval_mode: EvalMode,
        evaluator: impl Fn(&mut Ctx, IsConst, Val) -> Val + 'static,
    ) -> Primitive {
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Mutable(Reader::new(evaluator)),
            eval_mode,
        }
    }

    pub(crate) fn new_ctx_mutable_dispatch(
        id: &str,
        eval_mode: EvalMode,
        evaluator_const: impl Fn(&mut Ctx, Val) -> Val + 'static,
        evaluator_mutable: impl Fn(&mut Ctx, Val) -> Val + 'static,
    ) -> Primitive {
        let evaluator = Self::dispatch(evaluator_const, evaluator_mutable);
        Primitive {
            id: Name::from(id),
            ctx_fn: PrimitiveCtxFn::Mutable(Reader::new(evaluator)),
            eval_mode,
        }
    }

    fn dispatch<Const, Mutable>(
        c: Const,
        m: Mutable,
    ) -> impl Fn(&mut Ctx, IsConst, Val) -> Val + 'static
    where
        Const: Fn(&mut Ctx, Val) -> Val + 'static,
        Mutable: Fn(&mut Ctx, Val) -> Val + 'static,
    {
        move |ctx, is_const, input| {
            if is_const {
                c(ctx, input)
            } else {
                m(ctx, input)
            }
        }
    }
}

pub(crate) mod strategy;

pub(crate) mod ctx;

pub(crate) mod ctx_free;
