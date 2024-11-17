use std::mem::swap;

use crate::{
    Adapt,
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
    syntax::ADAPT,
    transformer::ByVal,
    types::either::Either,
};

#[derive(Clone)]
pub(crate) struct AdaptPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_spec: Named<FuncVal>,
    pub(crate) set_spec: Named<FuncVal>,
    pub(crate) get_value: Named<FuncVal>,
    pub(crate) set_value: Named<FuncVal>,
}

impl Default for AdaptPrelude {
    fn default() -> Self {
        AdaptPrelude {
            new: new(),
            apply: apply(),
            get_spec: get_spec(),
            set_spec: set_spec(),
            get_value: get_value(),
            set_value: set_value(),
        }
    }
}

impl Prelude for AdaptPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.apply.put(m);
        self.get_spec.put(m);
        self.set_spec.put(m);
        self.get_value.put(m);
        self.set_value.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_free_fn(ADAPT, call_mode, ask_mode, true, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Adapt(Adapt::new(pair.first, pair.second).into())
}

fn apply() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mut_fn("adapt.apply", call_mode, ask_mode, false, func)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Adapt(adapt) = input else {
        return Val::default();
    };
    Eval.transform_adapt(ctx, adapt)
}

fn get_spec() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn(
        "adapt.specification",
        call_mode,
        ask_mode,
        true,
        fn_get_spec,
    )
}

fn fn_get_spec(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Adapt(adapt) => adapt.spec.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Adapt(adapt) => Adapt::from(adapt).spec,
            _ => Val::default(),
        },
    })
}

fn set_spec() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn(
        "adapt.set_specification",
        call_mode,
        ask_mode,
        true,
        fn_set_spec,
    )
}

fn fn_set_spec(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut adapt) => {
            let Some(Val::Adapt(adapt)) = adapt.as_mut() else {
                return Val::default();
            };
            swap(&mut adapt.spec, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_value() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("adapt.value", call_mode, ask_mode, true, fn_get_value)
}

fn fn_get_value(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Adapt(adapt) => adapt.value.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Adapt(adapt) => Adapt::from(adapt).value,
            _ => Val::default(),
        },
    })
}

fn set_value() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn("adapt.set_value", call_mode, ask_mode, true, fn_set_value)
}

fn fn_set_value(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut adapt) => {
            let Some(Val::Adapt(adapt)) = adapt.as_mut() else {
                return Val::default();
            };
            swap(&mut adapt.value, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
