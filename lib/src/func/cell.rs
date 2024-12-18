use std::{
    fmt::{
        Debug,
        DebugStruct,
    },
    hash::Hash,
};

use crate::{
    Symbol,
    Val,
    ext,
    func::{
        Func,
        FuncImpl,
        FuncMode,
        comp::Composite,
        eval_free,
        prim::Primitive,
    },
};

pub trait CellFn {
    fn call_mut(&mut self, input: Val) -> Val;
}

ext!(pub CellFnExt : CellFn);

pub type CellFunc = Func<CellPrimExt, CellCompExt>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CellPrimExt {
    pub(crate) fn1: Box<dyn CellFnExt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CellCompExt {}

impl CellFunc {
    pub(crate) fn transform_mut(&mut self, input: Val) -> Val {
        match &mut self.transformer {
            FuncImpl::Primitive(f) => f.transform_mut(input),
            FuncImpl::Composite(f) => f.transform_mut(input),
        }
    }

    pub(crate) fn transform(&self, input: Val) -> Val {
        match &self.transformer {
            FuncImpl::Primitive(f) => f.transform(input),
            FuncImpl::Composite(f) => f.transform(input),
        }
    }
}

impl Primitive<CellPrimExt> {
    fn transform_mut(&mut self, input: Val) -> Val {
        self.ext.fn1.call_mut(input)
    }

    fn transform(&self, input: Val) -> Val {
        self.ext.fn1.dyn_clone().call_mut(input)
    }
}

impl Composite<CellCompExt> {
    fn transform_mut(&mut self, input: Val) -> Val {
        eval_free(
            &mut self.prelude,
            input,
            self.input_name.clone(),
            &self.body_mode,
            self.body.clone(),
        )
    }

    fn transform(&self, input: Val) -> Val {
        eval_free(
            &mut self.prelude.clone(),
            input,
            self.input_name.clone(),
            &self.body_mode,
            self.body.clone(),
        )
    }
}

impl CellFunc {
    pub fn new(mode: FuncMode, cacheable: bool, id: Symbol, fn1: Box<dyn CellFnExt>) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: CellPrimExt { fn1 },
        });
        Self {
            mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<CellPrimExt> {
    pub(crate) fn new(id: &str, f: impl CellFnExt + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: CellPrimExt { fn1: Box::new(f) },
        }
    }

    pub(crate) fn dbg_field_ext(&self, s: &mut DebugStruct) {
        s.field("fn", &self.ext.fn1);
    }
}
