use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    eval_mode::{
        eager::Eager,
        EvalMode,
    },
    func::MutableDispatcher,
    io_mode::IoMode,
    prelude::{
        default_mode,
        named_mutable_fn,
        pair_mode,
        reverse_mode,
        Named,
        Prelude,
    },
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
    let input_mode = reverse_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_reverse::<FreeCtx>,
        |ctx, val| fn_reverse(ctx, val),
        |ctx, val| fn_reverse(ctx, val),
    );
    named_mutable_fn("??", input_mode, output_mode, func)
}

fn fn_reverse<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Reverse(reverse) = input else {
        return Val::default();
    };
    Eager.eval_output_then_solve(&mut ctx, reverse.func, reverse.output)
}

fn pipe() -> Named<FuncVal> {
    let input_mode = pair_mode(IoMode::Eval(EvalMode::Value), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("\\", input_mode, output_mode, fn_pipe)
}

fn fn_pipe(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eager.eval_output_then_solve(&mut ctx, pair.second, pair.first)
}
