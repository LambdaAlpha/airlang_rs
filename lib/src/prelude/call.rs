use crate::{
    ctx::CtxMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        call_mode,
        default_mode,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    syntax::CALL_INFIX,
    transform::eval::Eval,
    val::func::FuncVal,
    Call,
    Pair,
    Val,
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) call: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            new: new(),
            call: call(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.call.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn(CALL_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.first, pair.second).into())
}

fn call() -> Named<FuncVal> {
    let input_mode = call_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_call::<FreeCtx>,
        |ctx, val| fn_call(ctx, val),
        |ctx, val| fn_call(ctx, val),
    );
    named_mutable_fn("!!", input_mode, output_mode, func)
}

fn fn_call<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Call(call) = input else {
        return Val::default();
    };
    let call = Call::from(call);
    Eval.eval_input_then_call(ctx, call.func, call.input)
}
