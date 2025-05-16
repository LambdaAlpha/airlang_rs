use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::Composite;
use crate::types::ref1::DynRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStaticCompFunc {
    pub(crate) comp: Composite,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        let inner = &mut self.comp.ctx.clone();
        let input_name = self.comp.input_name.clone();
        let body = self.comp.body.clone();
        Composite::free_transform(inner, input_name, input, body)
    }
}

impl FreeCellFn<Val, Val> for FreeStaticCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for FreeStaticCompFunc {
    fn opt_const_static_call(&self, _ctx: Option<ConstRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl ConstCellFn<Ctx, Val, Val> for FreeStaticCompFunc {
    fn const_cell_call(&mut self, _ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.free_static_call(input)
    }

    fn opt_const_cell_call(&mut self, _ctx: Option<ConstRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl MutStaticFn<Ctx, Val, Val> for FreeStaticCompFunc {
    fn dyn_static_call(&self, _ctx: DynRef<Ctx>, input: Val) -> Val {
        self.free_static_call(input)
    }

    fn opt_dyn_static_call(&self, _ctx: Option<DynRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl MutCellFn<Ctx, Val, Val> for FreeStaticCompFunc {
    fn mut_cell_call(&mut self, _ctx: &mut Ctx, input: Val) -> Val {
        self.free_static_call(input)
    }

    fn dyn_cell_call(&mut self, _ctx: DynRef<Ctx>, input: Val) -> Val {
        self.free_static_call(input)
    }

    fn opt_dyn_cell_call(&mut self, _ctx: Option<DynRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
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
