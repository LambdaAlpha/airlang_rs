use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    FuncMode,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        comp::Composite,
        prim::Primitive,
    },
    transformer::Transformer,
    val::func::{
        const_static_comp::ConstStaticCompFuncVal,
        const_static_prim::ConstStaticPrimFuncVal,
        free_cell_comp::FreeCellCompFuncVal,
        free_cell_prim::FreeCellPrimFuncVal,
        free_static_comp::FreeStaticCompFuncVal,
        free_static_prim::FreeStaticPrimFuncVal,
        mode::ModeFuncVal,
        mut_static_comp::MutStaticCompFuncVal,
        mut_static_prim::MutStaticPrimFuncVal,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    Mode(ModeFuncVal),
    FreeCellPrim(FreeCellPrimFuncVal),
    FreeCellComp(FreeCellCompFuncVal),
    FreeStaticPrim(FreeStaticPrimFuncVal),
    FreeStaticComp(FreeStaticCompFuncVal),
    ConstStaticPrim(ConstStaticPrimFuncVal),
    ConstStaticComp(ConstStaticCompFuncVal),
    MutStaticPrim(MutStaticPrimFuncVal),
    MutStaticComp(MutStaticCompFuncVal),
}

impl Transformer<Val, Val> for FuncVal {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Mode(f) => f.transform(ctx, input),
            FuncVal::FreeCellPrim(f) => f.transform(ctx, input),
            FuncVal::FreeCellComp(f) => f.transform(ctx, input),
            FuncVal::FreeStaticPrim(f) => f.transform(ctx, input),
            FuncVal::FreeStaticComp(f) => f.transform(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.transform(ctx, input),
            FuncVal::ConstStaticComp(f) => f.transform(ctx, input),
            FuncVal::MutStaticPrim(f) => f.transform(ctx, input),
            FuncVal::MutStaticComp(f) => f.transform(ctx, input),
        }
    }
}

impl FuncTrait for FuncVal {
    fn mode(&self) -> &FuncMode {
        match self {
            FuncVal::Mode(f) => f.mode(),
            FuncVal::FreeCellPrim(f) => f.mode(),
            FuncVal::FreeCellComp(f) => f.mode(),
            FuncVal::FreeStaticPrim(f) => f.mode(),
            FuncVal::FreeStaticComp(f) => f.mode(),
            FuncVal::ConstStaticPrim(f) => f.mode(),
            FuncVal::ConstStaticComp(f) => f.mode(),
            FuncVal::MutStaticPrim(f) => f.mode(),
            FuncVal::MutStaticComp(f) => f.mode(),
        }
    }

    fn cacheable(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.cacheable(),
            FuncVal::FreeCellPrim(f) => f.cacheable(),
            FuncVal::FreeCellComp(f) => f.cacheable(),
            FuncVal::FreeStaticPrim(f) => f.cacheable(),
            FuncVal::FreeStaticComp(f) => f.cacheable(),
            FuncVal::ConstStaticPrim(f) => f.cacheable(),
            FuncVal::ConstStaticComp(f) => f.cacheable(),
            FuncVal::MutStaticPrim(f) => f.cacheable(),
            FuncVal::MutStaticComp(f) => f.cacheable(),
        }
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Mode(f) => f.transform_mut(ctx, input),
            FuncVal::FreeCellPrim(f) => f.transform_mut(ctx, input),
            FuncVal::FreeCellComp(f) => f.transform_mut(ctx, input),
            FuncVal::FreeStaticPrim(f) => f.transform_mut(ctx, input),
            FuncVal::FreeStaticComp(f) => f.transform_mut(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.transform_mut(ctx, input),
            FuncVal::ConstStaticComp(f) => f.transform_mut(ctx, input),
            FuncVal::MutStaticPrim(f) => f.transform_mut(ctx, input),
            FuncVal::MutStaticComp(f) => f.transform_mut(ctx, input),
        }
    }
}

impl FuncVal {
    pub(crate) fn primitive(&self) -> Option<&Primitive> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::FreeCellPrim(f) => Some(&f.prim),
            FuncVal::FreeCellComp(_) => None,
            FuncVal::FreeStaticPrim(f) => Some(&f.prim),
            FuncVal::FreeStaticComp(_) => None,
            FuncVal::ConstStaticPrim(f) => Some(&f.prim),
            FuncVal::ConstStaticComp(_) => None,
            FuncVal::MutStaticPrim(f) => Some(&f.prim),
            FuncVal::MutStaticComp(_) => None,
        }
    }

    pub(crate) fn composite(&self) -> Option<&Composite> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::FreeCellPrim(_) => None,
            FuncVal::FreeCellComp(f) => Some(&f.comp),
            FuncVal::FreeStaticPrim(_) => None,
            FuncVal::FreeStaticComp(f) => Some(&f.comp),
            FuncVal::ConstStaticPrim(_) => None,
            FuncVal::ConstStaticComp(f) => Some(&f.comp),
            FuncVal::MutStaticPrim(_) => None,
            FuncVal::MutStaticComp(f) => Some(&f.comp),
        }
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.is_primitive(),
            FuncVal::FreeCellPrim(_) => true,
            FuncVal::FreeCellComp(_) => false,
            FuncVal::FreeStaticPrim(_) => true,
            FuncVal::FreeStaticComp(_) => false,
            FuncVal::ConstStaticPrim(_) => true,
            FuncVal::ConstStaticComp(_) => false,
            FuncVal::MutStaticPrim(_) => true,
            FuncVal::MutStaticComp(_) => false,
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncVal::Mode(f) => f.fmt(formatter),
            FuncVal::FreeCellPrim(f) => f.fmt(formatter),
            FuncVal::FreeCellComp(f) => f.fmt(formatter),
            FuncVal::FreeStaticPrim(f) => f.fmt(formatter),
            FuncVal::FreeStaticComp(f) => f.fmt(formatter),
            FuncVal::ConstStaticPrim(f) => f.fmt(formatter),
            FuncVal::ConstStaticComp(f) => f.fmt(formatter),
            FuncVal::MutStaticPrim(f) => f.fmt(formatter),
            FuncVal::MutStaticComp(f) => f.fmt(formatter),
        }
    }
}

pub(crate) mod mode;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;
