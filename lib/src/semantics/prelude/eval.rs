use crate::{
    semantics::{
        eval::{
            Composed,
            Ctx,
            Func,
            FuncImpl,
            FuncTrait,
            Name,
            Primitive,
        },
        prelude::names,
        val::{
            MapVal,
            Val,
        },
    },
    types::{
        Keeper,
        Reader,
        Str,
        Symbol,
    },
};

pub(crate) fn val() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::VAL),
            eval: Reader::new(fn_val),
        }),
    })
    .into()
}

fn fn_val(_: &mut Ctx, input: Val) -> Val {
    input
}

pub(crate) fn eval() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL),
            eval: Reader::new(fn_eval),
        }),
    })
    .into()
}

fn fn_eval(ctx: &mut Ctx, input: Val) -> Val {
    ctx.eval(input)
}

pub(crate) fn eval_twice() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL_TWICE),
            eval: Reader::new(fn_eval_twice),
        }),
    })
    .into()
}

fn fn_eval_twice(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Ref(k) => {
            let Ok(input) = Keeper::reader(&k.0) else {
                return Val::default();
            };
            ctx.eval_by_ref(&input.val)
        }
        i => {
            let val = ctx.eval(i);
            ctx.eval(val)
        }
    }
}

pub(crate) fn eval_thrice() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL_THRICE),
            eval: Reader::new(fn_eval_thrice),
        }),
    })
    .into()
}

fn fn_eval_thrice(ctx: &mut Ctx, input: Val) -> Val {
    let val = ctx.eval(input);
    fn_eval_twice(ctx, val)
}

pub(crate) fn eval_escape() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL_ESCAPE),
            eval: Reader::new(fn_eval_escape),
        }),
    })
    .into()
}

fn fn_eval_escape(ctx: &mut Ctx, input: Val) -> Val {
    ctx.eval_escape(input)
}

pub(crate) fn eval_in_ctx() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL_IN_CTX),
            eval: Reader::new(fn_eval_in_ctx),
        }),
    })
    .into()
}

fn fn_eval_in_ctx(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Ctx(mut target_ctx) = ctx.eval(pair.first) else {
        return Val::default();
    };
    let val = ctx.eval(pair.second);
    target_ctx.eval(val)
}
pub(crate) fn parse() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PARSE),
            eval: Reader::new(fn_parse),
        }),
    })
    .into()
}

fn fn_parse(ctx: &mut Ctx, input: Val) -> Val {
    let Val::String(input) = ctx.eval(input)else {
        return Val::default();
    };
    crate::semantics::parse(&input).unwrap_or_default()
}

pub(crate) fn stringify() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::STRINGIFY),
            eval: Reader::new(fn_stringify),
        }),
    })
    .into()
}

fn fn_stringify(ctx: &mut Ctx, input: Val) -> Val {
    let val = ctx.eval(input);
    let Ok(str) = crate::semantics::generate(&val) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}

pub(crate) fn func() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FUNC),
            eval: Reader::new(fn_func),
        }),
    })
    .into()
}

fn fn_func(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = ctx.eval_escape(map_remove(&mut map, "body"));
    let func_ctx = match ctx.eval(map_remove(&mut map, "context")) {
        Val::Ctx(func_ctx) => *func_ctx,
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match ctx.eval_escape(map_remove(&mut map, "input")) {
        Val::Symbol(Symbol(name)) => name,
        Val::Unit(_) => Name::from("input"),
        _ => return Val::default(),
    };
    let caller_name = match ctx.eval_escape(map_remove(&mut map, "caller")) {
        Val::Symbol(Symbol(name)) => name,
        Val::Unit(_) => Name::from("caller"),
        _ => return Val::default(),
    };
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Composed(Composed {
            body,
            ctx: func_ctx,
            input_name,
            caller_name,
        }),
    })
    .into()
}

fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn chain() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CHAIN),
            eval: Reader::new(fn_chain),
        }),
    })
    .into()
}

fn fn_chain(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Func(func) = ctx.eval(pair.second) else {
        return Val::default();
    };
    func.eval(ctx, pair.first)
}
