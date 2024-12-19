use crate::{
    ConstFnCtx,
    FuncMode,
    Invariant,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::{
            Composite,
            eval_aware,
            eval_free,
        },
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
        match ctx.for_const_fn() {
            ConstFnCtx::Free(_ctx) => eval_free(
                &mut self.comp.prelude.clone(),
                input,
                self.comp.input_name.clone(),
                &self.comp.body_mode,
                self.comp.body.clone(),
            ),
            ConstFnCtx::Const(mut ctx) => {
                let f = |ctx| {
                    eval_aware(
                        self.comp.prelude.clone(),
                        ctx,
                        self.ctx_name.clone(),
                        Invariant::Const,
                        input,
                        self.comp.input_name.clone(),
                        &self.comp.body_mode,
                        self.comp.body.clone(),
                    )
                };
                // INVARIANT: We use the const invariant to indicate not to modify this context.
                ctx.temp_take(f)
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
    pub(crate) fn new(comp: Composite, mode: FuncMode, cacheable: bool, ctx_name: Symbol) -> Self {
        Self {
            comp,
            mode,
            cacheable,
            ctx_name,
        }
    }
}
