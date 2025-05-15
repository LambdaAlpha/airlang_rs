use crate::FuncMode;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::func::comp::Composite;
use crate::transformer::Transformer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) mode: FuncMode,
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

    fn code(&self) -> Val {
        self.comp.func_code()
    }
}

impl FreeStaticCompFunc {
    pub(crate) fn new(comp: Composite, mode: FuncMode) -> Self {
        Self { comp, mode }
    }
}
