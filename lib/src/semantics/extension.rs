use {
    crate::semantics::{
        ctx_access::mutable::CtxForMutableFn,
        Val,
    },
    std::cell::RefCell,
};

pub(crate) type CallExtension = Box<dyn Fn(CtxForMutableFn, Val, Val) -> Val>;
pub(crate) type ReverseExtension = Box<dyn Fn(CtxForMutableFn, Val, Val) -> Val>;

thread_local! (
    pub(crate) static CALL_EXTENSION: RefCell<CallExtension> = RefCell::new(Box::new(fn_default_call))
);

thread_local! (
    pub(crate) static REVERSE_EXTENSION: RefCell<ReverseExtension> = RefCell::new(Box::new(fn_default_reverse))
);

fn fn_default_call(_ctx: CtxForMutableFn, _func: Val, _input: Val) -> Val {
    Val::default()
}

fn fn_default_reverse(_ctx: CtxForMutableFn, _func: Val, _output: Val) -> Val {
    Val::default()
}
