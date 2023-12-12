use {
    crate::{
        CtxForMutableFn,
        Val,
    },
    std::cell::RefCell,
};

pub trait Extension {
    fn call(&self, ctx: CtxForMutableFn, func: Val, input: Val) -> Val;
    fn reverse(&self, ctx: CtxForMutableFn, func: Val, output: Val) -> Val;
}

thread_local! (
    pub(crate) static EXTENSION: RefCell<Box<dyn Extension>> = RefCell::new(Box::new(DefaultExtension))
);

struct DefaultExtension;

impl Extension for DefaultExtension {
    fn call(&self, _ctx: CtxForMutableFn, _func: Val, _input: Val) -> Val {
        Val::default()
    }

    fn reverse(&self, _ctx: CtxForMutableFn, _func: Val, _output: Val) -> Val {
        Val::default()
    }
}
