use crate::semantics::{
    eval::{
        ctx::Ctx,
        strategy::{
            eval::{
                DefaultByRefStrategy,
                DefaultConstByRefStrategy,
                DefaultConstStrategy,
                DefaultStrategy,
            },
            ByRefStrategy,
            EvalStrategy,
        },
        BasicEvalMode,
        EvalMode,
        Func,
        Primitive,
    },
    prelude::{
        names,
        prelude_func,
    },
    val::Val,
};

pub(crate) fn sequence() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable_dispatch(
        names::SEQUENCE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_sequence::<DefaultConstStrategy>,
        fn_sequence::<DefaultStrategy>,
    )))
}

fn fn_sequence<Eval: EvalStrategy>(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut output = Val::default();
    for val in list {
        output = Eval::eval(ctx, val);
    }
    output
}

pub(crate) fn condition() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable_dispatch(
        names::IF,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_if::<DefaultConstStrategy>,
        fn_if::<DefaultStrategy>,
    )))
}

fn fn_if<Eval: EvalStrategy>(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut iter = list.into_iter();
    let Some(condition) = iter.next() else {
        return Val::default();
    };
    let Val::Bool(b) = Eval::eval(ctx, condition) else {
        return Val::default();
    };
    if b.bool() {
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        Eval::eval(ctx, branch)
    } else {
        let _ = iter.next();
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        Eval::eval(ctx, branch)
    }
}

pub(crate) fn while_loop() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable_dispatch(
        names::WHILE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_while::<DefaultConstByRefStrategy>,
        fn_while::<DefaultByRefStrategy>,
    )))
}

fn fn_while<Eval: ByRefStrategy>(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let Some(condition) = list.get(0) else {
        return Val::default();
    };
    let Some(body) = list.get(1) else {
        return Val::default();
    };
    loop {
        let Val::Bool(b) = Eval::eval(ctx, condition) else {
            return Val::default();
        };
        if b.bool() {
            Eval::eval(ctx, body);
        } else {
            break;
        }
    }
    Val::default()
}
