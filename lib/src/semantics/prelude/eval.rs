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
            TaggedVal,
        },
        prelude::{
            map::fn_map_new,
            names,
        },
        val::{
            ListVal,
            MapVal,
            Val,
        },
    },
    types::{
        Keeper,
        Pair,
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
            eval_ref(ctx, &input.val)
        }
        i => {
            let val = ctx.eval(i);
            ctx.eval(val)
        }
    }
}

fn eval_ref(ctx: &mut Ctx, input: &Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.get(s),
        Val::Ref(k) => ctx.eval_ref_val(k),
        Val::Pair(p) => eval_ref_pair(ctx, &p.first, &p.second),
        Val::List(l) => eval_ref_list(ctx, l),
        Val::Map(m) => eval_ref_map(ctx, m),
        Val::Call(c) => eval_ref_call(ctx, &c.func, &c.input),
        Val::Reverse(r) => eval_ref_reverse(ctx, &r.func, &r.output),
        v => v.clone(),
    }
}

fn eval_ref_pair(ctx: &mut Ctx, first: &Val, second: &Val) -> Val {
    let pair = Pair::new(eval_ref(ctx, first), eval_ref(ctx, second));
    Val::Pair(Box::new(pair))
}

fn eval_ref_list(ctx: &mut Ctx, list: &ListVal) -> Val {
    let list = list.into_iter().map(|v| eval_ref(ctx, v)).collect();
    Val::List(list)
}

fn eval_ref_map(ctx: &mut Ctx, map: &MapVal) -> Val {
    let map = map
        .into_iter()
        .map(|(k, v)| (eval_ref(ctx, k), eval_ref(ctx, v)))
        .collect();
    Val::Map(map)
}

fn eval_ref_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
    let Val::Func(func) = eval_ref(ctx, func) else {
        return Val::default();
    };
    func.eval(ctx, input.clone())
}

fn eval_ref_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
    ctx.eval_reverse(func.clone(), output.clone())
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
    let constants = match fn_map_new(ctx, map_remove(&mut map, "const")) {
        Val::Map(m) => {
            let Some(constants) = into_name_map(m) else {
                return Val::default();
            };
            constants
        }
        Val::Unit(_) => NameMap::default(),
        _ => return Val::default(),
    };
    let input_name = match ctx.eval_escape(map_remove(&mut map, "input")) {
        Val::Symbol(s) => {
            if &*s == "_" {
                None
            } else {
                Some(Name::from(<_ as Into<String>>::into(s)))
            }
        }
        Val::Unit(_) => Some(Name::from("input")),
        _ => return Val::default(),
    };
    let caller_name = match ctx.eval_escape(map_remove(&mut map, "caller")) {
        Val::Symbol(s) => {
            if &*s == "_" {
                None
            } else {
                Some(Name::from(<_ as Into<String>>::into(s)))
            }
        }
        Val::Unit(_) => Some(Name::from("caller")),
        _ => return Val::default(),
    };
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Composed(Composed {
            body,
            constants,
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

fn into_name_map(map: MapVal) -> Option<NameMap> {
    map.into_iter()
        .map(|(k, v)| match k {
            Val::Symbol(s) => Some((
                Name::from(<_ as Into<String>>::into(s)),
                TaggedVal::new_const(v),
            )),
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
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Func(func) = ctx.eval(pair.second) else {
        return Val::default();
    };
    func.eval(ctx, pair.first)
}
