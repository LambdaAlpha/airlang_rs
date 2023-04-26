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
    types::ImRef,
};

pub(crate) fn sequence() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::SEQUENCE),
            eval: ImRef::new(fn_sequence),
        }),
    })
}

fn fn_sequence(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        let mut output = Val::default();
        for val in list {
            output = ctx.eval(&val);
        }
        return output;
    }
    Val::default()
}

pub(crate) fn condition() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::IF),
            eval: ImRef::new(fn_if),
        }),
    })
}

fn fn_if(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        if let Some(condition) = list.get(0) {
            if let Val::Bool(b) = ctx.eval(condition) {
                if b.bool() {
                    if let Some(branch) = list.get(1) {
                        return ctx.eval(branch);
                    }
                } else {
                    if let Some(branch) = list.get(2) {
                        return ctx.eval(branch);
                    }
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn while_loop() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::WHILE),
            eval: ImRef::new(fn_while),
        }),
    })
}

fn fn_while(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(list) = input {
        if let Some(condition) = list.get(0) {
            if let Some(body) = list.get(1) {
                loop {
                    if let Val::Bool(b) = ctx.eval(condition) {
                        if b.bool() {
                            ctx.eval(body);
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
