use crate::semantics::{
    eval::{
        Ctx,
        Func,
        Primitive,
    },
    prelude::names,
    val::Val,
};

pub(crate) fn sequence() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::SEQUENCE,
        fn_sequence,
    )))
    .into()
}

fn fn_sequence(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut output = Val::default();
    for val in list {
        output = ctx.eval(val);
    }
    output
}

pub(crate) fn condition() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::IF,
        fn_if,
    )))
    .into()
}

fn fn_if(ctx: &mut Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut iter = list.into_iter();
    let Some(condition) = iter.next() else {
        return Val::default();
    };
    let Val::Bool(b) = ctx.eval(condition) else {
        return Val::default();
    };
    if b.bool() {
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        ctx.eval(branch)
    } else {
        let _ = iter.next();
        let Some(branch) = iter.next() else {
            return Val::default();
        };
        ctx.eval(branch)
    }
}

pub(crate) fn while_loop() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::WHILE,
        fn_while,
    )))
    .into()
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
        let Val::Bool(b) = ctx.eval_by_ref(condition) else {
            return Val::default();
        };
        if b.bool() {
            ctx.eval_by_ref(body);
        } else {
            break;
        }
    }
    Val::default()
}
