use crate::{
    ctx_access::CtxAccessor,
    eval::{
        output::OutputBuilder,
        Evaluator,
        ValBuilder,
    },
    val::func::FuncVal,
    PropVal,
    Val,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Answer {
    #[default]
    Unsolved,
    Unsolvable,
    Unverified(Val),
    Verified(PropVal),
}

pub(crate) fn solve<Ctx: CtxAccessor>(ctx: &mut Ctx, func: FuncVal, output: Val) -> Val {
    if !func.is_ctx_free() {
        return Val::default();
    }
    let Ok(meta) = ctx.get_meta() else {
        return Val::default();
    };
    let Ok(solver) = meta.get(SOLVER) else {
        return Val::default();
    };
    let Val::Func(FuncVal(solver)) = solver else {
        return Val::default();
    };
    let reverse = ValBuilder.from_reverse(Val::Func(func.clone()), output.clone());
    let input = solver.eval(ctx, reverse);
    let Val::Answer(answer) = &input else {
        return Val::default();
    };
    let Answer::Verified(prop) = &**answer else {
        return input;
    };
    if *prop.func() != func || *prop.output() != output || !prop.proved() {
        return Val::default();
    }
    input
}

pub(crate) const SOLVER: &str = "solver";
