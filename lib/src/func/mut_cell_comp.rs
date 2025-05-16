use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::Composite;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutCellCompFunc {
    pub(crate) comp: Composite,
    pub(crate) ctx_name: Symbol,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for MutCellCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::free_transform(inner, input_name, input, body)
    }
}

impl FreeCellFn<Val, Val> for MutCellCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        let inner = &mut self.comp.ctx;
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::free_transform(inner, input_name, input, body)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for MutCellCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::const_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl ConstCellFn<Ctx, Val, Val> for MutCellCompFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        let inner = &mut self.comp.ctx;
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::const_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl MutStaticFn<Ctx, Val, Val> for MutCellCompFunc {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::mut_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl MutCellFn<Ctx, Val, Val> for MutCellCompFunc {
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: Val) -> Val {
        let inner = &mut self.comp.ctx;
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::mut_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl FuncTrait for MutCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        let ctx = self.ctx_name.clone();
        let input = self.comp.input_name.clone();
        let output = self.comp.body.clone();
        Composite::ctx_aware_func_code(ctx, input, output)
    }
}

impl MutCellCompFunc {
    pub(crate) fn new(comp: Composite, ctx_name: Symbol, mode: FuncMode) -> Self {
        Self { comp, ctx_name, mode }
    }
}
