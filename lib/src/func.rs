use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    mem::swap,
};

use crate::{
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
    eval_mode::eager::EagerByRef,
    io_mode::IoMode,
    symbol::Symbol,
    val::{
        ctx::CtxVal,
        Val,
    },
};

#[derive(Eq, PartialEq, Hash)]
pub struct Func {
    pub(crate) input_mode: IoMode,
    pub(crate) output_mode: IoMode,
    pub(crate) evaluator: FuncEval,
}

pub trait CtxFreeFn {
    fn call(&self, input: Val) -> Val;
}

pub trait CtxConstFn {
    fn call(&self, ctx: CtxForConstFn, input: Val) -> Val;
}

pub trait CtxMutableFn {
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val;
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum FuncEval {
    Free(CtxFreeEval),
    Const(CtxConstEval),
    Mutable(CtxMutableEval),
}

pub(crate) type CtxFreeEval = FuncImpl<Primitive<Box<dyn CtxFreeFn>>, Composed<CtxFreeInfo>>;

pub(crate) type CtxConstEval = FuncImpl<Primitive<Box<dyn CtxConstFn>>, Composed<CtxConstInfo>>;

pub(crate) type CtxMutableEval =
    FuncImpl<Primitive<Box<dyn CtxMutableFn>>, Composed<CtxMutableInfo>>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composed(C),
}

#[derive(Clone)]
pub(crate) struct Primitive<F> {
    is_extension: bool,
    id: Symbol,
    eval_fn: F,
}

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
    pub fn input_mode(&self) -> &IoMode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &IoMode {
        &self.output_mode
    }

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

    pub(crate) fn primitive_is_extension(&self) -> Option<bool> {
        match &self.evaluator {
            FuncEval::Free(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.is_extension)
            }
            FuncEval::Const(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.is_extension)
            }
            FuncEval::Mutable(eval) => {
                let FuncImpl::Primitive(eval) = eval else {
                    return None;
                };
                Some(eval.is_extension)
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

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<Box<dyn CtxFreeFn>>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, _ctx: &mut Ctx, input: Val) -> Val {
        self.eval_fn.call(input)
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

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<Box<dyn CtxConstFn>>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_fn.call(ctx.for_const_fn(), input)
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
            CtxForConstFn::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.ctx.clone(),
                        ctx,
                        self.caller.name.clone(),
                        InvariantTag::Const,
                        input,
                        self.input_name.clone(),
                        &self.body,
                    )
                };
                // SAFETY: We use the const tag to indicate not to modify this context.
                unsafe { ctx.temp_take(f) }
            }
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for Primitive<Box<dyn CtxMutableFn>>
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_fn.call(ctx.for_mutable_fn(), input)
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
            CtxForMutableFn::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.ctx.clone(),
                        ctx,
                        self.caller.name.clone(),
                        InvariantTag::Const,
                        input,
                        self.input_name.clone(),
                        &self.body,
                    )
                };
                // SAFETY: We use the const tag to indicate not to modify this context.
                unsafe { ctx.temp_take(f) }
            }
            CtxForMutableFn::Mutable(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.ctx.clone(),
                        ctx,
                        self.caller.name.clone(),
                        InvariantTag::Final,
                        input,
                        self.input_name.clone(),
                        &self.body,
                    )
                };
                // SAFETY: We use the final tag to indicate not to move this context.
                unsafe { ctx.temp_take(f) }
            }
        }
    }
}

fn eval_free(mut new_ctx: Ctx, input: Val, input_name: Symbol, body: &Val) -> Val {
    let _ = new_ctx.put_val_local(input_name, TaggedVal::new(input));
    EagerByRef.eval(&mut MutableCtx::new(&mut new_ctx), body)
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
    let output = EagerByRef.eval(&mut MutableCtx::new(&mut new_ctx), body);
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
        self.id == other.id && self.is_extension == other.is_extension
    }
}

impl<F> Eq for Primitive<F> {}

impl<F> Hash for Primitive<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.is_extension.hash(state);
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

impl<T> Debug for Primitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Primitive").field(&self.id).finish()
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Func");
        match &self.evaluator {
            FuncEval::Free(eval) => match eval {
                CtxFreeEval::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
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
                    s.field("is_extension", &p.is_extension);
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
                    s.field("is_extension", &p.is_extension);
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
    pub(crate) fn new(input_mode: IoMode, output_mode: IoMode, evaluator: FuncEval) -> Self {
        Func {
            input_mode,
            output_mode,
            evaluator,
        }
    }

    pub fn new_free(
        input_mode: IoMode,
        output_mode: IoMode,
        id: Symbol,
        f: Box<dyn CtxFreeFn>,
    ) -> Self {
        let evaluator = FuncEval::Free(CtxFreeEval::Primitive(Primitive {
            is_extension: true,
            id,
            eval_fn: f,
        }));
        Func {
            input_mode,
            output_mode,
            evaluator,
        }
    }

    pub fn new_const(
        input_mode: IoMode,
        output_mode: IoMode,
        id: Symbol,
        f: Box<dyn CtxConstFn>,
    ) -> Self {
        let evaluator = FuncEval::Const(CtxConstEval::Primitive(Primitive {
            is_extension: true,
            id,
            eval_fn: f,
        }));
        Func {
            input_mode,
            output_mode,
            evaluator,
        }
    }

    pub fn new_mutable(
        input_mode: IoMode,
        output_mode: IoMode,
        id: Symbol,
        f: Box<dyn CtxMutableFn>,
    ) -> Self {
        let evaluator = FuncEval::Mutable(CtxMutableEval::Primitive(Primitive {
            is_extension: true,
            id,
            eval_fn: f,
        }));
        Func {
            input_mode,
            output_mode,
            evaluator,
        }
    }
}

impl<C> Primitive<C> {
    pub(crate) fn get_id(&self) -> &Symbol {
        &self.id
    }

    pub(crate) fn is_extension(&self) -> bool {
        self.is_extension
    }
}

impl Primitive<Box<dyn CtxFreeFn>> {
    pub(crate) fn new(id: &str, evaluator: impl CtxFreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            eval_fn: Box::new(evaluator),
        }
    }
}

impl Primitive<Box<dyn CtxConstFn>> {
    pub(crate) fn new(id: &str, evaluator: impl CtxConstFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            eval_fn: Box::new(evaluator),
        }
    }
}

pub(crate) struct ConstDispatcher<Free, Const> {
    free_fn: Free,
    const_fn: Const,
}

impl<Free, Const> ConstDispatcher<Free, Const>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
{
    #[allow(unused)]
    pub(crate) fn new(free_fn: Free, const_fn: Const) -> Self {
        Self { free_fn, const_fn }
    }
}

impl<Free, Const> CtxConstFn for ConstDispatcher<Free, Const>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: CtxForConstFn, input: Val) -> Val {
        match ctx {
            CtxForConstFn::Free(ctx) => (self.free_fn)(ctx, input),
            CtxForConstFn::Const(ctx) => (self.const_fn)(ctx, input),
        }
    }
}

impl Primitive<Box<dyn CtxMutableFn>> {
    pub(crate) fn new(id: &str, evaluator: impl CtxMutableFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            eval_fn: Box::new(evaluator),
        }
    }
}

pub(crate) struct MutableDispatcher<Free, Const, Mutable> {
    free_fn: Free,
    const_fn: Const,
    mutable_fn: Mutable,
}

impl<Free, Const, Mutable> MutableDispatcher<Free, Const, Mutable>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mutable: Fn(MutableCtx, Val) -> Val + 'static,
{
    pub(crate) fn new(free_fn: Free, const_fn: Const, mutable_fn: Mutable) -> Self {
        Self {
            free_fn,
            const_fn,
            mutable_fn,
        }
    }
}

impl<Free, Const, Mutable> CtxMutableFn for MutableDispatcher<Free, Const, Mutable>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mutable: Fn(MutableCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        match ctx {
            CtxForMutableFn::Free(ctx) => (self.free_fn)(ctx, input),
            CtxForMutableFn::Const(ctx) => (self.const_fn)(ctx, input),
            CtxForMutableFn::Mutable(ctx) => (self.mutable_fn)(ctx, input),
        }
    }
}

impl<T> CtxFreeFn for T
where
    T: Fn(Val) -> Val,
{
    fn call(&self, input: Val) -> Val {
        self(input)
    }
}

impl<T> CtxConstFn for T
where
    T: Fn(CtxForConstFn, Val) -> Val,
{
    fn call(&self, ctx: CtxForConstFn, input: Val) -> Val {
        self(ctx, input)
    }
}

impl<T> CtxMutableFn for T
where
    T: Fn(CtxForMutableFn, Val) -> Val,
{
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val {
        self(ctx, input)
    }
}
