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
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_ADD),
            eval: Reader::new(fn_add),
        }),
    })
}

fn fn_add(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Int(i1.add(i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn subtract() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_SUBTRACT),
            eval: Reader::new(fn_subtract),
        }),
    })
}

fn fn_subtract(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Int(i1.subtract(i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn multiply() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_MULTIPLY),
            eval: Reader::new(fn_multiply),
        }),
    })
}

fn fn_multiply(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Int(i1.multiply(i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn divide() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_DIVIDE),
            eval: Reader::new(fn_divide),
        }),
    })
}

fn fn_divide(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                if let Some(i) = i1.divide(i2) {
                    return Val::Int(i);
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn remainder() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_REMAINDER),
            eval: Reader::new(fn_remainder),
        }),
    })
}

fn fn_remainder(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                if let Some(i) = i1.remainder(i2) {
                    return Val::Int(i);
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn divide_remainder() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_DIVIDE_REMAINDER),
            eval: Reader::new(fn_divide_remainder),
        }),
    })
}

fn fn_divide_remainder(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                if let Some((quotient, rem)) = i1.divide_remainder(i2) {
                    return Val::Pair(Box::new(Pair::new(Val::Int(quotient), Val::Int(rem))));
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn less_than() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_THAN),
            eval: Reader::new(fn_less_than),
        }),
    })
}

fn fn_less_than(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Bool(i1.less_than(&i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn less_equal() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_EQUAL),
            eval: Reader::new(fn_less_equal),
        }),
    })
}

fn fn_less_equal(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Bool(i1.less_equal(&i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn greater_than() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_GREATER_THAN),
            eval: Reader::new(fn_greater_than),
        }),
    })
}

fn fn_greater_than(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Bool(i1.greater_than(&i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn greater_equal() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_GREATER_EQUAL),
            eval: Reader::new(fn_greater_equal),
        }),
    })
}

fn fn_greater_equal(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Bool(i1.greater_equal(&i2));
            }
        }
    }
    Val::default()
}

pub(crate) fn less_greater() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INT_LESS_GREATER),
            eval: Reader::new(fn_less_greater),
        }),
    })
}

fn fn_less_greater(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Int(i1) = ctx.eval(&pair.first) {
            if let Val::Int(i2) = ctx.eval(&pair.second) {
                return Val::Bool(i1.less_greater(&i2));
            }
        }
    }
    Val::default()
}
