use std::cell::RefCell;

use crate::{
    Prelude,
    PreludeCtx,
    Symbol,
    Val,
    traits::dyn_safe::dyn_any_clone_eq_hash,
    type1::{
        Type,
        TypeMeta,
    },
};

pub trait AirExt: Prelude + TypeMeta {}

thread_local!(pub(crate) static AIR_EXT: RefCell<Box<dyn AirExt>> = RefCell::new(Box::new(AirExtUnit)));

pub(crate) fn set_air_ext(prelude: Box<dyn AirExt>) {
    AIR_EXT.replace(prelude);
}

struct AirExtUnit;

impl Prelude for AirExtUnit {
    fn put(&self, _ctx: &mut dyn PreludeCtx) {}
}

impl TypeMeta for AirExtUnit {
    fn arbitrary(&self) -> Val {
        Val::default()
    }

    fn arbitrary_type(&self, _type1: Symbol) -> Val {
        Val::default()
    }
}

impl AirExt for AirExtUnit {}

dyn_any_clone_eq_hash!(pub ValExt : Type);
