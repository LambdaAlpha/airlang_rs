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
        CtxMutableFn,
        EvalMode,
        Primitive,
    },
    prelude::{
        names,
        PrimitiveFunc,
    },
    val::Val,
};

pub(crate) fn sequence() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Value);
    let primitive = Primitive::new_dispatch(
        names::SEQUENCE,
        fn_sequence::<DefaultConstStrategy>,
        fn_sequence::<DefaultStrategy>,
    );
    PrimitiveFunc::new(eval_mode, primitive)
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

pub(crate) fn condition() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Value);
    let primitive = Primitive::new_dispatch(
        names::IF,
        fn_if::<DefaultConstStrategy>,
        fn_if::<DefaultStrategy>,
    );
    PrimitiveFunc::new(eval_mode, primitive)
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

pub(crate) fn while_loop() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Value);
    let primitive = Primitive::new_dispatch(
        names::WHILE,
        fn_while::<DefaultConstByRefStrategy>,
        fn_while::<DefaultByRefStrategy>,
    );
    PrimitiveFunc::new(eval_mode, primitive)
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
