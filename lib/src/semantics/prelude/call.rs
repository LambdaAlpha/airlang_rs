use crate::semantics::{
    ctx_access::mutable::CtxForMutableFn,
    eval::input::ByVal,
    eval_mode::{
        eval::Eval,
        BasicEvalMode,
        EvalMode,
    },
    func::{
        CtxMutableFn,
        Primitive,
    },
    prelude::{
        names,
        PrimitiveFunc,
    },
    Val,
};

pub(crate) fn chain() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::CHAIN, fn_chain);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eval.eval_call(&mut ctx, pair.second, pair.first)
}
