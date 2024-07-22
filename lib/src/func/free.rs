use std::rc::Rc;

use crate::{
    ctx::ref1::CtxMeta,
    func::{
        eval_free,
        Composed,
        Func,
        FuncImpl,
        Primitive,
    },
    transformer::Transformer,
    Mode,
    Symbol,
    Val,
};

pub trait FreeFn {
    fn call(&self, input: Val) -> Val;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FreeInfo {}

pub type FreeFunc = Func<Rc<dyn FreeFn>, FreeInfo>;

impl Transformer<Val, Val> for Primitive<Rc<dyn FreeFn>> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(input)
    }
}

impl Transformer<Val, Val> for Composed<FreeInfo> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        eval_free(
            self.prelude.clone(),
            input,
            self.input_name.clone(),
            self.body.clone(),
        )
    }
}

impl FreeFunc {
    pub fn new(
        input_mode: Mode,
        output_mode: Mode,
        cacheable: bool,
        id: Symbol,
        fn1: Rc<dyn FreeFn>,
    ) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            fn1,
        });
        Self {
            input_mode,
            output_mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<Rc<dyn FreeFn>> {
    pub(crate) fn new(id: &str, f: impl FreeFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            fn1: Rc::new(f),
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
