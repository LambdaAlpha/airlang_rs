use std::rc::Rc;

use crate::{
    ctx::ref1::CtxMeta,
    func::{
        eval_aware,
        eval_free,
        Composed,
        Func,
        FuncImpl,
        Primitive,
    },
    transformer::Transformer,
    ConstCtx,
    ConstFnCtx,
    FreeCtx,
    Invariant,
    Mode,
    Symbol,
    Val,
};

pub trait ConstFn {
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstInfo {
    pub(crate) name: Symbol,
}

pub type ConstFunc = Func<Rc<dyn ConstFn>, ConstInfo>;

impl Transformer<Val, Val> for Primitive<Rc<dyn ConstFn>> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(ctx.for_const_fn(), input)
    }
}

impl Transformer<Val, Val> for Composed<ConstInfo> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_const_fn() {
            ConstFnCtx::Free(_ctx) => eval_free(
                self.prelude.clone(),
                input,
                self.input_name.clone(),
                self.body.clone(),
            ),
            ConstFnCtx::Const(mut ctx) => {
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

impl ConstFunc {
    pub fn new(input_mode: Mode, output_mode: Mode, id: Symbol, fn1: Rc<dyn ConstFn>) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            fn1,
        });
        Self {
            input_mode,
            output_mode,
            transformer,
        }
    }
}

impl Primitive<Rc<dyn ConstFn>> {
    pub(crate) fn new(id: &str, f: impl ConstFn + 'static) -> Self {
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
