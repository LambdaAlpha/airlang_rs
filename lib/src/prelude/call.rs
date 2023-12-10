use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    eval::{
        input::ByVal,
        Evaluator,
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
    pair::Pair,
    prelude::{
        named_mutable_fn,
        Named,
        Prelude,
    },
    val::func::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) call: Named<FuncVal>,
    pub(crate) chain: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            call: call(),
            chain: chain(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut NameMap) {
        self.call.put(m);
        self.chain.put(m);
    }
}

fn call() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_call::<FreeCtx>,
        |ctx, val| fn_call(ctx, val),
        |ctx, val| fn_call(ctx, val),
    );
    named_mutable_fn("$$", input_mode, output_mode, func)
}

fn fn_call<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Func(FuncVal(func)) = &pair.first else {
        return Val::default();
    };
    let input = func.input_mode.eval(&mut ctx, pair.second);
    func.eval(&mut ctx, input)
}

fn chain() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::Value),
        IoMode::Any(EvalMode::Value),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("$>", input_mode, output_mode, fn_chain)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    More.eval_call(&mut ctx, pair.second, pair.first)
}
