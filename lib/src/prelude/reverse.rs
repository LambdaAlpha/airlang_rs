use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        default_mode,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        reverse_mode,
        Named,
        Prelude,
    },
    syntax::REVERSE_INFIX,
    transform::eval::Eval,
    val::func::FuncVal,
    Reverse,
    Val,
};

#[derive(Clone)]
pub(crate) struct ReversePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) reverse: Named<FuncVal>,
}

#[allow(clippy::derivable_impls)]
impl Default for ReversePrelude {
    fn default() -> Self {
        ReversePrelude {
            new: new(),
            reverse: reverse(),
        }
    }
}

impl Prelude for ReversePrelude {
    fn put(&self, m: &mut NameMap) {
        self.new.put(m);
        self.reverse.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn(REVERSE_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Reverse(Box::new(Reverse::new(pair.first, pair.second)))
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
