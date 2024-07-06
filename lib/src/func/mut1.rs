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
    FreeCtx,
    Invariant,
    Mode,
    MutCtx,
    MutFnCtx,
    Symbol,
    Val,
};

pub trait MutFn {
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutInfo {
    pub(crate) name: Symbol,
}

pub type MutFunc = Func<Rc<dyn MutFn>, MutInfo>;

impl Transformer<Val, Val> for Primitive<Rc<dyn MutFn>> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(ctx.for_mut_fn(), input)
    }
}

impl Transformer<Val, Val> for Composed<MutInfo> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_mut_fn() {
            MutFnCtx::Free(_ctx) => eval_free(
                self.prelude.clone(),
                input,
                self.input_name.clone(),
                self.body.clone(),
            ),
            MutFnCtx::Const(mut ctx) => {
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
            MutFnCtx::Mut(mut ctx) => {
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

impl MutFunc {
    pub fn new(input_mode: Mode, output_mode: Mode, id: Symbol, fn1: Rc<dyn MutFn>) -> Self {
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

impl Primitive<Rc<dyn MutFn>> {
    pub(crate) fn new(id: &str, f: impl MutFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            fn1: Rc::new(f),
        }
    }
}

pub(crate) struct MutDispatcher<Free, Const, Mut> {
    free_fn: Free,
    const_fn: Const,
    mut_fn: Mut,
}

impl<Free, Const, Mut> MutDispatcher<Free, Const, Mut>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mut: Fn(MutCtx, Val) -> Val + 'static,
{
    pub(crate) fn new(free_fn: Free, const_fn: Const, mut_fn: Mut) -> Self {
        Self {
            free_fn,
            const_fn,
            mut_fn,
        }
    }
}

impl<Free, Const, Mut> MutFn for MutDispatcher<Free, Const, Mut>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mut: Fn(MutCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val {
        match ctx {
            MutFnCtx::Free(ctx) => (self.free_fn)(ctx, input),
            MutFnCtx::Const(ctx) => (self.const_fn)(ctx, input),
            MutFnCtx::Mut(ctx) => (self.mut_fn)(ctx, input),
        }
    }
}

impl<T> MutFn for T
where
    T: Fn(MutFnCtx, Val) -> Val,
{
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val {
        self(ctx, input)
    }
}
