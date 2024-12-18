use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use cell::CellFuncVal;
use const1::ConstFuncVal;
use free::FreeFuncVal;
use mode::ModeFuncVal;
use mut1::MutFuncVal;

use crate::{
    Ctx,
    FuncMode,
    Mode,
    PrimitiveMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    transformer::Transformer,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    Mode(ModeFuncVal),
    Cell(CellFuncVal),
    Free(FreeFuncVal),
    Const(ConstFuncVal),
    Mut(MutFuncVal),
}

impl FuncVal {
    pub(crate) fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Mode(f) => f.transform(ctx, input),
            FuncVal::Cell(f) => f.transform(input),
            FuncVal::Free(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Mode(f) => f.transform(ctx, input),
            FuncVal::Cell(f) => f.transform_mut(input),
            FuncVal::Free(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn mode(&self) -> &FuncMode {
        match self {
            FuncVal::Mode(_) => &FuncMode {
                call: Mode::Primitive(PrimitiveMode::Id),
                abstract1: Mode::Primitive(PrimitiveMode::Eval),
                ask: Mode::Primitive(PrimitiveMode::Eval),
            },
            FuncVal::Cell(f) => &f.mode,
            FuncVal::Free(f) => &f.mode,
            FuncVal::Const(f) => &f.mode,
            FuncVal::Mut(f) => &f.mode,
        }
    }

    pub(crate) fn cacheable(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.cacheable(),
            FuncVal::Cell(f) => f.cacheable,
            FuncVal::Free(f) => f.cacheable,
            FuncVal::Const(f) => f.cacheable,
            FuncVal::Mut(f) => f.cacheable,
        }
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.is_primitive(),
            FuncVal::Cell(f) => f.is_primitive(),
            FuncVal::Free(f) => f.is_primitive(),
            FuncVal::Const(f) => f.is_primitive(),
            FuncVal::Mut(f) => f.is_primitive(),
        }
    }

    pub(crate) fn id(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.id(),
            FuncVal::Free(f) => f.id(),
            FuncVal::Const(f) => f.id(),
            FuncVal::Mut(f) => f.id(),
        }
    }

    pub(crate) fn is_extension(&self) -> Option<bool> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.is_extension(),
            FuncVal::Free(f) => f.is_extension(),
            FuncVal::Const(f) => f.is_extension(),
            FuncVal::Mut(f) => f.is_extension(),
        }
    }

    pub(crate) fn body_mode(&self) -> Option<&Mode> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.body_mode(),
            FuncVal::Free(f) => f.body_mode(),
            FuncVal::Const(f) => f.body_mode(),
            FuncVal::Mut(f) => f.body_mode(),
        }
    }

    pub(crate) fn body(&self) -> Option<&Val> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.body(),
            FuncVal::Free(f) => f.body(),
            FuncVal::Const(f) => f.body(),
            FuncVal::Mut(f) => f.body(),
        }
    }

    pub(crate) fn prelude(&self) -> Option<&Ctx> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.prelude(),
            FuncVal::Free(f) => f.prelude(),
            FuncVal::Const(f) => f.prelude(),
            FuncVal::Mut(f) => f.prelude(),
        }
    }

    pub(crate) fn input_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.input_name(),
            FuncVal::Free(f) => f.input_name(),
            FuncVal::Const(f) => f.input_name(),
            FuncVal::Mut(f) => f.input_name(),
        }
    }

    pub(crate) fn ctx_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(_) => None,
            FuncVal::Free(_) => None,
            FuncVal::Const(f) => f.ctx_name(),
            FuncVal::Mut(f) => f.ctx_name(),
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncVal::Mode(func) => ModeFuncVal::fmt(func, f),
            FuncVal::Cell(func) => CellFuncVal::fmt(func, f),
            FuncVal::Free(func) => FreeFuncVal::fmt(func, f),
            FuncVal::Const(func) => ConstFuncVal::fmt(func, f),
            FuncVal::Mut(func) => MutFuncVal::fmt(func, f),
        }
    }
}

pub(crate) mod mode;

pub(crate) mod cell;

pub(crate) mod free;

pub(crate) mod const1;

pub(crate) mod mut1;
