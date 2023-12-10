use crate::{
    ctx::NameMap,
    ctx_access::mutable::CtxForMutableFn,
    eval_mode::EvalMode,
    extension::{
        CALL_EXTENSION,
        REVERSE_EXTENSION,
    },
    io_mode::IoMode,
    prelude::{
        named_mutable_fn,
        Named,
        Prelude,
    },
    types::Pair,
    FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct ExtPrelude {
    pub(crate) call: Named<FuncVal>,
    pub(crate) reverse: Named<FuncVal>,
}

impl Default for ExtPrelude {
    fn default() -> Self {
        ExtPrelude {
            call: call(),
            reverse: reverse(),
        }
    }
}

impl Prelude for ExtPrelude {
    fn put(&self, m: &mut NameMap) {
        self.call.put(m);
        self.reverse.put(m);
    }
}

fn call() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::Less),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("$", input_mode, output_mode, fn_call)
}

fn fn_call(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    CALL_EXTENSION.with(|f| (f.borrow())(ctx, pair.first, pair.second))
}

fn reverse() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::Less),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("?", input_mode, output_mode, fn_reverse)
}

fn fn_reverse(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    REVERSE_EXTENSION.with(|f| (f.borrow())(ctx, pair.first, pair.second))
}
