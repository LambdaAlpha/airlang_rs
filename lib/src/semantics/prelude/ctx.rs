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
    types::ImRef,
};

pub(crate) fn assign() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN),
            eval: ImRef::new(fn_assign),
        }),
    })
}

fn fn_assign(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        let name: &str = match &pair.first {
            Val::Letter(l) => l,
            Val::Symbol(s) => s,
            Val::String(s) => s,
            _ => return Val::default(),
        };
        let val = ctx.eval(&pair.second);
        return ctx.put(Name::from(name), val);
    }
    Val::default()
}
