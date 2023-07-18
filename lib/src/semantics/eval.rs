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
        fmt::Debug,
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
    input_eval_mode: EvalMode,
    evaluator: FuncEval,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FuncEval {
    Free(CtxFreeEval),
    Const(CtxConstEval),
    Mutable(CtxMutableEval),
}

type CtxFreeEval = FuncImpl<Primitive<CtxFreeFn>, Composed<CtxFreeInfo>>;

type CtxConstEval = FuncImpl<Primitive<CtxConstFn>, Composed<CtxConstInfo>>;

type CtxMutableEval = FuncImpl<Primitive<CtxMutableFn>, Composed<CtxMutableInfo>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composed(C),
}

#[derive(Debug, Clone)]
pub(crate) struct Primitive<F> {
    id: Name,
    eval_fn: F,
}

pub(crate) type IsConst = bool;

pub(crate) type CtxFreeFn = Reader<dyn Fn(Val) -> Val>;

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub(crate) type CtxConstFn = Reader<dyn Fn(&mut Ctx, Val) -> Val>;

pub(crate) type CtxMutableFn = Reader<dyn Fn(&mut Ctx, IsConst, Val) -> Val>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed<C> {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Name,
    pub(crate) caller: C,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxFreeInfo {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxConstInfo {
    pub(crate) name: Name,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxMutableInfo {
    pub(crate) name: Name,
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
    pub(crate) fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        let input = self.input_eval_mode.eval(ctx, input);
        self.evaluator.eval_mutable(ctx, input)
    }

    pub(crate) fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        let input = self.input_eval_mode.eval_const(ctx, input);
        self.evaluator.eval_const(ctx, input)
    }

    pub(crate) fn eval_free(&self, input: Val) -> Val {
        let mut ctx = Ctx::default();
        let input = self.input_eval_mode.eval(&mut ctx, input);
        self.evaluator.eval_free(ctx, input)
    }
}

pub(crate) trait FuncEvalTrait {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val;
    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val;
    fn eval_free(&self, ctx: Ctx, input: Val) -> Val;
}

impl FuncEvalTrait for FuncEval {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncEval::Free(func) => func.eval_mutable(ctx, input),
            FuncEval::Const(func) => func.eval_mutable(ctx, input),
            FuncEval::Mutable(func) => func.eval_mutable(ctx, input),
        }
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncEval::Free(func) => func.eval_const(ctx, input),
            FuncEval::Const(func) => func.eval_const(ctx, input),
            FuncEval::Mutable(func) => func.eval_const(ctx, input),
        }
    }

    fn eval_free(&self, ctx: Ctx, input: Val) -> Val {
        match self {
            FuncEval::Free(func) => func.eval_free(ctx, input),
            FuncEval::Const(func) => func.eval_free(ctx, input),
            FuncEval::Mutable(func) => func.eval_free(ctx, input),
        }
    }
}

impl<P: FuncEvalTrait, C: FuncEvalTrait> FuncEvalTrait for FuncImpl<P, C> {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval_mutable(ctx, input),
            FuncImpl::Composed(c) => c.eval_mutable(ctx, input),
        }
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval_const(ctx, input),
            FuncImpl::Composed(c) => c.eval_const(ctx, input),
        }
    }

    fn eval_free(&self, ctx: Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval_free(ctx, input),
            FuncImpl::Composed(c) => c.eval_free(ctx, input),
        }
    }
}

impl FuncEvalTrait for Primitive<CtxFreeFn> {
    fn eval_mutable(&self, _: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(input)
    }

    fn eval_const(&self, _: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(input)
    }

    fn eval_free(&self, _: Ctx, input: Val) -> Val {
        (self.eval_fn)(input)
    }
}

impl FuncEvalTrait for Composed<CtxFreeInfo> {
    fn eval_mutable(&self, _: &mut Ctx, input: Val) -> Val {
        eval_free_in_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }

    fn eval_const(&self, _: &mut Ctx, input: Val) -> Val {
        eval_free_in_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }

    fn eval_free(&self, _: Ctx, input: Val) -> Val {
        eval_free_in_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }
}

impl FuncEvalTrait for Primitive<CtxConstFn> {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx, input)
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx, input)
    }

    fn eval_free(&self, mut ctx: Ctx, input: Val) -> Val {
        (self.eval_fn)(&mut ctx, input)
    }
}

impl FuncEvalTrait for Composed<CtxConstInfo> {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        eval_aware_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Const,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        eval_aware_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Const,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }

    fn eval_free(&self, ctx: Ctx, input: Val) -> Val {
        eval_free_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Const,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }
}

impl FuncEvalTrait for Primitive<CtxMutableFn> {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx, false, input)
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx, true, input)
    }

    fn eval_free(&self, mut ctx: Ctx, input: Val) -> Val {
        (self.eval_fn)(&mut ctx, false, input)
    }
}

impl FuncEvalTrait for Composed<CtxMutableInfo> {
    fn eval_mutable(&self, ctx: &mut Ctx, input: Val) -> Val {
        eval_aware_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Final,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }

    fn eval_const(&self, ctx: &mut Ctx, input: Val) -> Val {
        eval_aware_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Const,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }

    fn eval_free(&self, ctx: Ctx, input: Val) -> Val {
        eval_free_in_aware(
            self.ctx.clone(),
            ctx,
            self.caller.name.clone(),
            InvariantTag::Final,
            input,
            self.input_name.clone(),
            &self.body,
        )
    }
}

fn eval_free_in_free(mut new_ctx: Ctx, input: Val, input_name: Name, body: &Val) -> Val {
    new_ctx.put_val_local(input_name, TaggedVal::new(input));
    DefaultByRefStrategy::eval(&mut new_ctx, body)
}

fn eval_free_in_aware(
    mut new_ctx: Ctx,
    caller: Ctx,
    caller_name: Name,
    caller_tag: InvariantTag,
    input: Val,
    input_name: Name,
    body: &Val,
) -> Val {
    new_ctx.put_val_local(input_name, TaggedVal::new(input));
    put_ctx(&mut new_ctx, caller, caller_name, caller_tag);
    DefaultByRefStrategy::eval(&mut new_ctx, body)
}

fn eval_aware_in_aware(
    mut new_ctx: Ctx,
    caller: &mut Ctx,
    caller_name: Name,
    caller_tag: InvariantTag,
    input: Val,
    input_name: Name,
    body: &Val,
) -> Val {
    new_ctx.put_val_local(input_name, TaggedVal::new(input));
    keep_eval_restore(new_ctx, caller, caller_name, caller_tag, body)
}

fn keep_eval_restore(
    mut new_ctx: Ctx,
    ctx: &mut Ctx,
    caller_name: Name,
    caller_tag: InvariantTag,
    body: &Val,
) -> Val {
    let caller = own_ctx(ctx);
    let keeper = keep_ctx(&mut new_ctx, caller, caller_name, caller_tag);
    let output = DefaultByRefStrategy::eval(&mut new_ctx, body);
    restore_ctx(ctx, &keeper);
    output
}

// here is why we need a `&mut Ctx` for a const eval
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn put_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Name, tag: InvariantTag) {
    let keeper = Keeper::new(TaggedVal {
        val: Val::Ctx(Box::new(ctx).into()),
        tag,
    });
    let caller_ref = Val::Ref(RefVal(keeper));
    new_ctx.put_val_local(name, TaggedVal::new(caller_ref));
}

fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Name, tag: InvariantTag) -> Keeper<TaggedVal> {
    let keeper = Keeper::new(TaggedVal {
        val: Val::Ctx(Box::new(ctx).into()),
        tag,
    });
    let caller_ref = Val::Ref(RefVal(keeper.clone()));
    new_ctx.put_val_local(name, TaggedVal::new(caller_ref));
    keeper
}

fn restore_ctx(ctx: &mut Ctx, keeper: &Keeper<TaggedVal>) {
    if let Ok(o) = Keeper::owner(keeper) {
        if let Val::Ctx(CtxVal(c)) = Owner::move_data(o).val {
            *ctx = *c;
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

impl<F> PartialEq for Primitive<F> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<F> Eq for Primitive<F> {}

impl<F> Hash for Primitive<F> {
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
    pub(crate) fn new(input_eval_mode: EvalMode, evaluator: FuncEval) -> Self {
        Func {
            input_eval_mode,
            evaluator,
        }
    }
}

impl<C> Primitive<C> {
    pub(crate) fn get_id(&self) -> &Name {
        &self.id
    }
}

impl Primitive<CtxFreeFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(Val) -> Val + 'static) -> Self {
        Primitive {
            id: Name::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }
}

impl Primitive<CtxConstFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(&mut Ctx, Val) -> Val + 'static) -> Self {
        Primitive {
            id: Name::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }
}

impl Primitive<CtxMutableFn> {
    pub(crate) fn new(
        id: &str,
        evaluator: impl Fn(&mut Ctx, IsConst, Val) -> Val + 'static,
    ) -> Self {
        Primitive {
            id: Name::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    pub(crate) fn new_dispatch(
        id: &str,
        evaluator_const: impl Fn(&mut Ctx, Val) -> Val + 'static,
        evaluator_mutable: impl Fn(&mut Ctx, Val) -> Val + 'static,
    ) -> Self {
        let evaluator = Self::dispatch(evaluator_const, evaluator_mutable);
        Primitive {
            id: Name::from(id),
            eval_fn: Reader::new(evaluator),
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
