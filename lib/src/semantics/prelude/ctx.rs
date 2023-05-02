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

pub(crate) fn assign() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN),
            eval: Reader::new(fn_assign),
        }),
    })
}

fn fn_assign(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        let name: &str = match &pair.first {
            Val::Symbol(s) => s,
            Val::String(s) => s,
            _ => return Val::default(),
        };
        let val = ctx.eval(&pair.second);
        return ctx.put(Name::from(name), val);
    }
    Val::default()
}

pub(crate) fn remove() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MOVE),
            eval: Reader::new(fn_move),
        }),
    })
}

fn fn_move(ctx: &mut Ctx, input: Val) -> Val {
    let name: &str = match &input {
        Val::Symbol(s) => s,
        Val::String(s) => s,
        _ => return Val::default(),
    };
    return ctx.remove(name);
}
