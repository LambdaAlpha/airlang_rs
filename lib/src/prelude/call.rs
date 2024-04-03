use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        call_mode,
        default_mode,
        named_mutable_fn,
        Named,
        Prelude,
    },
    transform::eval::Eval,
    val::func::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) call: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude { call: call() }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut NameMap) {
        self.call.put(m);
    }
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

fn fn_call<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    Eval.eval_input_then_call(&mut ctx, call.func, call.input)
}
