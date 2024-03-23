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
    mode::TransformMode,
    symbol::Symbol,
    transform::eval::EvalByRef,
    transformer::Transformer,
    val::{
        ctx::CtxVal,
        Val,
    },
};

#[derive(Eq, PartialEq, Hash)]
pub struct Func {
    pub(crate) input_mode: TransformMode,
    pub(crate) output_mode: TransformMode,
    pub(crate) core: FuncCore,
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
pub(crate) enum FuncCore {
    Free(CtxFree),
    Const(CtxConst),
    Mutable(CtxMutable),
}

pub(crate) type CtxFree = FuncImpl<Primitive<Box<dyn CtxFreeFn>>, Composed<CtxFreeInfo>>;

pub(crate) type CtxConst = FuncImpl<Primitive<Box<dyn CtxConstFn>>, Composed<CtxConstInfo>>;

pub(crate) type CtxMutable = FuncImpl<Primitive<Box<dyn CtxMutableFn>>, Composed<CtxMutableInfo>>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composed(C),
}

#[derive(Clone)]
pub(crate) struct Primitive<F> {
    is_extension: bool,
    id: Symbol,
    f: F,
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
    pub fn input_mode(&self) -> &TransformMode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &TransformMode {
        &self.output_mode
    }

    #[allow(unused)]
    pub(crate) fn is_ctx_free(&self) -> bool {
        matches!(&self.core, FuncCore::Free(_))
    }

    #[allow(unused)]
    pub(crate) fn is_ctx_const(&self) -> bool {
        matches!(&self.core, FuncCore::Free(_) | FuncCore::Const(_))
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match &self.core {
            FuncCore::Free(f) => matches!(f, FuncImpl::Primitive(_)),
            FuncCore::Const(f) => matches!(f, FuncImpl::Primitive(_)),
            FuncCore::Mutable(f) => matches!(f, FuncImpl::Primitive(_)),
        }
    }

    pub(crate) fn primitive_id(&self) -> Option<Symbol> {
        match &self.core {
            FuncCore::Free(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
            FuncCore::Const(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
        }
    }

    pub(crate) fn primitive_is_extension(&self) -> Option<bool> {
        match &self.core {
            FuncCore::Free(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
            FuncCore::Const(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
        }
    }

    pub(crate) fn composed_body(&self) -> Option<Val> {
        match &self.core {
            FuncCore::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
            FuncCore::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
        }
    }

    pub(crate) fn composed_context(&self) -> Option<Ctx> {
        match &self.core {
            FuncCore::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.ctx.clone())
            }
            FuncCore::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.ctx.clone())
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.ctx.clone())
            }
        }
    }

    pub(crate) fn composed_input_name(&self) -> Option<Symbol> {
        match &self.core {
            FuncCore::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
            FuncCore::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
        }
    }

    pub(crate) fn composed_caller_name(&self) -> Option<Symbol> {
        match &self.core {
            FuncCore::Free(_) => None,
            FuncCore::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.caller.name.clone())
            }
            FuncCore::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.caller.name.clone())
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Func
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.core.transform(ctx, input)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for FuncCore
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncCore::Free(func) => func.transform(ctx, input),
            FuncCore::Const(func) => func.transform(ctx, input),
            FuncCore::Mutable(func) => func.transform(ctx, input),
        }
    }
}

impl<Ctx, P: Transformer<Ctx, Val, Val>, C: Transformer<Ctx, Val, Val>> Transformer<Ctx, Val, Val>
    for FuncImpl<P, C>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncImpl::Primitive(p) => p.transform(ctx, input),
            FuncImpl::Composed(c) => c.transform(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Primitive<Box<dyn CtxFreeFn>>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, _ctx: &mut Ctx, input: Val) -> Val {
        self.f.call(input)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Composed<CtxFreeInfo>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, _ctx: &mut Ctx, input: Val) -> Val {
        eval_free(self.ctx.clone(), input, self.input_name.clone(), &self.body)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Primitive<Box<dyn CtxConstFn>>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.f.call(ctx.for_const_fn(), input)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Composed<CtxConstInfo>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
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
                // INVARIANT: We use the const tag to indicate not to modify this context.
                ctx.temp_take(f)
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Primitive<Box<dyn CtxMutableFn>>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.f.call(ctx.for_mutable_fn(), input)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for Composed<CtxMutableInfo>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
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
                // INVARIANT: We use the const tag to indicate not to modify this context.
                ctx.temp_take(f)
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
                // INVARIANT: We use the final tag to indicate not to move this context.
                ctx.temp_take(f)
            }
        }
    }
}

fn eval_free(mut new_ctx: Ctx, input: Val, input_name: Symbol, body: &Val) -> Val {
    let _ = new_ctx.put_val(input_name, TaggedVal::new(input));
    EvalByRef.transform(&mut MutableCtx::new(&mut new_ctx), body)
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
    let _ = new_ctx.put_val(input_name, TaggedVal::new(input));
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
    let output = EvalByRef.transform(&mut MutableCtx::new(&mut new_ctx), body);
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
    let _ = new_ctx.put_val(name, TaggedVal { val, tag });
}

fn restore_ctx(ctx: &mut Ctx, new_ctx: Ctx, name: &str) {
    let Ok(Val::Ctx(CtxVal(caller))) = new_ctx.into_val(name) else {
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
        match &self.core {
            FuncCore::Free(f) => match f {
                CtxFree::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxFree::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("caller_access", &"free");
                    s.field("body", &c.body);
                    s.field("context", &c.ctx);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncCore::Const(f) => match f {
                CtxConst::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxConst::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("caller_access", &"constant");
                    s.field("body", &c.body);
                    s.field("context", &c.ctx);
                    s.field("caller_name", &c.caller.name);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncCore::Mutable(f) => match f {
                CtxMutable::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxMutable::Composed(c) => {
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
    pub(crate) fn new(
        input_mode: TransformMode,
        output_mode: TransformMode,
        core: FuncCore,
    ) -> Self {
        Func {
            input_mode,
            output_mode,
            core,
        }
    }

    pub fn new_free(
        input_mode: TransformMode,
        output_mode: TransformMode,
        id: Symbol,
        f: Box<dyn CtxFreeFn>,
    ) -> Self {
        let core = FuncCore::Free(CtxFree::Primitive(Primitive {
            is_extension: true,
            id,
            f,
        }));
        Func {
            input_mode,
            output_mode,
            core,
        }
    }

    pub fn new_const(
        input_mode: TransformMode,
        output_mode: TransformMode,
        id: Symbol,
        f: Box<dyn CtxConstFn>,
    ) -> Self {
        let core = FuncCore::Const(CtxConst::Primitive(Primitive {
            is_extension: true,
            id,
            f,
        }));
        Func {
            input_mode,
            output_mode,
            core,
        }
    }

    pub fn new_mutable(
        input_mode: TransformMode,
        output_mode: TransformMode,
        id: Symbol,
        f: Box<dyn CtxMutableFn>,
    ) -> Self {
        let core = FuncCore::Mutable(CtxMutable::Primitive(Primitive {
            is_extension: true,
            id,
            f,
        }));
        Func {
            input_mode,
            output_mode,
            core,
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
    pub(crate) fn new(id: &str, f: impl CtxFreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            f: Box::new(f),
        }
    }
}

impl Primitive<Box<dyn CtxConstFn>> {
    pub(crate) fn new(id: &str, f: impl CtxConstFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            f: Box::new(f),
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
    pub(crate) fn new(id: &str, f: impl CtxMutableFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            f: Box::new(f),
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
