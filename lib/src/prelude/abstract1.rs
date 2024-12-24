use std::mem::swap;

use const_format::concatcp;

use crate::{
    Abstract,
    ConstFnCtx,
    FreeCtx,
    FuncMode,
    FuncVal,
    Map,
    Mode,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    core::EvalCore,
    ctx::{
        CtxValue,
        default::DefaultCtx,
        ref1::CtxMeta,
    },
    func::mut_static_prim::MutDispatcher,
    mode::eval::Eval,
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
        ABSTRACT,
        ABSTRACT_STR,
    },
    types::either::Either,
};

#[derive(Clone)]
pub(crate) struct AbstractPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) new_dependent: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_input: Named<FuncVal>,
    pub(crate) set_input: Named<FuncVal>,
}

impl Default for AbstractPrelude {
    fn default() -> Self {
        AbstractPrelude {
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

impl Prelude for AbstractPrelude {
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
    let id = ABSTRACT_STR;
    let f = fn_new;
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Abstract(Abstract::new(pair.first, pair.second).into())
}

fn new_dependent() -> Named<FuncVal> {
    let id = concatcp!(ABSTRACT, ABSTRACT);
    let f = fn_new_dependent;
    let call = pair_mode(Mode::default(), form_mode());
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
    let input = pair.second;
    let input = EvalCore::abstract_eval_input(&Eval, ctx, &func, input);
    Val::Abstract(Abstract::new(func, input).into())
}

fn apply() -> Named<FuncVal> {
    let id = "abstract.apply";
    let f = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    let call = Mode::default();
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

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Abstract(abstract1) = input else {
        return Val::default();
    };
    let abstract1 = Abstract::from(abstract1);
    EvalCore::abstract1(ctx, abstract1.func, abstract1.input)
}

fn get_func() -> Named<FuncVal> {
    let id = "abstract.function";
    let f = fn_get_func;
    let call = Mode::default();
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
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "abstract.set_function";
    let f = fn_set_func;
    let call = Mode::default();
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
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut abstract1) => {
            let Some(Val::Abstract(abstract1)) = abstract1.as_mut() else {
                return Val::default();
            };
            swap(&mut abstract1.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_input() -> Named<FuncVal> {
    let id = "abstract.input";
    let f = fn_get_input;
    let call = Mode::default();
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

fn fn_get_input(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.input.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).input,
            _ => Val::default(),
        },
    })
}

fn set_input() -> Named<FuncVal> {
    let id = "abstract.set_input";
    let f = fn_set_input;
    let call = Mode::default();
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

fn fn_set_input(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut abstract1) => {
            let Some(Val::Abstract(abstract1)) = abstract1.as_mut() else {
                return Val::default();
            };
            swap(&mut abstract1.input, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
