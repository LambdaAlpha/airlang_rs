use crate::{
    FuncMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::Composite,
        ctx_aware_comp::{
            func_call,
            mut_func_transform,
        },
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
        let body = self.comp.body.clone();
        mut_func_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl FuncTrait for MutCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn call(&self) -> Val {
        let ctx = self.ctx_name.clone();
        let input = self.comp.input_name.clone();
        let output = self.comp.body.clone();
        func_call(ctx, input, output)
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let inner = &mut self.comp.ctx;
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        mut_func_transform(inner, ctx_name, ctx, input_name, input, body)
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
}
