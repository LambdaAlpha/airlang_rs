use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::func::comp::Composite;
use crate::func::ctx_aware_comp::func_call;
use crate::func::ctx_aware_comp::mut_func_transform;
use crate::transformer::Transformer;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) ctx_name: Symbol,
    pub(crate) mode: FuncMode,
}

impl Transformer<Val, Val> for MutStaticCompFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        let inner = &mut self.comp.ctx.clone();
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        mut_func_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl FuncTrait for MutStaticCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn call(&self) -> Val {
        let ctx = self.ctx_name.clone();
        let input = self.comp.input_name.clone();
        let output = self.comp.body.clone();
        func_call(ctx, input, output)
    }
}

impl MutStaticCompFunc {
    pub(crate) fn new(comp: Composite, ctx_name: Symbol, mode: FuncMode) -> Self {
        Self { comp, mode, ctx_name }
    }
}
