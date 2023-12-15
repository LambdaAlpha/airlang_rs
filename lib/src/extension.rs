use {
    crate::{
        CtxForMutableFn,
        Val,
    },
    std::cell::RefCell,
};

pub trait Extension {
    fn call(&mut self, ctx: CtxForMutableFn, func: Val, input: Val) -> Val;
    fn reverse(&mut self, ctx: CtxForMutableFn, func: Val, output: Val) -> Val;
}

thread_local! (
    pub(crate) static EXTENSION: RefCell<Box<dyn Extension>> = RefCell::new(Box::new(DefaultExtension))
);

struct DefaultExtension;

impl Extension for DefaultExtension {
    fn call(&mut self, _ctx: CtxForMutableFn, _func: Val, _input: Val) -> Val {
        Val::default()
    }

    fn reverse(&mut self, _ctx: CtxForMutableFn, _func: Val, _output: Val) -> Val {
        Val::default()
    }
}
