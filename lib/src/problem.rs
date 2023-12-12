use {
    crate::{
        ctx_access::CtxAccessor,
        eval::Evaluator,
        val::func::FuncVal,
        Val,
    },
    std::assert_matches::debug_assert_matches,
};

pub(crate) fn solve<Ctx: CtxAccessor>(ctx: &mut Ctx, reverse: Val) -> Val {
    debug_assert_matches!(reverse, Val::Reverse(_));
    let Ok(meta) = ctx.get_meta() else {
        return reverse;
    };
    let Ok(solver) = meta.get(SOLVER) else {
        return reverse;
    };
    let Val::Func(FuncVal(solver)) = solver else {
        return Val::default();
    };
    solver.eval(ctx, reverse)
}

pub(crate) const SOLVER: &str = "solver";
