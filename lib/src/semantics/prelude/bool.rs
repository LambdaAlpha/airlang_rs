use crate::{
    semantics::{
        eval::{
            Ctx,
            EvalMode,
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
        ImRef,
    },
};

pub(crate) fn not() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NOT),
            eval: ImRef::new(fn_not),
        }),
    })
}

fn fn_not(_: &mut Ctx, input: Val) -> Val {
    if let Val::Bool(b) = input {
        return Val::Bool(b.not());
    }
    Val::default()
}

pub(crate) fn and() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::AND),
            eval: ImRef::new(fn_and),
        }),
    })
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
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::OR),
            eval: ImRef::new(fn_or),
        }),
    })
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
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EQUAL),
            eval: ImRef::new(fn_equal),
        }),
    })
}

fn fn_equal(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        return Val::Bool(Bool::new(ctx.eval(&pair.first) == ctx.eval(&pair.second)));
    }
    Val::default()
}

pub(crate) fn not_equal() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NOT_EQUAL),
            eval: ImRef::new(fn_not_equal),
        }),
    })
}

fn fn_not_equal(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        return Val::Bool(Bool::new(ctx.eval(&pair.first) != ctx.eval(&pair.second)));
    }
    Val::default()
}
