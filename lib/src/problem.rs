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
    Val,
};

pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> AnswerVal
where
    Ctx: CtxMeta<'a>,
{
    let none = AnswerVal::from(Answer::None);
    let Ok(solver) = ctx.reborrow().get_solver() else {
        return none;
    };
    let solver = solver.clone();
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
