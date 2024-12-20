use crate::{
    ConstFnCtx,
    FuncMode,
    Invariant,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::Composite,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) ctx_name: Symbol,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for ConstStaticCompFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let inner = &mut self.comp.ctx.clone();
        Composite::put_input(inner, self.comp.input_name.clone(), input);

        match ctx.for_const_fn() {
            ConstFnCtx::Free(_ctx) => {
                Composite::transform(&self.comp.body_mode, inner, self.comp.body.clone())
            }
            ConstFnCtx::Const(ctx) => {
                let eval = |inner: &mut crate::Ctx| {
                    Composite::transform(&self.comp.body_mode, inner, self.comp.body.clone())
                };
                let name = self.ctx_name.clone();
                Composite::with_ctx(inner, ctx.unwrap(), name, Invariant::Const, eval)
            }
        }
    }
}

impl FuncTrait for ConstStaticCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }
}

impl ConstStaticCompFunc {
    pub(crate) fn new(comp: Composite, ctx_name: Symbol, mode: FuncMode, cacheable: bool) -> Self {
        Self {
            comp,
            mode,
            cacheable,
            ctx_name,
        }
    }
}
