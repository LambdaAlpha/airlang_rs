use std::{
    fmt::Debug,
    hash::Hash,
};

use crate::{
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
    val::Val,
};

pub(crate) trait FuncTrait: Transformer<Val, Val> {
    fn mode(&self) -> &FuncMode;
    fn cacheable(&self) -> bool;

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.transform(ctx, input)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub call: Mode,
    pub abstract1: Mode,
    pub ask: Mode,
}

pub(crate) mod prim;

pub(crate) mod comp;

pub(crate) mod mode;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;

pub(crate) mod repr;
