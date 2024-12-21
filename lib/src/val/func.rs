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
        const_cell_comp::ConstCellCompFuncVal,
        const_cell_prim::ConstCellPrimFuncVal,
        const_static_comp::ConstStaticCompFuncVal,
        const_static_prim::ConstStaticPrimFuncVal,
        free_cell_comp::FreeCellCompFuncVal,
        free_cell_prim::FreeCellPrimFuncVal,
        free_static_comp::FreeStaticCompFuncVal,
        free_static_prim::FreeStaticPrimFuncVal,
        mode::ModeFuncVal,
        mut_cell_comp::MutCellCompFuncVal,
        mut_cell_prim::MutCellPrimFuncVal,
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
    ConstCellPrim(ConstCellPrimFuncVal),
    ConstCellComp(ConstCellCompFuncVal),
    ConstStaticPrim(ConstStaticPrimFuncVal),
    ConstStaticComp(ConstStaticCompFuncVal),
    MutCellPrim(MutCellPrimFuncVal),
    MutCellComp(MutCellCompFuncVal),
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
            FuncVal::ConstCellPrim(f) => f.transform(ctx, input),
            FuncVal::ConstCellComp(f) => f.transform(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.transform(ctx, input),
            FuncVal::ConstStaticComp(f) => f.transform(ctx, input),
            FuncVal::MutCellPrim(f) => f.transform(ctx, input),
            FuncVal::MutCellComp(f) => f.transform(ctx, input),
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
            FuncVal::ConstCellPrim(f) => f.mode(),
            FuncVal::ConstCellComp(f) => f.mode(),
            FuncVal::ConstStaticPrim(f) => f.mode(),
            FuncVal::ConstStaticComp(f) => f.mode(),
            FuncVal::MutCellPrim(f) => f.mode(),
            FuncVal::MutCellComp(f) => f.mode(),
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
            FuncVal::ConstCellPrim(f) => f.cacheable(),
            FuncVal::ConstCellComp(f) => f.cacheable(),
            FuncVal::ConstStaticPrim(f) => f.cacheable(),
            FuncVal::ConstStaticComp(f) => f.cacheable(),
            FuncVal::MutCellPrim(f) => f.cacheable(),
            FuncVal::MutCellComp(f) => f.cacheable(),
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
            FuncVal::ConstCellPrim(f) => f.transform_mut(ctx, input),
            FuncVal::ConstCellComp(f) => f.transform_mut(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.transform_mut(ctx, input),
            FuncVal::ConstStaticComp(f) => f.transform_mut(ctx, input),
            FuncVal::MutCellPrim(f) => f.transform_mut(ctx, input),
            FuncVal::MutCellComp(f) => f.transform_mut(ctx, input),
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
            FuncVal::ConstCellPrim(f) => Some(&f.prim),
            FuncVal::ConstCellComp(_) => None,
            FuncVal::ConstStaticPrim(f) => Some(&f.prim),
            FuncVal::ConstStaticComp(_) => None,
            FuncVal::MutCellPrim(f) => Some(&f.prim),
            FuncVal::MutCellComp(_) => None,
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
            FuncVal::ConstCellPrim(_) => None,
            FuncVal::ConstCellComp(f) => Some(&f.comp),
            FuncVal::ConstStaticPrim(_) => None,
            FuncVal::ConstStaticComp(f) => Some(&f.comp),
            FuncVal::MutCellPrim(_) => None,
            FuncVal::MutCellComp(f) => Some(&f.comp),
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
            FuncVal::ConstCellPrim(_) => true,
            FuncVal::ConstCellComp(_) => false,
            FuncVal::ConstStaticPrim(_) => true,
            FuncVal::ConstStaticComp(_) => false,
            FuncVal::MutCellPrim(_) => true,
            FuncVal::MutCellComp(_) => false,
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
            FuncVal::ConstCellPrim(f) => f.fmt(formatter),
            FuncVal::ConstCellComp(f) => f.fmt(formatter),
            FuncVal::ConstStaticPrim(f) => f.fmt(formatter),
            FuncVal::ConstStaticComp(f) => f.fmt(formatter),
            FuncVal::MutCellPrim(f) => f.fmt(formatter),
            FuncVal::MutCellComp(f) => f.fmt(formatter),
            FuncVal::MutStaticPrim(f) => f.fmt(formatter),
            FuncVal::MutStaticComp(f) => f.fmt(formatter),
        }
    }
}

// improve performance by redirecting transform_mut to transform
// to avoid calling deref_mut, which calls Rc::make_mut
macro_rules! impl_const_func_trait {
    ($type1:ty) => {
        impl $crate::transformer::Transformer<$crate::val::Val, $crate::val::Val> for $type1 {
            fn transform<'a, Ctx>(&self, ctx: Ctx, input: $crate::val::Val) -> $crate::val::Val
            where
                Ctx: $crate::ctx::ref1::CtxMeta<'a>,
            {
                self.0.transform(ctx, input)
            }
        }

        impl $crate::func::FuncTrait for $type1 {
            fn mode(&self) -> &$crate::func::FuncMode {
                self.0.mode()
            }

            fn cacheable(&self) -> bool {
                self.0.cacheable()
            }

            fn transform_mut<'a, Ctx>(
                &mut self,
                ctx: Ctx,
                input: $crate::val::Val,
            ) -> $crate::val::Val
            where
                Ctx: $crate::ctx::ref1::CtxMeta<'a>,
            {
                use $crate::transformer::Transformer;
                self.0.transform(ctx, input)
            }
        }
    };
}

#[allow(unused)]
pub(crate) use impl_const_func_trait;

pub(crate) mod mode;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod const_cell_prim;

pub(crate) mod const_cell_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod mut_cell_prim;

pub(crate) mod mut_cell_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;
