use crate::{
    semantics::{
        eval::{
            Composed,
            Ctx,
            Func,
            FuncImpl,
            FuncTrait,
            Name,
            NameMap,
            Primitive,
        },
        prelude::names,
        val::{
            MapVal,
            Val,
        },
    },
    types::{
        Call,
        Pair,
        Reader,
        Reverse,
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
    let val = ctx.eval(input);
    ctx.eval(val)
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

pub(crate) fn fn_eval_escape(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Call(c) => match &c.func {
            Val::Symbol(s) => {
                if &**s == "\\" {
                    ctx.eval(c.input)
                } else {
                    let func = fn_eval_escape(ctx, c.func);
                    let input = fn_eval_escape(ctx, c.input);
                    let call = Box::new(Call::new(func, input));
                    Val::Call(call)
                }
            }
            _ => {
                let func = fn_eval_escape(ctx, c.func);
                let input = fn_eval_escape(ctx, c.input);
                let call = Box::new(Call::new(func, input));
                Val::Call(call)
            }
        },
        Val::Pair(p) => {
            let first = fn_eval_escape(ctx, p.first);
            let second = fn_eval_escape(ctx, p.second);
            let pair = Box::new(Pair::new(first, second));
            Val::Pair(pair)
        }
        Val::Reverse(r) => {
            let func = fn_eval_escape(ctx, r.func);
            let output = fn_eval_escape(ctx, r.output);
            let reverse = Box::new(Reverse::new(func, output));
            Val::Reverse(reverse)
        }
        Val::List(l) => {
            let list = l.into_iter().map(|v| fn_eval_escape(ctx, v)).collect();
            Val::List(list)
        }
        Val::Map(m) => {
            let map = m
                .into_iter()
                .map(|(k, v)| {
                    let key = fn_eval_escape(ctx, k);
                    let value = fn_eval_escape(ctx, v);
                    (key, value)
                })
                .collect();
            Val::Map(map)
        }
        i => i,
    }
}

pub(crate) fn eval_positional_escape() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL_POSITIONAL_ESCAPE),
            eval: Reader::new(fn_eval_positional_escape),
        }),
    })
    .into()
}

fn fn_eval_positional_escape(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Map(m) => {
            let map = m
                .into_iter()
                .map(|(k, v)| {
                    let key = fn_eval_escape(ctx, k);
                    let value = fn_eval_positional_escape(ctx, v);
                    (key, value)
                })
                .collect();
            Val::Map(map)
        }
        Val::Pair(p) => {
            let first = fn_eval_positional_escape(ctx, p.first);
            let second = fn_eval_positional_escape(ctx, p.second);
            let pair = Box::new(Pair::new(first, second));
            Val::Pair(pair)
        }
        Val::List(l) => {
            let list = l
                .into_iter()
                .map(|v| fn_eval_positional_escape(ctx, v))
                .collect();
            Val::List(list)
        }
        i => ctx.eval(i),
    }
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
    if let Val::Pair(pair) = input {
        if let Val::Ctx(mut target_ctx) = ctx.eval(pair.first) {
            let val = ctx.eval(pair.second);
            target_ctx.eval(val)
        } else {
            Val::default()
        }
    } else {
        Val::default()
    }
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
    if let Val::String(input) = ctx.eval(input) {
        if let Ok(val) = crate::semantics::parse(&input) {
            return val;
        }
    }
    Val::default()
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
    if let Ok(str) = crate::semantics::generate(&val) {
        return Val::String(Str::from(str));
    }
    Val::default()
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
    if let Val::Map(map) = input {
        let body = fn_eval_escape(ctx, map_get(&map, "body"));
        let constants = match fn_eval_positional_escape(ctx, map_get(&map, "const")) {
            Val::Map(m) => {
                if let Some(constants) = into_name_map(m) {
                    constants
                } else {
                    return Val::default();
                }
            }
            Val::Unit(_) => NameMap::default(),
            _ => return Val::default(),
        };
        let input_name = match fn_eval_escape(ctx, map_get(&map, "input")) {
            Val::Symbol(s) => {
                if &*s == "_" {
                    None
                } else {
                    Some(Name::from(&*s))
                }
            }
            Val::Unit(_) => Some(Name::from("input")),
            _ => return Val::default(),
        };
        let caller_name = match fn_eval_escape(ctx, map_get(&map, "caller")) {
            Val::Symbol(s) => {
                if &*s == "_" {
                    None
                } else {
                    Some(Name::from(&*s))
                }
            }
            Val::Unit(_) => Some(Name::from("caller")),
            _ => return Val::default(),
        };
        return Box::new(Func {
            func_trait: FuncTrait {},
            func_impl: FuncImpl::Composed(Composed {
                body,
                constants: Reader::new(constants),
                input_name,
                caller_name,
            }),
        })
        .into();
    }
    Val::default()
}

fn map_get(map: &MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.get(&name).map(Clone::clone).unwrap_or_default()
}

fn into_name_map(map: MapVal) -> Option<NameMap> {
    map.into_iter()
        .map(|(k, v)| match k {
            Val::Symbol(s) => Some((Name::from(&*s), v)),
            _ => None,
        })
        .collect()
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
    if let Val::Pair(pair) = input {
        if let Val::Func(func) = ctx.eval(pair.second) {
            return func.eval(ctx, pair.first);
        }
    }
    Val::default()
}
