use {
    crate::{
        ctx::{
            ConstCtx,
            DynCtx,
        },
        eval::{
            eval_interpret,
            Cmd,
            Output,
        },
        prelude::{
            named_fn,
            Named,
            Prelude,
        },
    },
    airlang::{
        initial_ctx,
        Val,
    },
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

#[derive(Clone)]
pub(crate) struct EvalPrelude {
    pub(crate) eval: Named<Rc<dyn Cmd>>,
    pub(crate) meta_eval: Named<Rc<dyn Cmd>>,
    pub(crate) reset: Named<Rc<dyn Cmd>>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        Self {
            eval: named_fn("repl.eval", eval),
            meta_eval: named_fn("repl.meta", repl_eval),
            reset: named_fn("repl.reset", reset),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>) {
        self.eval.put(m);
        self.meta_eval.put(m);
        self.reset.put(m);
    }
}

fn eval(_const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, input: Val) -> Output {
    eval_interpret(&mut dyn_ctx.ctx, input)
}

fn repl_eval(_const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, input: Val) -> Output {
    eval_interpret(&mut dyn_ctx.meta_ctx, input)
}

pub(crate) fn reset(_const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    dyn_ctx.ctx = initial_ctx();
    Output::Print(Box::new("context reset"))
}
