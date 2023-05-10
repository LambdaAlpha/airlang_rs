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
    if let Val::Bool(b) = ctx.eval(&input) {
        return Val::Bool(b.not());
    }
    Val::default()
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
    if let Val::Pair(pair) = input {
        if let Val::Bool(left) = ctx.eval(&pair.first) {
            if left.bool() {
                if let Val::Bool(right) = ctx.eval(&pair.second) {
                    return Val::Bool(right);
                }
            } else {
                return Val::Bool(Bool::f());
            }
        }
    }
    Val::default()
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
    if let Val::Pair(pair) = input {
        if let Val::Bool(left) = ctx.eval(&pair.first) {
            if left.bool() {
                return Val::Bool(Bool::t());
            } else {
                if let Val::Bool(right) = ctx.eval(&pair.second) {
                    return Val::Bool(right);
                }
            }
        }
    }
    Val::default()
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
    if let Val::Pair(pair) = input {
        return Val::Bool(Bool::new(ctx.eval(&pair.first) == ctx.eval(&pair.second)));
    }
    Val::default()
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
    if let Val::Pair(pair) = input {
        return Val::Bool(Bool::new(ctx.eval(&pair.first) != ctx.eval(&pair.second)));
    }
    Val::default()
}
