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
    types::{
        Bool,
        Reader,
    },
};

pub(crate) fn not() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NOT),
            eval: Reader::new(fn_not),
        }),
    })
    .into()
}

fn fn_not(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Bool(b) = ctx.eval(input) else {
        return Val::default();
    };
    Val::Bool(b.not())
}

pub(crate) fn and() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::AND),
            eval: Reader::new(fn_and),
        }),
    })
    .into()
}

fn fn_and(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = ctx.eval(pair.first) else {
        return Val::default();
    };
    if left.bool() {
        let Val::Bool(right) = ctx.eval(pair.second) else {
            return Val::default();
        };
        Val::Bool(right)
    } else {
        Val::Bool(Bool::f())
    }
}

pub(crate) fn or() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::OR),
            eval: Reader::new(fn_or),
        }),
    })
    .into()
}

fn fn_or(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = ctx.eval(pair.first) else {
        return Val::default();
    };
    if left.bool() {
        Val::Bool(Bool::t())
    } else {
        let Val::Bool(right) = ctx.eval(pair.second) else {
            return Val::default();
        };
        Val::Bool(right)
    }
}

pub(crate) fn equal() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EQUAL),
            eval: Reader::new(fn_equal),
        }),
    })
    .into()
}

fn fn_equal(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(ctx.eval(pair.first) == ctx.eval(pair.second)))
}

pub(crate) fn not_equal() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NOT_EQUAL),
            eval: Reader::new(fn_not_equal),
        }),
    })
    .into()
}

fn fn_not_equal(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(ctx.eval(pair.first) != ctx.eval(pair.second)))
}
