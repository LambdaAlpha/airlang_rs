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
        prelude::{
            eval::fn_eval_escape,
            names,
        },
        val::Val,
    },
    types::{
        Keeper,
        Reader,
        Str,
    },
};

pub(crate) fn length() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::STR_LENGTH),
            eval: Reader::new(fn_length),
        }),
    })
    .into()
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_str = fn_eval_escape(ctx, input);
    match name_or_str {
        Val::Symbol(name) => {
            if let Some(Val::String(s)) = ctx.get_ref(&name) {
                return Val::Int(s.len().into());
            }
        }
        Val::Keeper(k) => {
            if let Ok(r) = Keeper::reader(&k) {
                if let Val::String(s) = &*r {
                    return Val::Int(s.len().into());
                }
            }
        }
        Val::String(s) => {
            return Val::Int(s.len().into());
        }
        _ => {}
    }
    Val::default()
}

pub(crate) fn concat() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::STR_CONCAT),
            eval: Reader::new(fn_concat),
        }),
    })
    .into()
}

fn fn_concat(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::List(strings) = ctx.eval(input) {
        let mut ret = String::new();
        for str in strings {
            if let Val::String(str) = str {
                ret.push_str(&str)
            } else {
                return Val::default();
            }
        }
        return Val::String(Str::from(ret));
    }
    Val::default()
}
