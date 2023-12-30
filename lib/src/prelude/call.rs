use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
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
        call_mode,
        default_mode,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    val::func::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) call: Named<FuncVal>,
    pub(crate) pipe: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            call: call(),
            pipe: pipe(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut NameMap) {
        self.call.put(m);
        self.pipe.put(m);
    }
}

fn call() -> Named<FuncVal> {
    let input_mode = call_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_call::<FreeCtx>,
        |ctx, val| fn_call(ctx, val),
        |ctx, val| fn_call(ctx, val),
    );
    named_mutable_fn("$$", input_mode, output_mode, func)
}

fn fn_call<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    More.eval_input_then_call(&mut ctx, call.func, call.input)
}

fn pipe() -> Named<FuncVal> {
    let input_mode = pair_mode(IoMode::Eval(EvalMode::Value), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("|", input_mode, output_mode, fn_pipe)
}

fn fn_pipe(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    More.eval_input_then_call(&mut ctx, pair.second, pair.first)
}
