use {
    crate::{
        ctx::{
            ConstCtx,
            DynCtx,
        },
        eval::{
            Cmd,
            Output,
        },
        prelude::{
            named_fn,
            Named,
            Prelude,
        },
    },
    airlang::semantics::Val,
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

#[derive(Clone)]
pub(crate) struct ReplPrelude {
    pub(crate) exit: Named<Rc<dyn Cmd>>,
}

impl Default for ReplPrelude {
    fn default() -> Self {
        Self {
            exit: named_fn("repl.exit", exit),
        }
    }
}

impl Prelude for ReplPrelude {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>) {
        self.exit.put(m);
    }
}

fn exit(_const_ctx: &ConstCtx, _dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    Output::Break
}
