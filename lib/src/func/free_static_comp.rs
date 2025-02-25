use crate::{
    FuncMode,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::Composite,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for FreeStaticCompFunc {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        let inner = &mut self.comp.ctx.clone();
        if Composite::put_input(inner, self.comp.input_name.clone(), input).is_err() {
            return Val::default();
        }
        Composite::transform(inner, self.comp.body.clone())
    }
}

impl FuncTrait for FreeStaticCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn call(&self) -> Val {
        self.comp.func_call()
    }
}

impl FreeStaticCompFunc {
    pub(crate) fn new(comp: Composite, mode: FuncMode, cacheable: bool) -> Self {
        Self { comp, mode, cacheable }
    }
}
