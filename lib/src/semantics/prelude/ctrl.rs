use crate::semantics::{
    eval::{
        strategy::{
            eval::{
                DefaultByRefStrategy,
                DefaultStrategy,
            },
            ByRefStrategy,
            EvalStrategy,
        },
        BasicEvalMode,
        Ctx,
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::SEQUENCE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_sequence,
    )))
}

fn fn_sequence(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut output = Val::default();
    for val in list {
        output = DefaultStrategy::eval(ctx, val);
    }
    output
}

pub(crate) fn condition() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::IF,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_if,
    )))
}

fn fn_if(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut iter = list.into_iter();
    let Some(condition) = iter.next() else {
        return Val::default();
    };
    let Val::Bool(b) = DefaultStrategy::eval(ctx, condition) else {
        return Val::default();
    };
    if b.bool() {
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        DefaultStrategy::eval(ctx, branch)
    } else {
        let _ = iter.next();
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        DefaultStrategy::eval(ctx, branch)
    }
}

pub(crate) fn while_loop() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::WHILE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_while,
    )))
}

fn fn_while(ctx: &mut Ctx, input: Val) -> Val {
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
        let Val::Bool(b) = DefaultByRefStrategy::eval(ctx, condition) else {
            return Val::default();
        };
        if b.bool() {
            DefaultByRefStrategy::eval(ctx, body);
        } else {
            break;
        }
    }
    Val::default()
}
