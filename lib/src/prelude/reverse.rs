use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        default_mode,
        named_mutable_fn,
        reverse_mode,
        Named,
        Prelude,
    },
    transform::eval::Eval,
    val::func::FuncVal,
    Val,
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
    Eval.eval_output_then_solve(&mut ctx, reverse.func, reverse.output)
}
