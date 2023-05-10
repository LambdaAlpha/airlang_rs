use crate::{
    semantics::{
        eval::{
            Ctx,
            Func,
            FuncImpl,
            FuncTrait,
            Name,
            Primitive,
        },
        prelude::names,
        val::Val,
    },
    types::Reader,
};

pub(crate) fn sequence() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::SEQUENCE),
            eval: Reader::new(fn_sequence),
        }),
    })
    .into()
}

fn fn_sequence(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        let mut output = Val::default();
        for val in list {
            output = ctx.eval(val);
        }
        return output;
    }
    Val::default()
}

pub(crate) fn condition() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::IF),
            eval: Reader::new(fn_if),
        }),
    })
    .into()
}

fn fn_if(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        let mut iter = list.into_iter();
        if let Some(condition) = iter.next() {
            if let Val::Bool(b) = ctx.eval(condition) {
                if b.bool() {
                    if let Some(branch) = iter.next() {
                        return ctx.eval(branch);
                    }
                } else {
                    let _ = iter.next();
                    if let Some(branch) = iter.next() {
                        return ctx.eval(branch);
                    }
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn while_loop() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::WHILE),
            eval: Reader::new(fn_while),
        }),
    })
    .into()
}

fn fn_while(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        if let Some(condition) = list.get(0) {
            if let Some(body) = list.get(1) {
                loop {
                    if let Val::Bool(b) = ctx.eval(condition.clone()) {
                        if b.bool() {
                            ctx.eval(body.clone());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    Val::default()
}
