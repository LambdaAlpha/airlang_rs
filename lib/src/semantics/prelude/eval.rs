use crate::{
    repr::Repr,
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
        Reader,
        Str,
        Symbol,
    },
};

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
    if let Val::Pair(pair) = input {
        if let Val::Ctx(mut target_ctx) = ctx.eval(pair.first) {
            let val = ctx.eval(pair.second);
            target_ctx.eval(val)
        } else {
            Val::default()
        }
    } else {
        let val = ctx.eval(input);
        ctx.eval(val)
    }
}

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
        if let Ok(repr) = crate::syntax::parse(&input) {
            return Val::from(repr);
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
    if let Ok(repr) = ctx.eval(input).try_into() {
        return Val::String(Str::from(crate::syntax::generate(&repr)));
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
        let body = map_get(&map, "body");
        let constants = match map_get(&map, "const") {
            Val::Map(m) => {
                if let Some(constants) = eval_name_map(ctx, m) {
                    constants
                } else {
                    return Val::default();
                }
            }
            Val::Unit(_) => NameMap::default(),
            _ => return Val::default(),
        };
        let input_name = match map_get(&map, "input") {
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
        let caller_name = match map_get(&map, "caller") {
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
    let name = Repr::Symbol(Symbol::from_str(name));
    map.get(&name).map(Clone::clone).unwrap_or_default()
}

fn eval_name_map(ctx: &mut Ctx, map: MapVal) -> Option<NameMap> {
    let mut name_map = NameMap::default();
    for (k, v) in map.into_iter() {
        let name = match k {
            Repr::Symbol(s) => Name::from(&*s),
            _ => return None,
        };
        let val = ctx.eval(v);
        name_map.insert(name, val);
    }
    Some(name_map)
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
