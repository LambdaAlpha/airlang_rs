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
        Either,
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
    let name_or_str = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_str, |ref_or_val| {
        let f = |val: &Val| {
            let Val::String(s) = val else {
                return Val::default();
            };
            Val::Int(s.len().into())
        };
        match ref_or_val {
            Either::Left(val) => f(val),
            Either::Right(val) => f(&val),
        }
    })
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
    let Val::List(strings) = ctx.eval(input) else {
        return Val::default();
    };
    let mut ret = String::new();
    for str in strings {
        let Val::String(str) = str else {
            return Val::default();
        };
        ret.push_str(&str);
    }
    Val::String(Str::from(ret))
}
