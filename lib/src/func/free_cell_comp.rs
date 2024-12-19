use crate::{
    FuncMode,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::{
            Composite,
            eval_free,
        },
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCellCompFunc {
    pub(crate) comp: Composite,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for FreeCellCompFunc {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        eval_free(
            &mut self.comp.prelude.clone(),
            input,
            self.comp.input_name.clone(),
            &self.comp.body_mode,
            self.comp.body.clone(),
        )
    }
}

impl FuncTrait for FreeCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn transform_mut<'a, Ctx>(&mut self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        eval_free(
            &mut self.comp.prelude,
            input,
            self.comp.input_name.clone(),
            &self.comp.body_mode,
            self.comp.body.clone(),
        )
    }
}

impl FreeCellCompFunc {
    pub(crate) fn new(comp: Composite, mode: FuncMode, cacheable: bool) -> Self {
        Self {
            comp,
            mode,
            cacheable,
        }
    }
}
