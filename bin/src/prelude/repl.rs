use std::rc::Rc;

use airlang::{
    EvalMode,
    IoMode,
    MutableCtx,
    Symbol,
    Val,
};
use airlang_ext::{
    ExtFn,
    ExtFunc,
};

use crate::prelude::{
    put_func,
    Prelude,
};

pub(crate) struct ReplPrelude {
    pub(crate) exit: Rc<ExtFunc>,
}

impl Default for ReplPrelude {
    fn default() -> Self {
        Self { exit: exit() }
    }
}

impl Prelude for ReplPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        put_func(&self.exit, ctx.reborrow());
    }
}

fn exit() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("repl.exit") };
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_exit);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_exit(_input: Val) -> Val {
    std::process::exit(0)
}
