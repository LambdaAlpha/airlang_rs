use std::{
    fmt::DebugStruct,
    rc::Rc,
};

use crate::{
    ConstCtx,
    ConstFnCtx,
    FreeCtx,
    Invariant,
    Mode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        Composite,
        Func,
        FuncImpl,
        Primitive,
        eval_aware,
        eval_free,
    },
    transformer::Transformer,
};

pub trait ConstFn {
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val;
}

#[derive(Clone)]
pub struct ConstPrimitiveExt {
    pub(crate) fn1: Rc<dyn ConstFn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstCompositeExt {
    pub(crate) ctx_name: Symbol,
}

pub type ConstFunc = Func<ConstPrimitiveExt, ConstCompositeExt>;

impl Transformer<Val, Val> for Primitive<ConstPrimitiveExt> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.ext.fn1.call(ctx.for_const_fn(), input)
    }
}

impl Transformer<Val, Val> for Composite<ConstCompositeExt> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_const_fn() {
            ConstFnCtx::Free(_ctx) => eval_free(
                &mut self.prelude.clone(),
                input,
                self.input_name.clone(),
                self.body.clone(),
            ),
            ConstFnCtx::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.prelude.clone(),
                        ctx,
                        self.ext.ctx_name.clone(),
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

impl ConstFunc {
    pub fn new(
        call_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        id: Symbol,
        fn1: Rc<dyn ConstFn>,
    ) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: ConstPrimitiveExt { fn1 },
        });
        Self {
            call_mode,
            ask_mode,
            cacheable,
            transformer,
        }
    }

    pub(crate) fn ctx_name(&self) -> Option<Symbol> {
        let FuncImpl::Composite(c) = self.transformer() else {
            return None;
        };
        Some(c.ext.ctx_name.clone())
    }
}

impl Primitive<ConstPrimitiveExt> {
    pub(crate) fn new(id: &str, f: impl ConstFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: ConstPrimitiveExt { fn1: Rc::new(f) },
        }
    }
}

impl Composite<ConstCompositeExt> {
    pub(crate) fn dbg_field_ext(&self, s: &mut DebugStruct) {
        s.field("ctx_name", &self.ext.ctx_name);
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

impl<Free, Const> ConstFn for ConstDispatcher<Free, Const>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val {
        match ctx {
            ConstFnCtx::Free(ctx) => (self.free_fn)(ctx, input),
            ConstFnCtx::Const(ctx) => (self.const_fn)(ctx, input),
        }
    }
}

impl<T> ConstFn for T
where
    T: Fn(ConstFnCtx, Val) -> Val,
{
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val {
        self(ctx, input)
    }
}
