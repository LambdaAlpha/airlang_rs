use crate::{
    FuncMode,
    Invariant,
    Mode,
    MutFnCtx,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::Composite,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutCellCompFunc {
    pub(crate) comp: Composite,
    pub(crate) ctx_name: Symbol,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for MutCellCompFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let inner = &mut self.comp.ctx.clone();
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body_mode = &self.comp.body_mode;
        let body = self.comp.body.clone();
        Self::transform_mut(inner, ctx_name, ctx, input_name, input, body_mode, body)
    }
}

impl FuncTrait for MutCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let inner = &mut self.comp.ctx;
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body_mode = &self.comp.body_mode;
        let body = self.comp.body.clone();
        Self::transform_mut(inner, ctx_name, ctx, input_name, input, body_mode, body)
    }
}

impl MutCellCompFunc {
    pub(crate) fn new(comp: Composite, ctx_name: Symbol, mode: FuncMode, cacheable: bool) -> Self {
        Self {
            comp,
            ctx_name,
            mode,
            cacheable,
        }
    }

    fn transform_mut<'a, Ctx>(
        inner: &mut crate::Ctx,
        ctx_name: Symbol,
        outer: Ctx,
        input_name: Symbol,
        input: Val,
        body_mode: &Mode,
        body: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Composite::put_input(inner, input_name, input);

        match outer.for_mut_fn() {
            MutFnCtx::Free(_ctx) => Composite::transform(body_mode, inner, body),
            MutFnCtx::Const(ctx) => {
                let eval = |inner: &mut crate::Ctx| Composite::transform(body_mode, inner, body);
                Composite::with_ctx(inner, ctx.unwrap(), ctx_name, Invariant::Const, eval)
            }
            MutFnCtx::Mut(ctx) => {
                let eval = |inner: &mut crate::Ctx| Composite::transform(body_mode, inner, body);
                Composite::with_ctx(inner, ctx.unwrap(), ctx_name, Invariant::Final, eval)
            }
        }
    }
}
