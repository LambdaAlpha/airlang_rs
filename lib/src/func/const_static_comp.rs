use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::DynRef;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::Composite;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) ctx_name: Symbol,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for ConstStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::free_transform(inner, input_name, input, body)
    }
}

impl FreeCellFn<Val, Val> for ConstStaticCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for ConstStaticCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let ctx_name = self.ctx_name.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::const_transform(inner, ctx_name, ctx, input_name, input, body)
    }
}

impl ConstCellFn<Ctx, Val, Val> for ConstStaticCompFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, Val, Val> for ConstStaticCompFunc {
    fn dyn_static_call(&self, ctx: DynRef<Ctx>, input: Val) -> Val {
        self.const_static_call(ctx.into_const(), input)
    }
}

impl MutCellFn<Ctx, Val, Val> for ConstStaticCompFunc {
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: Val) -> Val {
        self.const_static_call(ConstRef::new(ctx), input)
    }

    fn dyn_cell_call(&mut self, ctx: DynRef<Ctx>, input: Val) -> Val {
        self.const_static_call(ctx.into_const(), input)
    }
}

impl FuncTrait for ConstStaticCompFunc {
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

impl ConstStaticCompFunc {
    pub(crate) fn new(comp: Composite, ctx_name: Symbol, mode: FuncMode) -> Self {
        Self { comp, mode, ctx_name }
    }
}
