use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
    },
    eval::{
        Evaluator,
        EAGER,
        ID,
        LAZY,
    },
    eval_mode::{
        eager::Eager,
        id::Id,
        lazy::Lazy,
        EvalMode,
    },
    io_mode::IoMode,
    prelude::{
        default_mode,
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct EvalPrelude {
    pub(crate) id: Named<FuncVal>,
    pub(crate) lazy: Named<FuncVal>,
    pub(crate) eager: Named<FuncVal>,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        EvalPrelude {
            id: id(),
            lazy: lazy(),
            eager: eager(),
        }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, m: &mut NameMap) {
        self.id.put(m);
        self.lazy.put(m);
        self.eager.put(m);
    }
}

fn id() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Id);
    let output_mode = default_mode();
    named_free_fn(ID, input_mode, output_mode, fn_id)
}

fn fn_id(input: Val) -> Val {
    Id.eval(&mut FreeCtx, input)
}

fn lazy() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Id);
    let output_mode = default_mode();
    named_mutable_fn(LAZY, input_mode, output_mode, fn_lazy)
}

fn fn_lazy(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Lazy.eval(&mut ctx, input)
}

fn eager() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Id);
    let output_mode = default_mode();
    named_mutable_fn(EAGER, input_mode, output_mode, fn_eager)
}

fn fn_eager(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eager.eval(&mut ctx, input)
}
