use crate::ctx::ref1::CtxMeta;
use crate::func::func_mode::FuncMode;
use crate::transformer::Transformer;
use crate::val::Val;

pub(crate) trait FuncTrait: Transformer<Val, Val> {
    fn mode(&self) -> &FuncMode;

    fn call(&self) -> Val;

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.transform(ctx, input)
    }
}

pub(crate) mod func_mode;

pub(crate) mod prim;

pub(crate) mod comp;

pub(crate) mod mode;

pub(crate) mod ctx_aware_comp;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod const_cell_prim;

pub(crate) mod const_cell_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;

pub(crate) mod mut_cell_prim;

pub(crate) mod mut_cell_comp;

pub(crate) mod repr;
