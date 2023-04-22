use crate::{
    repr::Repr,
    semantics::{
        eval::{
            Composed,
            Ctx,
            EvalMode,
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
    traits::TryClone,
    types::{
        ImRef,
        Letter,
        Str,
    },
};

pub(crate) fn eval() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::EVAL),
            eval: ImRef::new(fn_eval),
        }),
    })
}

fn fn_eval(ctx: &mut Ctx, input: Val) -> Val {
    ctx.eval(&input)
}

pub(crate) fn val() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::VAL),
            eval: ImRef::new(fn_val),
        }),
    })
}

fn fn_val(_: &mut Ctx, input: Val) -> Val {
    input
}

pub(crate) fn parse() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PARSE),
            eval: ImRef::new(fn_parse),
        }),
    })
}

fn fn_parse(_: &mut Ctx, input: Val) -> Val {
    if let Val::String(input) = input {
        if let Ok(repr) = crate::syntax::parse(&input) {
            return Val::from(repr);
        }
    }
    Val::default()
}

pub(crate) fn stringify() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::STRINGIFY),
            eval: ImRef::new(fn_stringify),
        }),
    })
}

fn fn_stringify(_: &mut Ctx, input: Val) -> Val {
    if let Ok(repr) = input.try_into() {
        return Val::String(Str::from(crate::syntax::generate(&repr)));
    }
    Val::default()
}

pub(crate) fn func() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FUNC),
            eval: ImRef::new(fn_func),
        }),
    })
}

fn fn_func(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Map(map) = input {
        let input_eval_mode = match map_get(&map, "input_eval_mode") {
            Val::Letter(l) => match &*l {
                "val" => EvalMode::Val,
                "eval" => EvalMode::Eval,
                _ => return Val::default(),
            },
            Val::Unit(_) => EvalMode::Eval,
            _ => return Val::default(),
        };
        let eval = map_get(&map, "eval");
        let constants = match map_get(&map, "constants") {
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
        let input_name = match map_get(&map, "input_name") {
            Val::Letter(l) => Some(Name::from(&*l)),
            Val::Symbol(s) => Some(Name::from(&*s)),
            Val::Unit(_) => None,
            _ => return Val::default(),
        };
        let caller_name = match map_get(&map, "caller_name") {
            Val::Letter(l) => Some(Name::from(&*l)),
            Val::Symbol(s) => Some(Name::from(&*s)),
            Val::Unit(_) => None,
            _ => return Val::default(),
        };
        return Val::Func(Func {
            func_trait: FuncTrait { input_eval_mode },
            func_impl: FuncImpl::Composed(Composed {
                eval: ImRef::new(eval),
                constants: ImRef::new(constants),
                input_name,
                caller_name,
            }),
        });
    }
    Val::default()
}

fn map_get(map: &MapVal, name: &str) -> Val {
    let name = Repr::Letter(Letter::from_str(name));
    map.get(&name)
        .and_then(|v| v.try_clone())
        .unwrap_or_default()
}

fn eval_name_map(ctx: &mut Ctx, map: MapVal) -> Option<NameMap> {
    let mut name_map = NameMap::default();
    for (k, v) in map.into_iter() {
        let name = match k {
            Repr::Letter(l) => Name::from(&*l),
            Repr::Symbol(s) => Name::from(&*s),
            _ => return None,
        };
        let val = ctx.eval(&v);
        name_map.insert(name, val);
    }
    Some(name_map)
}
