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
        default_mode,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    problem::solve,
    val::func::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct ReversePrelude {
    pub(crate) reverse: Named<FuncVal>,
    pub(crate) pipe: Named<FuncVal>,
}

#[allow(clippy::derivable_impls)]
impl Default for ReversePrelude {
    fn default() -> Self {
        ReversePrelude {
            reverse: reverse(),
            pipe: pipe(),
        }
    }
}

impl Prelude for ReversePrelude {
    fn put(&self, m: &mut NameMap) {
        self.reverse.put(m);
        self.pipe.put(m);
    }
}

fn reverse() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
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

fn pipe() -> Named<FuncVal> {
    let input_mode = pair_mode(IoMode::Eval(EvalMode::Value), IoMode::Eval(EvalMode::Value));
    let output_mode = default_mode();
    named_mutable_fn("\\", input_mode, output_mode, fn_pipe)
}

fn fn_pipe(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    More.eval_reverse(&mut ctx, pair.second, pair.first)
}
