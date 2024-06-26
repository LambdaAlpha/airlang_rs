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
    rc::Rc,
};

use crate::{
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
        ref1::{
            CtxMeta,
            CtxRef,
        },
        Ctx,
        CtxValue,
        Invariant,
    },
    mode::Mode,
    symbol::Symbol,
    transform::eval::Eval,
    transformer::Transformer,
    val::{
        ctx::CtxVal,
        Val,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Func {
    pub(crate) input_mode: Mode,
    pub(crate) output_mode: Mode,
    pub(crate) transformer: FuncTransformer,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FuncTransformer {
    Free(CtxFree),
    Const(CtxConst),
    Mutable(CtxMutable),
}

pub(crate) type CtxFree = FuncImpl<Primitive<Rc<dyn CtxFreeFn>>, Composed<CtxFreeInfo>>;

pub(crate) type CtxConst = FuncImpl<Primitive<Rc<dyn CtxConstFn>>, Composed<CtxConstInfo>>;

pub(crate) type CtxMutable = FuncImpl<Primitive<Rc<dyn CtxMutableFn>>, Composed<CtxMutableInfo>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composed(C),
}

#[derive(Clone)]
pub(crate) struct Primitive<Fn> {
    is_extension: bool,
    id: Symbol,
    fn1: Fn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed<C> {
    pub(crate) body: Val,
    pub(crate) prelude: Ctx,
    pub(crate) input_name: Symbol,
    pub(crate) ctx: C,
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
    pub fn input_mode(&self) -> &Mode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &Mode {
        &self.output_mode
    }

    #[allow(unused)]
    pub(crate) fn is_ctx_free(&self) -> bool {
        matches!(&self.transformer, FuncTransformer::Free(_))
    }

    #[allow(unused)]
    pub(crate) fn is_ctx_const(&self) -> bool {
        matches!(
            &self.transformer,
            FuncTransformer::Free(_) | FuncTransformer::Const(_)
        )
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match &self.transformer {
            FuncTransformer::Free(f) => matches!(f, FuncImpl::Primitive(_)),
            FuncTransformer::Const(f) => matches!(f, FuncImpl::Primitive(_)),
            FuncTransformer::Mutable(f) => matches!(f, FuncImpl::Primitive(_)),
        }
    }

    pub(crate) fn primitive_id(&self) -> Option<Symbol> {
        match &self.transformer {
            FuncTransformer::Free(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
            FuncTransformer::Const(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.id.clone())
            }
        }
    }

    pub(crate) fn primitive_is_extension(&self) -> Option<bool> {
        match &self.transformer {
            FuncTransformer::Free(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
            FuncTransformer::Const(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Primitive(f) = f else {
                    return None;
                };
                Some(f.is_extension)
            }
        }
    }

    pub(crate) fn composed_body(&self) -> Option<Val> {
        match &self.transformer {
            FuncTransformer::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
            FuncTransformer::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.body.clone())
            }
        }
    }

    pub(crate) fn composed_prelude(&self) -> Option<Ctx> {
        match &self.transformer {
            FuncTransformer::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.prelude.clone())
            }
            FuncTransformer::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.prelude.clone())
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.prelude.clone())
            }
        }
    }

    pub(crate) fn composed_input_name(&self) -> Option<Symbol> {
        match &self.transformer {
            FuncTransformer::Free(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
            FuncTransformer::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.input_name.clone())
            }
        }
    }

    pub(crate) fn composed_ctx_name(&self) -> Option<Symbol> {
        match &self.transformer {
            FuncTransformer::Free(_) => None,
            FuncTransformer::Const(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.ctx.name.clone())
            }
            FuncTransformer::Mutable(f) => {
                let FuncImpl::Composed(f) = f else {
                    return None;
                };
                Some(f.ctx.name.clone())
            }
        }
    }
}

impl Transformer<Val, Val> for Func {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.transformer.transform(ctx, input)
    }
}

impl Transformer<Val, Val> for FuncTransformer {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncTransformer::Free(func) => func.transform(ctx, input),
            FuncTransformer::Const(func) => func.transform(ctx, input),
            FuncTransformer::Mutable(func) => func.transform(ctx, input),
        }
    }
}

impl<P: Transformer<Val, Val>, C: Transformer<Val, Val>> Transformer<Val, Val> for FuncImpl<P, C> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncImpl::Primitive(p) => p.transform(ctx, input),
            FuncImpl::Composed(c) => c.transform(ctx, input),
        }
    }
}

impl Transformer<Val, Val> for Primitive<Rc<dyn CtxFreeFn>> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(input)
    }
}

impl Transformer<Val, Val> for Composed<CtxFreeInfo> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        eval_free(
            self.prelude.clone(),
            input,
            self.input_name.clone(),
            self.body.clone(),
        )
    }
}

impl Transformer<Val, Val> for Primitive<Rc<dyn CtxConstFn>> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(ctx.for_const_fn(), input)
    }
}

impl Transformer<Val, Val> for Composed<CtxConstInfo> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_const_fn() {
            CtxForConstFn::Free(_ctx) => eval_free(
                self.prelude.clone(),
                input,
                self.input_name.clone(),
                self.body.clone(),
            ),
            CtxForConstFn::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.prelude.clone(),
                        ctx,
                        self.ctx.name.clone(),
                        Invariant::Const,
                        input,
                        self.input_name.clone(),
                        self.body.clone(),
                    )
                };
                // INVARIANT: We use the const invariant to indicate not to modify this context.
                ctx.temp_take(f)
            }
        }
    }
}

impl Transformer<Val, Val> for Primitive<Rc<dyn CtxMutableFn>> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(ctx.for_mutable_fn(), input)
    }
}

impl Transformer<Val, Val> for Composed<CtxMutableInfo> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_mutable_fn() {
            CtxForMutableFn::Free(_ctx) => eval_free(
                self.prelude.clone(),
                input,
                self.input_name.clone(),
                self.body.clone(),
            ),
            CtxForMutableFn::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.prelude.clone(),
                        ctx,
                        self.ctx.name.clone(),
                        Invariant::Const,
                        input,
                        self.input_name.clone(),
                        self.body.clone(),
                    )
                };
                // INVARIANT: We use the const invariant to indicate not to modify this context.
                ctx.temp_take(f)
            }
            CtxForMutableFn::Mutable(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.prelude.clone(),
                        ctx,
                        self.ctx.name.clone(),
                        Invariant::Final,
                        input,
                        self.input_name.clone(),
                        self.body.clone(),
                    )
                };
                // INVARIANT: We use the final invariant to indicate not to move this context.
                ctx.temp_take(f)
            }
        }
    }
}

fn eval_free(mut new_ctx: Ctx, input: Val, input_name: Symbol, body: Val) -> Val {
    let result = (&mut new_ctx).put_value(input_name, CtxValue::new(input));
    if result.is_err() {
        return Val::default();
    }
    Eval.transform(MutableCtx::new(&mut new_ctx), body)
}

fn eval_aware(
    mut new_ctx: Ctx,
    caller: &mut Ctx,
    caller_name: Symbol,
    caller_invariant: Invariant,
    input: Val,
    input_name: Symbol,
    body: Val,
) -> Val {
    let result = (&mut new_ctx).put_value(input_name, CtxValue::new(input));
    if result.is_err() {
        return Val::default();
    }
    keep_eval_restore(new_ctx, caller, caller_name, caller_invariant, body)
}

fn keep_eval_restore(
    mut new_ctx: Ctx,
    ctx: &mut Ctx,
    caller_name: Symbol,
    caller_invariant: Invariant,
    body: Val,
) -> Val {
    if !ctx.is_assignable(caller_name.clone()) {
        return Val::default();
    }
    let caller = own_ctx(ctx);
    keep_ctx(&mut new_ctx, caller, caller_name.clone(), caller_invariant);
    let output = Eval.transform(MutableCtx::new(&mut new_ctx), body);
    restore_ctx(ctx, new_ctx, caller_name);
    output
}

// here is why we need a `&mut Ctx` for a const func
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Symbol, invariant: Invariant) {
    let val = Val::Ctx(CtxVal::from(ctx));
    new_ctx
        .put_value(name, CtxValue { val, invariant })
        .expect("name should be assignable");
}

fn restore_ctx(ctx: &mut Ctx, new_ctx: Ctx, name: Symbol) {
    let Ok(Val::Ctx(caller)) = new_ctx.into_val(name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    let caller = Ctx::from(caller);
    *ctx = caller;
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

impl<T> Debug for Primitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Primitive").field(&self.id).finish()
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Func");
        match &self.transformer {
            FuncTransformer::Free(f) => match f {
                CtxFree::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxFree::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("output_mode", &self.output_mode);
                    s.field("context_access", &"free");
                    s.field("body", &c.body);
                    s.field("prelude", &c.prelude);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncTransformer::Const(f) => match f {
                CtxConst::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxConst::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("output_mode", &self.output_mode);
                    s.field("context_access", &"constant");
                    s.field("body", &c.body);
                    s.field("prelude", &c.prelude);
                    s.field("context_name", &c.ctx.name);
                    s.field("input_name", &c.input_name);
                }
            },
            FuncTransformer::Mutable(f) => match f {
                CtxMutable::Primitive(p) => {
                    s.field("id", &p.id);
                    s.field("is_extension", &p.is_extension);
                }
                CtxMutable::Composed(c) => {
                    s.field("input_mode", &self.input_mode);
                    s.field("output_mode", &self.output_mode);
                    s.field("context_access", &"mutable");
                    s.field("body", &c.body);
                    s.field("prelude", &c.prelude);
                    s.field("context_name", &c.ctx.name);
                    s.field("input_name", &c.input_name);
                }
            },
        }
        s.finish()
    }
}

impl Func {
    pub(crate) fn new(input_mode: Mode, output_mode: Mode, transformer: FuncTransformer) -> Self {
        Func {
            input_mode,
            output_mode,
            transformer,
        }
    }

    pub fn new_free(
        input_mode: Mode,
        output_mode: Mode,
        id: Symbol,
        fn1: Rc<dyn CtxFreeFn>,
    ) -> Self {
        let transformer = FuncTransformer::Free(CtxFree::Primitive(Primitive {
            is_extension: true,
            id,
            fn1,
        }));
        Func {
            input_mode,
            output_mode,
            transformer,
        }
    }

    pub fn new_const(
        input_mode: Mode,
        output_mode: Mode,
        id: Symbol,
        fn1: Rc<dyn CtxConstFn>,
    ) -> Self {
        let transformer = FuncTransformer::Const(CtxConst::Primitive(Primitive {
            is_extension: true,
            id,
            fn1,
        }));
        Func {
            input_mode,
            output_mode,
            transformer,
        }
    }

    pub fn new_mutable(
        input_mode: Mode,
        output_mode: Mode,
        id: Symbol,
        fn1: Rc<dyn CtxMutableFn>,
    ) -> Self {
        let transformer = FuncTransformer::Mutable(CtxMutable::Primitive(Primitive {
            is_extension: true,
            id,
            fn1,
        }));
        Func {
            input_mode,
            output_mode,
            transformer,
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

impl Primitive<Rc<dyn CtxFreeFn>> {
    pub(crate) fn new(id: &str, f: impl CtxFreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            fn1: Rc::new(f),
        }
    }
}

impl Primitive<Rc<dyn CtxConstFn>> {
    pub(crate) fn new(id: &str, f: impl CtxConstFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            fn1: Rc::new(f),
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

impl Primitive<Rc<dyn CtxMutableFn>> {
    pub(crate) fn new(id: &str, f: impl CtxMutableFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            fn1: Rc::new(f),
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
