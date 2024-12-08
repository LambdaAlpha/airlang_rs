use std::{
    any::Any,
    fmt::{
        Debug,
        DebugStruct,
    },
    hash::{
        Hash,
        Hasher,
    },
};

use crate::{
    Mode,
    Symbol,
    Val,
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

pub trait CellFnExt: CellFn + Debug {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn CellFnExt) -> bool;
    fn dyn_clone(&self) -> Box<dyn CellFnExt>;
    fn dyn_hash(&self, hasher: &mut dyn Hasher);
}

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

impl<T: CellFn + Debug + Any + Eq + Clone + Hash> CellFnExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn CellFnExt) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn dyn_clone(&self) -> Box<dyn CellFnExt> {
        Box::new(self.clone())
    }

    fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher);
    }
}

impl Clone for Box<dyn CellFnExt> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

impl PartialEq for dyn CellFnExt {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

// https://github.com/rust-lang/rust/issues/31740
impl PartialEq<&Self> for Box<dyn CellFnExt> {
    fn eq(&self, other: &&Self) -> bool {
        <Self as PartialEq>::eq(self, *other)
    }
}

impl Eq for dyn CellFnExt {}

impl Hash for dyn CellFnExt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}
