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
        Pair,
        Reader,
    },
};

pub(crate) fn add() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_ADD),
            eval: Reader::new(fn_add),
        }),
    })
    .into()
}

fn fn_add(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Int(i1.add(i2))
}

pub(crate) fn subtract() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_SUBTRACT),
            eval: Reader::new(fn_subtract),
        }),
    })
    .into()
}

fn fn_subtract(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Int(i1.subtract(i2))
}

pub(crate) fn multiply() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_MULTIPLY),
            eval: Reader::new(fn_multiply),
        }),
    })
    .into()
}

fn fn_multiply(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Int(i1.multiply(i2))
}

pub(crate) fn divide() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_DIVIDE),
            eval: Reader::new(fn_divide),
        }),
    })
    .into()
}

fn fn_divide(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

pub(crate) fn remainder() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_REMAINDER),
            eval: Reader::new(fn_remainder),
        }),
    })
    .into()
}

fn fn_remainder(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

pub(crate) fn divide_remainder() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_DIVIDE_REMAINDER),
            eval: Reader::new(fn_divide_remainder),
        }),
    })
    .into()
}

fn fn_divide_remainder(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Box::new(Pair::new(Val::Int(quotient), Val::Int(rem))))
}

pub(crate) fn less_than() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_THAN),
            eval: Reader::new(fn_less_than),
        }),
    })
    .into()
}

fn fn_less_than(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Bool(i1.less_than(&i2))
}

pub(crate) fn less_equal() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_EQUAL),
            eval: Reader::new(fn_less_equal),
        }),
    })
    .into()
}

fn fn_less_equal(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Bool(i1.less_equal(&i2))
}

pub(crate) fn greater_than() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_GREATER_THAN),
            eval: Reader::new(fn_greater_than),
        }),
    })
    .into()
}

fn fn_greater_than(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Bool(i1.greater_than(&i2))
}

pub(crate) fn greater_equal() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_GREATER_EQUAL),
            eval: Reader::new(fn_greater_equal),
        }),
    })
    .into()
}

fn fn_greater_equal(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Bool(i1.greater_equal(&i2))
}

pub(crate) fn less_greater() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_GREATER),
            eval: Reader::new(fn_less_greater),
        }),
    })
    .into()
}

fn fn_less_greater(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let Val::Int(i2) = ctx.eval(pair.second) else {
        return Val::default();
    };
    Val::Bool(i1.less_greater(&i2))
}
