use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    eval::{
        input::ByVal,
        output::OutputBuilder,
        Evaluator,
        ValBuilder,
    },
    eval_mode::{
        more::More,
        EvalMode,
    },
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
    types::Pair,
    val::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct ReversePrelude {
    pub(crate) reverse: Named<FuncVal>,
    pub(crate) chain: Named<FuncVal>,
}

#[allow(clippy::derivable_impls)]
impl Default for ReversePrelude {
    fn default() -> Self {
        ReversePrelude {
            reverse: reverse(),
            chain: chain(),
        }
    }
}

impl Prelude for ReversePrelude {
    fn put(&self, m: &mut NameMap) {
        self.reverse.put(m);
        self.chain.put(m);
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
    named_mutable_fn("??", input_mode, output_mode, func)
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

fn chain() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::Value),
        IoMode::Any(EvalMode::Value),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("?>", input_mode, output_mode, fn_chain)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    More.eval_reverse(&mut ctx, pair.second, pair.first)
}
