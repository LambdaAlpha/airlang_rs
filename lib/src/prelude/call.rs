use std::mem::swap;

use crate::{
    ctx::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        ref1::CtxMeta,
        CtxMap,
        DefaultCtx,
    },
    func::MutableDispatcher,
    prelude::{
        default_mode,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        symbol_id_mode,
        Named,
        Prelude,
    },
    syntax::CALL_INFIX,
    transform::eval::Eval,
    types::either::Either,
    val::func::FuncVal,
    Call,
    CallMode,
    FreeCtx,
    Mode,
    Pair,
    Transform,
    Val,
    ValMode,
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) new_dependent: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_input: Named<FuncVal>,
    pub(crate) set_input: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            new: new(),
            new_dependent: new_dependent(),
            apply: apply(),
            get_func: get_func(),
            set_func: set_func(),
            get_input: get_input(),
            set_input: set_input(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.new_dependent.put(m);
        self.apply.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
        self.get_input.put(m);
        self.set_input.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let val_mode = ValMode {
        pair: Box::new(Pair::new(default_mode(), default_mode())),
        call: Box::new(CallMode::Struct(Call::new(default_mode(), default_mode()))),
        ..Default::default()
    };
    let input_mode = Mode::Custom(Box::new(val_mode));
    let output_mode = default_mode();
    named_free_fn(CALL_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    match input {
        Val::Pair(pair) => {
            let pair = Pair::from(pair);
            Val::Call(Call::new(pair.first, pair.second).into())
        }
        Val::Call(call) => Val::Call(call),
        _ => Val::default(),
    }
}

fn new_dependent() -> Named<FuncVal> {
    let val_mode = ValMode {
        pair: Box::new(Pair::new(default_mode(), Mode::Predefined(Transform::Id))),
        call: Box::new(CallMode::Struct(Call::new(
            default_mode(),
            Mode::Predefined(Transform::Id),
        ))),
        ..Default::default()
    };
    let input_mode = Mode::Custom(Box::new(val_mode));
    let output_mode = default_mode();
    named_mutable_fn("!!", input_mode, output_mode, fn_new_dependent)
}

fn fn_new_dependent(ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Pair(pair) => {
            let pair = Pair::from(pair);
            let func = pair.first;
            let input = pair.second;
            let input = Eval.eval_input(ctx, &func, input);
            Val::Call(Call::new(func, input).into())
        }
        Val::Call(call) => {
            let call = Call::from(call);
            let func = call.func;
            let input = call.input;
            let input = Eval.eval_input(ctx, &func, input);
            Val::Call(Call::new(func, input).into())
        }
        _ => Val::default(),
    }
}

fn apply() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mutable_fn("call.apply", input_mode, output_mode, func)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Call(call) = input else {
        return Val::default();
    };
    let call = Call::from(call);
    Eval::call(ctx, call.func, call.input)
}

fn get_func() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("call.function", input_mode, output_mode, fn_get_func)
}

fn fn_get_func(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Call(call) => call.func.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Call(call) => Call::from(call).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("call.set_function", input_mode, output_mode, fn_set_func)
}

fn fn_set_func(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut call) => {
            let Some(Val::Call(call)) = call.as_mut() else {
                return Val::default();
            };
            swap(&mut call.func, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_input() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("call.input", input_mode, output_mode, fn_get_input)
}

fn fn_get_input(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Call(call) => call.input.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Call(call) => Call::from(call).input,
            _ => Val::default(),
        },
    })
}

fn set_input() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("call.set_input", input_mode, output_mode, fn_set_input)
}

fn fn_set_input(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut call) => {
            let Some(Val::Call(call)) = call.as_mut() else {
                return Val::default();
            };
            swap(&mut call.input, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
