use std::rc::Rc;

use crate::{
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        Func,
        FuncImpl,
        FuncMode,
        comp::Composite,
        eval_free,
        prim::Primitive,
    },
    transformer::Transformer,
};

pub trait FreeFn {
    fn call(&self, input: Val) -> Val;
}

#[derive(Clone)]
pub struct FreePrimExt {
    pub(crate) fn1: Rc<dyn FreeFn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FreeCompExt {}

pub type FreeFunc = Func<FreePrimExt, FreeCompExt>;

impl Transformer<Val, Val> for Primitive<FreePrimExt> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.ext.fn1.call(input)
    }
}

impl Transformer<Val, Val> for Composite<FreeCompExt> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        eval_free(
            &mut self.prelude.clone(),
            input,
            self.input_name.clone(),
            &self.body_mode,
            self.body.clone(),
        )
    }
}

impl FreeFunc {
    pub fn new(mode: FuncMode, cacheable: bool, id: Symbol, fn1: Rc<dyn FreeFn>) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: FreePrimExt { fn1 },
        });
        Self {
            mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<FreePrimExt> {
    pub(crate) fn new(id: &str, f: impl FreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: FreePrimExt { fn1: Rc::new(f) },
        }
    }
}

impl<T> FreeFn for T
where
    T: Fn(Val) -> Val,
{
    fn call(&self, input: Val) -> Val {
        self(input)
    }
}
