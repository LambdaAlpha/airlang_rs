use std::{
    fmt::{
        Debug,
        DebugStruct,
    },
    hash::Hash,
};

use crate::{
    Mode,
    Symbol,
    Val,
    ext,
    func::{
        Composite,
        Func,
        FuncImpl,
        Primitive,
        eval_free,
    },
};

pub trait CellFn {
    fn call_mut(&mut self, input: Val) -> Val;
}

ext!(pub CellFnExt : CellFn);

pub type CellFunc = Func<CellPrimitiveExt, CellCompositeExt>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CellPrimitiveExt {
    pub(crate) fn1: Box<dyn CellFnExt>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CellCompositeExt {}

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

impl Primitive<CellPrimitiveExt> {
    fn transform_mut(&mut self, input: Val) -> Val {
        self.ext.fn1.call_mut(input)
    }

    fn transform(&self, input: Val) -> Val {
        self.ext.fn1.dyn_clone().call_mut(input)
    }
}

impl Composite<CellCompositeExt> {
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
    pub fn new(
        call_mode: Mode,
        abstract_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        id: Symbol,
        fn1: Box<dyn CellFnExt>,
    ) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: CellPrimitiveExt { fn1 },
        });
        Self {
            call_mode,
            abstract_mode,
            ask_mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<CellPrimitiveExt> {
    pub(crate) fn new(id: &str, f: impl CellFnExt + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: CellPrimitiveExt { fn1: Box::new(f) },
        }
    }

    pub(crate) fn dbg_field_ext(&self, s: &mut DebugStruct) {
        s.field("fn", &self.ext.fn1);
    }
}
