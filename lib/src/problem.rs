use crate::{
    answer::Answer,
    ctx::ref1::{
        CtxMeta,
        CtxRef,
    },
    transformer::Transformer,
    val::func::FuncVal,
    Ask,
    Symbol,
    Val,
};

pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Ok(meta) = ctx.reborrow().get_meta() else {
        return Val::default();
    };
    let Ok(solver) = meta.get_ref(Symbol::from_str(SOLVER)) else {
        return Val::default();
    };
    let Val::Func(solver) = solver.clone() else {
        return Val::default();
    };
    let ask = Ask::new(Val::Func(func.clone()), output.clone());
    let ask = Val::Ask(ask.into());
    let input = solver.transform(ctx, ask);
    let Val::Answer(answer) = &input else {
        return Val::default();
    };
    let Answer::Cache(cache) = &**answer else {
        return input;
    };
    let Val::Func(cache_func) = &cache.func else {
        return Val::default();
    };
    if *cache_func != func || cache.output != output {
        return Val::default();
    }
    input
}

pub(crate) const SOLVER: &str = "solver";
