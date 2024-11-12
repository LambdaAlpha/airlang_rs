use std::rc::Rc;

use crate::{
    Mode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        Composite,
        Func,
        FuncImpl,
        Primitive,
        eval_free,
    },
    transformer::Transformer,
};

pub trait FreeFn {
    fn call(&self, input: Val) -> Val;
}

#[derive(Clone)]
pub struct FreePrimitiveExt {
    pub(crate) fn1: Rc<dyn FreeFn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FreeCompositeExt {}

pub type FreeFunc = Func<FreePrimitiveExt, FreeCompositeExt>;

impl Transformer<Val, Val> for Primitive<FreePrimitiveExt> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.ext.fn1.call(input)
    }
}

impl Transformer<Val, Val> for Composite<FreeCompositeExt> {
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
    pub fn new(
        call_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        id: Symbol,
        fn1: Rc<dyn FreeFn>,
    ) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: FreePrimitiveExt { fn1 },
        });
        Self {
            call_mode,
            ask_mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<FreePrimitiveExt> {
    pub(crate) fn new(id: &str, f: impl FreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: FreePrimitiveExt { fn1: Rc::new(f) },
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
