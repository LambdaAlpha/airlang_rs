use std::mem::swap;

use crate::{
    Abstract,
    ConstFnCtx,
    FreeCtx,
    FuncVal,
    Map,
    Mode,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        default::DefaultCtx,
        ref1::CtxMeta,
    },
    func::mut1::MutDispatcher,
    mode::eval::Eval,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
    },
    syntax::ABSTRACT,
    transformer::ByVal,
    types::either::Either,
};

#[derive(Clone)]
pub(crate) struct AbstractPrelude {
    pub(crate) new: Named<FuncVal>,
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
    named_free_fn(ABSTRACT, call_mode, ask_mode, true, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Abstract(Abstract::new(pair.first, pair.second).into())
}

fn apply() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mut_fn("abstract.apply", call_mode, ask_mode, false, func)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Abstract(abstract1) = input else {
        return Val::default();
    };
    Eval.transform_abstract(ctx, abstract1)
}

fn get_func() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("abstract.function", call_mode, ask_mode, true, fn_get_func)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.func.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn(
        "abstract.set_function",
        call_mode,
        ask_mode,
        true,
        fn_set_func,
    )
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut abstract1) => {
            let Some(Val::Abstract(abstract1)) = abstract1.as_mut() else {
                return Val::default();
            };
            swap(&mut abstract1.func, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_input() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("abstract.input", call_mode, ask_mode, true, fn_get_input)
}

fn fn_get_input(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Abstract(abstract1) => abstract1.input.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Abstract(abstract1) => Abstract::from(abstract1).input,
            _ => Val::default(),
        },
    })
}

fn set_input() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn(
        "abstract.set_input",
        call_mode,
        ask_mode,
        true,
        fn_set_input,
    )
}

fn fn_set_input(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut abstract1) => {
            let Some(Val::Abstract(abstract1)) = abstract1.as_mut() else {
                return Val::default();
            };
            swap(&mut abstract1.input, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
