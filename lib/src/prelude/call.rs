use std::mem::swap;

use crate::{
    Call,
    FreeCtx,
    Map,
    Mode,
    Pair,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    func::mut1::MutDispatcher,
    mode::{
        basic::BasicMode,
        eval::Eval,
    },
    prelude::{
        Named,
        Prelude,
        form_mode,
        named_const_fn,
        named_mut_fn,
        named_static_fn,
        pair_mode,
    },
    syntax::CALL,
    types::either::Either,
    val::func::FuncVal,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_static_fn(CALL, call_mode, ask_mode, true, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.first, pair.second).into())
}

fn new_dependent() -> Named<FuncVal> {
    let call_mode = pair_mode(Mode::default(), form_mode(), BasicMode::default());
    let ask_mode = Mode::default();
    named_mut_fn("!!", call_mode, ask_mode, false, fn_new_dependent)
}

fn fn_new_dependent(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let func = pair.first;
    let input = pair.second;
    let input = Eval.eval_input(ctx, &func, input);
    Val::Call(Call::new(func, input).into())
}

fn apply() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mut_fn("call.apply", call_mode, ask_mode, false, func)
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("call.function", call_mode, ask_mode, true, fn_get_func)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn("call.set_function", call_mode, ask_mode, true, fn_set_func)
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("call.input", call_mode, ask_mode, true, fn_get_input)
}

fn fn_get_input(ctx: ConstFnCtx, input: Val) -> Val {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn("call.set_input", call_mode, ask_mode, true, fn_set_input)
}

fn fn_set_input(ctx: MutFnCtx, input: Val) -> Val {
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
