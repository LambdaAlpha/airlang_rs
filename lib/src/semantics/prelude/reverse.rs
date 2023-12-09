use crate::{
    semantics::{
        ctx::NameMap,
        ctx_access::{
            free::FreeCtx,
            CtxAccessor,
        },
        eval::{
            output::OutputBuilder,
            Evaluator,
            ValBuilder,
        },
        eval_mode::EvalMode,
        func::{
            CtxMutableFn,
            Primitive,
        },
        io_mode::IoMode,
        prelude::{
            named_mutable_fn,
            Named,
            Prelude,
        },
        problem::solve,
        FuncVal,
        Val,
    },
    types::Pair,
};

#[derive(Clone)]
pub(crate) struct ReversePrelude {
    pub(crate) reverse: Named<FuncVal>,
}

#[allow(clippy::derivable_impls)]
impl Default for ReversePrelude {
    fn default() -> Self {
        ReversePrelude { reverse: reverse() }
    }
}

impl Prelude for ReversePrelude {
    fn put(&self, m: &mut NameMap) {
        self.reverse.put(m);
    }
}

fn reverse() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_reverse::<FreeCtx>,
        |ctx, val| fn_reverse(ctx, val),
        |ctx, val| fn_reverse(ctx, val),
    );
    named_mutable_fn("?", input_mode, output_mode, func)
}

fn fn_reverse<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Func(FuncVal(func)) = &pair.first else {
        return Val::default();
    };
    let output = func.output_mode.eval(&mut ctx, pair.second);
    let reverse = ValBuilder.from_reverse(pair.first, output);
    solve(&mut ctx, reverse)
}
