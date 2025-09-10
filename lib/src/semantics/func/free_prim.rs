use std::rc::Rc;

use super::prim::impl_prim_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Symbol;

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
    pub(crate) id: Symbol,
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
        Self { id: Symbol::default(), fn_: Rc::new(F) }
    }
}

impl_prim_func!(FreePrimFunc);
