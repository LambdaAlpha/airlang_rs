use std::{
    any::Any,
    fmt::Debug,
    hash::{
        Hash,
        Hasher,
    },
};

use crate::{
    CtxForMutableFn,
    IoMode,
    Val,
};

pub trait ValExt: Debug + AsFuncExt {
    fn as_any(&self) -> &dyn Any;

    fn dyn_eq(&self, other: &dyn ValExt) -> bool;
    fn dyn_clone(&self) -> Box<dyn ValExt>;
    fn dyn_hash(&self, hasher: &mut dyn Hasher);
}

pub trait AsFuncExt {
    fn as_func(&self) -> Option<&dyn FuncExt>;
}

pub trait FuncExt {
    fn input_mode(&self) -> &IoMode;
    fn output_mode(&self) -> &IoMode;
    fn call(&self, ctx: CtxForMutableFn, input: Val) -> Val;
}

impl<T: Any + Eq + Clone + Hash + Debug + AsFuncExt> ValExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn ValExt) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn dyn_clone(&self) -> Box<dyn ValExt> {
        Box::new(self.clone())
    }

    fn dyn_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher);
    }
}

impl Clone for Box<dyn ValExt> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

impl PartialEq for dyn ValExt {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

// https://github.com/rust-lang/rust/issues/31740
impl PartialEq<&Self> for Box<dyn ValExt> {
    fn eq(&self, other: &&Self) -> bool {
        <Self as PartialEq>::eq(self, *other)
    }
}

impl Eq for dyn ValExt {}

impl Hash for dyn ValExt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct UnitExt;

impl AsFuncExt for UnitExt {
    fn as_func(&self) -> Option<&dyn FuncExt> {
        None
    }
}
