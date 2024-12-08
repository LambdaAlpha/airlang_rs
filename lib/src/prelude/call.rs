use std::mem::swap;

use const_format::concatcp;

use crate::{
    Call,
    FreeCtx,
    Map,
    Mode,
    Pair,
    Symbol,
    Val,
    core::EvalCore,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    func::mut1::MutDispatcher,
    mode::{
        eval::Eval,
        primitive::PrimitiveMode,
    },
    prelude::{
        Named,
        Prelude,
        form_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
    },
    syntax::{
        CALL,
        CALL_STR,
    },
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
    let id = CALL_STR;
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_new;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.first, pair.second).into())
}

fn new_dependent() -> Named<FuncVal> {
    let id = concatcp!(CALL, CALL);
    let call_mode = pair_mode(Mode::default(), form_mode(), PrimitiveMode::default());
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_new_dependent;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_new_dependent(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let func = pair.first;
    let input = pair.second;
    let input = EvalCore::eval_input(&Eval, ctx, &func, input);
    Val::Call(Call::new(func, input).into())
}

fn apply() -> Named<FuncVal> {
    let id = "call.apply";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Call(call) = input else {
        return Val::default();
    };
    let call = Call::from(call);
    EvalCore::call(ctx, call.func, call.input)
}

fn get_func() -> Named<FuncVal> {
    let id = "call.function";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_get_func;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "call.set_function";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_set_func;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "call.input";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_get_input;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "call.set_input";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_set_input;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
