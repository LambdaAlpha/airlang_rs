use {
    crate::ctx::{
        ConstCtx,
        DynCtx,
    },
    airlang::{
        generate,
        interpret,
        Ctx,
        Val,
    },
    std::{
        error::Error,
        fmt::Display,
    },
};

pub(crate) trait Cmd = Fn(&ConstCtx, &mut DynCtx, Val) -> Output;

pub(crate) enum Output {
    Break,
    Print(Box<dyn Display>),
    Eprint(Box<dyn Error>),
}

pub(crate) fn eval(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, input: Val) -> Output {
    let Val::Call(call) = input else {
        return eval_interpret(&mut dyn_ctx.ctx, input);
    };
    let Val::Symbol(func) = &call.func else {
        return eval_interpret(&mut dyn_ctx.ctx, Val::Call(call));
    };
    let Some(func) = const_ctx.cmd_map.get(&**func) else {
        return eval_interpret(&mut dyn_ctx.ctx, Val::Call(call));
    };
    func(const_ctx, dyn_ctx, call.input)
}

pub(crate) fn eval_interpret(ctx: &mut Ctx, val: Val) -> Output {
    let output = interpret(ctx, val);
    match generate(&output) {
        Ok(output) => Output::Print(Box::new(output)),
        Err(err) => Output::Eprint(Box::new(err)),
    }
}
