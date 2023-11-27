use {
    crate::{
        semantics::{
            ctx::{
                Ctx,
                InvariantTag,
                TaggedVal,
            },
            ctx_access::{
                constant::{
                    ConstCtx,
                    CtxForConstFn,
                },
                free::FreeCtx,
                mutable::{
                    CtxForMutableFn,
                    MutableCtx,
                },
                CtxAccessor,
            },
            eval::Evaluator,
            eval_mode::more::MoreByRef,
            input_mode::InputMode,
            val::{
                CtxVal,
                Val,
            },
        },
        types::{
            Reader,
            Symbol,
        },
    },
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

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct Func {
    pub(crate) input_mode: InputMode,
    pub(crate) evaluator: FuncEval,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum FuncEval {
    Free(CtxFreeEval),
    Const(CtxConstEval),
    Mutable(CtxMutableEval),
}

pub(crate) type CtxFreeEval = FuncImpl<Primitive<CtxFreeFn>, Composed<CtxFreeInfo>>;

pub(crate) type CtxConstEval = FuncImpl<Primitive<CtxConstFn>, Composed<CtxConstInfo>>;

pub(crate) type CtxMutableEval = FuncImpl<Primitive<CtxMutableFn>, Composed<CtxMutableInfo>>;

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

impl Func {
    #[allow(unused)]
    pub(crate) fn is_ctx_free(&self) -> bool {
        matches!(&self.evaluator, FuncEval::Free(_))
    }

    #[allow(unused)]
    pub(crate) fn is_ctx_const(&self) -> bool {
        matches!(&self.evaluator, FuncEval::Free(_) | FuncEval::Const(_))
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match &self.evaluator {
            FuncEval::Free(eval) => matches!(eval, FuncImpl::Primitive(_)),
            FuncEval::Const(eval) => matches!(eval, FuncImpl::Primitive(_)),
            FuncEval::Mutable(eval) => matches!(eval, FuncImpl::Primitive(_)),
        }
    }

    pub(crate) fn primitive_id(&self) -> Option<Symbol> {
        match &self.evaluator {
            FuncEval::Free(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.id.clone())
            }
            FuncEval::Const(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.id.clone())
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.id.clone())
            }
        }
    }

    pub(crate) fn composed_body(&self) -> Option<Val> {
        match &self.evaluator {
            FuncEval::Free(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.body.clone())
            }
            FuncEval::Const(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.body.clone())
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.body.clone())
            }
        }
    }

    pub(crate) fn composed_context(&self) -> Option<Ctx> {
        match &self.evaluator {
            FuncEval::Free(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.ctx.clone())
            }
            FuncEval::Const(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.ctx.clone())
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.ctx.clone())
            }
        }
    }

    pub(crate) fn composed_input_name(&self) -> Option<Symbol> {
        match &self.evaluator {
            FuncEval::Free(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.input_name.clone())
            }
            FuncEval::Const(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.input_name.clone())
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.input_name.clone())
            }
        }
    }

    pub(crate) fn composed_caller_name(&self) -> Option<Symbol> {
        match &self.evaluator {
            FuncEval::Free(_) => None,
            FuncEval::Const(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.caller.name.clone())
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Composed(eval) = eval else {
                    return None;
                };
                Some(eval.caller.name.clone())
            }
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Func
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.evaluator.eval(ctx, input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for FuncEval
where
    Ctx: CtxAccessor,
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
    Ctx: CtxAccessor,
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
    Ctx: CtxAccessor,
{
    fn eval(&self, _ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxFreeInfo>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, _ctx: &mut Ctx, input: Val) -> Val {
        eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<CtxConstFn>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx.for_const_fn(), input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxConstInfo>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match ctx.for_const_fn() {
            CtxForConstFn::Free(_ctx) => {
                eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
            }
            CtxForConstFn::Const(ctx) => eval_aware(
                self.ctx.clone(),
                ctx.0,
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
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        (self.eval_fn)(ctx.for_mutable_fn(), input)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Composed<CtxMutableInfo>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match ctx.for_mutable_fn() {
            CtxForMutableFn::Free(_ctx) => {
                eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
            }
            CtxForMutableFn::Const(ctx) => eval_aware(
                self.ctx.clone(),
                ctx.0,
                self.caller.name.clone(),
                InvariantTag::Const,
                input,
                self.input_name.clone(),
                &self.body,
            ),
            CtxForMutableFn::Mutable(ctx) => eval_aware(
                self.ctx.clone(),
                ctx.0,
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
    let _ = new_ctx.put_val_local(input_name, TaggedVal::new(input));
    MoreByRef.eval(&mut MutableCtx(&mut new_ctx), body)
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
    let _ = new_ctx.put_val_local(input_name, TaggedVal::new(input));
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
    keep_ctx(&mut new_ctx, caller, caller_name.clone(), caller_tag);
    let output = MoreByRef.eval(&mut MutableCtx(&mut new_ctx), body);
    restore_ctx(ctx, new_ctx, &caller_name);
    output
}

// here is why we need a `&mut Ctx` for a const func
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Symbol, tag: InvariantTag) {
    let val = Val::Ctx(CtxVal(Box::new(ctx)));
    let _ = new_ctx.put_val_local(name, TaggedVal { val, tag });
}

fn restore_ctx(ctx: &mut Ctx, new_ctx: Ctx, name: &str) {
    let Val::Ctx(CtxVal(caller)) = new_ctx.into_val(name) else {
        return;
    };
    *ctx = *caller;
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

#[allow(unused)]
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

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Func");
        match &self.evaluator {
            FuncEval::Free(eval) => match eval {
                CtxFreeEval::Primitive(p) => {
                    s.field("id", &p.id);
                }
                CtxFreeEval::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("caller_access", &"free");
                    s.field("body", &c.body);
                    s.field("context", &c.ctx);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncEval::Const(eval) => match eval {
                CtxConstEval::Primitive(p) => {
                    s.field("id", &p.id);
                }
                CtxConstEval::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("caller_access", &"constant");
                    s.field("body", &c.body);
                    s.field("context", &c.ctx);
                    s.field("caller_name", &c.caller.name);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncEval::Mutable(eval) => match eval {
                CtxMutableEval::Primitive(p) => {
                    s.field("id", &p.id);
                }
                CtxMutableEval::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("caller_access", &"mutable");
                    s.field("body", &c.body);
                    s.field("context", &c.ctx);
                    s.field("caller_name", &c.caller.name);
                    s.field("input_name", &c.input_name);
                }
            },
        }
        s.finish()
    }
}

impl Func {
    pub(crate) fn new(input_mode: InputMode, evaluator: FuncEval) -> Self {
        Func {
            input_mode,
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
            id: Symbol::from_str(id),
            eval_fn: Reader::new(evaluator),
        }
    }
}

impl Primitive<CtxConstFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(CtxForConstFn, Val) -> Val + 'static) -> Self {
        Primitive {
            id: Symbol::from_str(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    #[allow(unused)]
    pub(crate) fn dispatch<Free, Const>(
        f: Free,
        c: Const,
    ) -> impl Fn(CtxForConstFn, Val) -> Val + 'static
    where
        Free: Fn(FreeCtx, Val) -> Val + 'static,
        Const: Fn(ConstCtx, Val) -> Val + 'static,
    {
        move |ctx, val| match ctx {
            CtxForConstFn::Free(ctx) => f(ctx, val),
            CtxForConstFn::Const(ctx) => c(ctx, val),
        }
    }
}

impl Primitive<CtxMutableFn> {
    pub(crate) fn new(id: &str, evaluator: impl Fn(CtxForMutableFn, Val) -> Val + 'static) -> Self {
        Primitive {
            id: Symbol::from_str(id),
            eval_fn: Reader::new(evaluator),
        }
    }

    pub(crate) fn dispatch<Free, Const, Mutable>(
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
            CtxForMutableFn::Free(ctx) => f(ctx, val),
            CtxForMutableFn::Const(ctx) => c(ctx, val),
            CtxForMutableFn::Mutable(ctx) => m(ctx, val),
        }
    }
}
