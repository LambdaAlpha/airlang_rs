use {
    crate::{
        semantics::{
            eval::{
                ctx::{
                    constant::{
                        ConstCtx,
                        CtxForConstFn,
                    },
                    free::FreeCtx,
                    mutable::{
                        CtxForMutableFn,
                        MutableCtx,
                    },
                    Ctx,
                    CtxTrait,
                    InvariantTag,
                    TaggedVal,
                },
                strategy::{
                    eval::{
                        DefaultByRefStrategy,
                        DefaultStrategy,
                    },
                    inline::InlineStrategy,
                    interpolate::InterpolateStrategy,
                    val::ValStrategy,
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
            Symbol,
        },
    },
    std::{
        fmt::Debug,
        hash::{
            Hash,
            Hasher,
        },
        mem::swap,
    },
};

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
    id: Symbol,
    eval_fn: F,
}

pub(crate) type CtxFreeFn = Reader<dyn Fn(Val) -> Val>;

pub(crate) type CtxConstFn = Reader<dyn Fn(CtxForConstFn, Val) -> Val>;

pub(crate) type CtxMutableFn = Reader<dyn Fn(CtxForMutableFn, Val) -> Val>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed<C> {
    pub(crate) body: Val,
    pub(crate) ctx: Ctx,
    pub(crate) input_name: Symbol,
    pub(crate) caller: C,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxFreeInfo {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxConstInfo {
    pub(crate) name: Symbol,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxMutableInfo {
    pub(crate) name: Symbol,
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

pub(crate) trait Evaluator<Ctx, Input, Output> {
    fn eval(&self, ctx: &mut Ctx, input: Input) -> Output;
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Func
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        let input = self.input_eval_mode.eval(ctx, input);
        self.evaluator.eval(ctx, input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for FuncEval
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncEval::Free(func) => func.eval(ctx, input),
            FuncEval::Const(func) => func.eval(ctx, input),
            FuncEval::Mutable(func) => func.eval(ctx, input),
        }
    }
}

impl<Ctx, P: Evaluator<Ctx, Val, Val>, C: Evaluator<Ctx, Val, Val>> Evaluator<Ctx, Val, Val>
    for FuncImpl<P, C>
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.eval(ctx, input),
            FuncImpl::Composed(c) => c.eval(ctx, input),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<CtxFreeFn>
where
    Ctx: CtxTrait,
{
    fn eval(&self, _: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxFreeInfo>
where
    Ctx: CtxTrait,
{
    fn eval(&self, _: &mut Ctx, input: Val) -> Val {
        eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<CtxConstFn>
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx.for_const_fn(), input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxConstInfo>
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match ctx.for_const_fn() {
            CtxForConstFn::Free => {
                eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
            }
            CtxForConstFn::Const(ctx) => eval_aware(
                self.ctx.clone(),
                ctx,
                self.caller.name.clone(),
                InvariantTag::Const,
                input,
                self.input_name.clone(),
                &self.body,
            ),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<CtxMutableFn>
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx.for_mutable_fn(), input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxMutableInfo>
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match ctx.for_mutable_fn() {
            CtxForMutableFn::Free => {
                eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
            }
            CtxForMutableFn::Const(ctx) => eval_aware(
                self.ctx.clone(),
                ctx,
                self.caller.name.clone(),
                InvariantTag::Const,
                input,
                self.input_name.clone(),
                &self.body,
            ),
            CtxForMutableFn::Mutable(ctx) => eval_aware(
                self.ctx.clone(),
                ctx,
                self.caller.name.clone(),
                InvariantTag::Final,
                input,
                self.input_name.clone(),
                &self.body,
            ),
        }
    }
}

fn eval_free(mut new_ctx: Ctx, input: Val, input_name: Symbol, body: &Val) -> Val {
    new_ctx.put_val_local(input_name, TaggedVal::new(input));
    DefaultByRefStrategy.eval(&mut MutableCtx(&mut new_ctx), body)
}

fn eval_aware(
    mut new_ctx: Ctx,
    caller: &mut Ctx,
    caller_name: Symbol,
    caller_tag: InvariantTag,
    input: Val,
    input_name: Symbol,
    body: &Val,
) -> Val {
    new_ctx.put_val_local(input_name, TaggedVal::new(input));
    keep_eval_restore(new_ctx, caller, caller_name, caller_tag, body)
}

fn keep_eval_restore(
    mut new_ctx: Ctx,
    ctx: &mut Ctx,
    caller_name: Symbol,
    caller_tag: InvariantTag,
    body: &Val,
) -> Val {
    let caller = own_ctx(ctx);
    let keeper = keep_ctx(&mut new_ctx, caller, caller_name, caller_tag);
    let output = DefaultByRefStrategy.eval(&mut MutableCtx(&mut new_ctx), body);
    restore_ctx(ctx, &keeper);
    output
}

// here is why we need a `&mut Ctx` for a const eval
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Symbol, tag: InvariantTag) -> Keeper<TaggedVal> {
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

impl<Ctx> Evaluator<Ctx, Val, Val> for BasicEvalMode
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => ValStrategy.eval(ctx, input),
            BasicEvalMode::Eval => DefaultStrategy.eval(ctx, input),
            BasicEvalMode::Interpolate => InterpolateStrategy.eval(ctx, input),
            BasicEvalMode::Inline => InlineStrategy.eval(ctx, input),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxTrait,
{
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
    pub(crate) fn get_id(&self) -> &Symbol {
        &self.id
    }
}

impl Primitive<CtxFreeFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(Val) -> Val + 'static) -> Self {
        Primitive {
            id: Symbol::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }
}

impl Primitive<CtxConstFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(CtxForConstFn, Val) -> Val + 'static) -> Self {
        Primitive {
            id: Symbol::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    #[allow(unused)]
    pub(crate) fn new_dispatch(
        id: &str,
        free_evaluator: impl Fn(FreeCtx, Val) -> Val + 'static,
        const_evaluator: impl Fn(ConstCtx, Val) -> Val + 'static,
    ) -> Self {
        let evaluator = Self::dispatch(free_evaluator, const_evaluator);
        Primitive {
            id: Symbol::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    fn dispatch<Free, Const>(f: Free, c: Const) -> impl Fn(CtxForConstFn, Val) -> Val + 'static
    where
        Free: Fn(FreeCtx, Val) -> Val + 'static,
        Const: Fn(ConstCtx, Val) -> Val + 'static,
    {
        move |ctx, val| match ctx {
            CtxForConstFn::Free => f(FreeCtx, val),
            CtxForConstFn::Const(ctx) => c(ConstCtx(ctx), val),
        }
    }
}

impl Primitive<CtxMutableFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(CtxForMutableFn, Val) -> Val + 'static) -> Self {
        Primitive {
            id: Symbol::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    pub(crate) fn new_dispatch(
        id: &str,
        free_evaluator: impl Fn(FreeCtx, Val) -> Val + 'static,
        const_evaluator: impl Fn(ConstCtx, Val) -> Val + 'static,
        mutable_evaluator: impl Fn(MutableCtx, Val) -> Val + 'static,
    ) -> Self {
        let evaluator = Self::dispatch(free_evaluator, const_evaluator, mutable_evaluator);
        Primitive {
            id: Symbol::from(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    fn dispatch<Free, Const, Mutable>(
        f: Free,
        c: Const,
        m: Mutable,
    ) -> impl Fn(CtxForMutableFn, Val) -> Val + 'static
    where
        Free: Fn(FreeCtx, Val) -> Val + 'static,
        Const: Fn(ConstCtx, Val) -> Val + 'static,
        Mutable: Fn(MutableCtx, Val) -> Val + 'static,
    {
        move |ctx, val| match ctx {
            CtxForMutableFn::Free => f(FreeCtx, val),
            CtxForMutableFn::Const(ctx) => c(ConstCtx(ctx), val),
            CtxForMutableFn::Mutable(ctx) => m(MutableCtx(ctx), val),
        }
    }
}

pub(crate) mod strategy;

pub(crate) mod ctx;
