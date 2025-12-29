use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;

use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

pub trait FreeFn<Cfg, I, O> {
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O;
}

impl<Cfg, I, O, T> FreeFn<Cfg, I, O> for &T
where T: FreeFn<Cfg, I, O>
{
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (**self).free_call(cfg, input)
    }
}

impl<Cfg, I, O, T> FreeFn<Cfg, I, O> for &mut T
where T: FreeFn<Cfg, I, O>
{
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (**self).free_call(cfg, input)
    }
}

#[derive(Clone)]
pub struct FreePrimFunc {
    pub(crate) raw_input: bool,
    pub(crate) fn_: Rc<dyn FreeFn<Cfg, Val, Val>>,
}

impl FreeFn<Cfg, Val, Val> for FreePrimFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.fn_.free_call(cfg, input)
    }
}

impl Default for FreePrimFunc {
    fn default() -> Self {
        struct F;
        impl FreeFn<Cfg, Val, Val> for F {
            fn free_call(&self, _cfg: &mut Cfg, _input: Val) -> Val {
                Val::default()
            }
        }
        Self { fn_: Rc::new(F), raw_input: false }
    }
}

impl PartialEq for FreePrimFunc {
    fn eq(&self, other: &FreePrimFunc) -> bool {
        self.raw_input == other.raw_input && Rc::ptr_eq(&self.fn_, &other.fn_)
    }
}

impl Eq for FreePrimFunc {}

impl Debug for FreePrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FreePrimFunc")
            .field("raw_input", &self.raw_input)
            .field("fn", &Rc::as_ptr(&self.fn_))
            .finish()
    }
}
