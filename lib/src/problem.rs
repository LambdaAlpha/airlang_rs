use crate::{
    answer::Answer,
    ctx::ref1::{
        CtxMeta,
        CtxRef,
    },
    transformer::Transformer,
    val::func::FuncVal,
    AnswerVal,
    Ask,
    Symbol,
    Val,
};

pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> AnswerVal
where
    Ctx: CtxMeta<'a>,
{
    let none = AnswerVal::from(Answer::None);
    let Ok(meta) = ctx.reborrow().get_meta() else {
        return none;
    };
    let Ok(solver) = meta.get_ref(Symbol::from_str(SOLVER)) else {
        return none;
    };
    let Val::Func(solver) = solver.clone() else {
        return none;
    };
    let ask = Ask::new(Val::Func(func.clone()), output.clone());
    let ask = Val::Ask(ask.into());
    let answer = solver.transform(ctx, ask);
    let Val::Answer(answer) = answer else {
        return none;
    };
    let Answer::Cache(cache) = &*answer else {
        return answer;
    };
    let Val::Func(cache_func) = &cache.func else {
        return none;
    };
    if *cache_func != func || cache.output != output {
        return none;
    }
    answer
}

pub(crate) const SOLVER: &str = "solver";
