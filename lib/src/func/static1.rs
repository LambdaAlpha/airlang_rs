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

pub trait StaticFn {
    fn call(&self, input: Val) -> Val;
}

#[derive(Clone)]
pub struct StaticPrimitiveExt {
    pub(crate) fn1: Rc<dyn StaticFn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StaticCompositeExt {}

pub type StaticFunc = Func<StaticPrimitiveExt, StaticCompositeExt>;

impl Transformer<Val, Val> for Primitive<StaticPrimitiveExt> {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.ext.fn1.call(input)
    }
}

impl Transformer<Val, Val> for Composite<StaticCompositeExt> {
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

impl StaticFunc {
    pub fn new(
        call_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        id: Symbol,
        fn1: Rc<dyn StaticFn>,
    ) -> Self {
        let transformer = FuncImpl::Primitive(Primitive {
            is_extension: true,
            id,
            ext: StaticPrimitiveExt { fn1 },
        });
        Self {
            call_mode,
            ask_mode,
            cacheable,
            transformer,
        }
    }
}

impl Primitive<StaticPrimitiveExt> {
    pub(crate) fn new(id: &str, f: impl StaticFn + 'static) -> Self {
        Primitive {
            is_extension: false,
            id: Symbol::from_str(id),
            ext: StaticPrimitiveExt { fn1: Rc::new(f) },
        }
    }
}

impl<T> StaticFn for T
where
    T: Fn(Val) -> Val,
{
    fn call(&self, input: Val) -> Val {
        self(input)
    }
}
