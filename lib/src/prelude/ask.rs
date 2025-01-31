use std::mem::swap;

use const_format::concatcp;

use crate::{
    Ask,
    ConstFnCtx,
    FreeCtx,
    FuncMode,
    Map,
    Mode,
    Pair,
    Symbol,
    Val,
    core::EvalCore,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    func::mut_static_prim::MutDispatcher,
    mode::eval::EVAL,
    prelude::{
        Named,
        Prelude,
        id_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
        ref_pair_mode,
    },
    syntax::{
        ASK,
        ASK_CHAR,
    },
    types::either::Either,
    val::func::FuncVal,
};

#[derive(Clone)]
pub(crate) struct AskPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) new_dependent: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_output: Named<FuncVal>,
    pub(crate) set_output: Named<FuncVal>,
}

impl Default for AskPrelude {
    fn default() -> Self {
        AskPrelude {
            new: new(),
            new_dependent: new_dependent(),
            apply: apply(),
            get_func: get_func(),
            set_func: set_func(),
            get_output: get_output(),
            set_output: set_output(),
        }
    }
}

impl Prelude for AskPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.new_dependent.put(m);
        self.apply.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
        self.get_output.put(m);
        self.set_output.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = ASK;
    let f = fn_new;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Ask(Ask::new(pair.first, pair.second).into())
}

fn new_dependent() -> Named<FuncVal> {
    let id = concatcp!(ASK_CHAR, ASK_CHAR);
    let f = fn_new_dependent;
    let call = pair_mode(Mode::default(), id_mode());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_new_dependent(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let func = pair.first;
    let output = pair.second;
    let output = EvalCore::ask_eval_output(&EVAL, ctx, &func, output);
    Val::Ask(Ask::new(func, output).into())
}

fn apply() -> Named<FuncVal> {
    let id = "ask.apply";
    let f = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    let mode = FuncMode::default();
    let cacheable = false;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Ask(ask) = input else {
        return Val::default();
    };
    let ask = Ask::from(ask);
    EvalCore::ask(ctx, ask.func, ask.output)
}

fn get_func() -> Named<FuncVal> {
    let id = "ask.function";
    let f = fn_get_func;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Ask(ask) => ask.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Ask(ask) => Ask::from(ask).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "ask.set_function";
    let f = fn_set_func;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut ask) => {
            let Some(Val::Ask(ask)) = ask.as_mut() else {
                return Val::default();
            };
            swap(&mut ask.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_output() -> Named<FuncVal> {
    let id = "ask.output";
    let f = fn_get_output;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_output(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Ask(ask) => ask.output.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Ask(ask) => Ask::from(ask).output,
            _ => Val::default(),
        },
    })
}

fn set_output() -> Named<FuncVal> {
    let id = "ask.set_output";
    let f = fn_set_output;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_output(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut ask) => {
            let Some(Val::Ask(ask)) = ask.as_mut() else {
                return Val::default();
            };
            swap(&mut ask.output, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
