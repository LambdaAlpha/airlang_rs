use crate::{
    answer::Answer,
    ctx::ref1::{
        CtxMeta,
        CtxRef,
    },
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
    let Ok(solver) = ctx.reborrow().get_solver_dyn() else {
        return none;
    };
    let ask = Ask::new(Val::Func(func.clone()), output.clone());
    let ask = Val::Ask(ask.into());
    let answer = if solver.is_const {
        solver.ref1.transform(ask)
    } else {
        solver.ref1.transform_mut(ask)
    };
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
