use crate::{
    ctx::CtxMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        ask_mode,
        default_mode,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    syntax::ASK_INFIX,
    transform::eval::Eval,
    val::func::FuncVal,
    Ask,
    Val,
};

#[derive(Clone)]
pub(crate) struct AskPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) ask: Named<FuncVal>,
}

impl Default for AskPrelude {
    fn default() -> Self {
        AskPrelude {
            new: new(),
            ask: ask(),
        }
    }
}

impl Prelude for AskPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.ask.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn(ASK_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Ask(Box::new(Ask::new(pair.first, pair.second)))
}

fn ask() -> Named<FuncVal> {
    let input_mode = ask_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_ask::<FreeCtx>,
        |ctx, val| fn_ask(ctx, val),
        |ctx, val| fn_ask(ctx, val),
    );
    named_mutable_fn("??", input_mode, output_mode, func)
}

fn fn_ask<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Ask(ask) = input else {
        return Val::default();
    };
    Eval.eval_output_then_solve(ctx, ask.func, ask.output)
}
