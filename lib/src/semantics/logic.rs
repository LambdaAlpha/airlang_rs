use {
    crate::semantics::{
        ctx_access::{
            constant::ConstCtx,
            free::FreeCtx,
            mutable::MutableCtx,
        },
        eval::Evaluator,
        func::FuncEval,
        val::FuncVal,
        Ctx,
        Val,
    },
    std::assert_matches::assert_matches,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prop {
    func: FuncVal,
    input: Val,
    output: Val,
    ctx: PropCtx,
    truth: Truth,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PropCtx {
    Free,
    Const(Ctx),
    Mutable(Ctx, Ctx),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Truth {
    None,
    True,
    False,
}

impl Prop {
    pub(crate) fn new_free(func: FuncVal, input: Val, output: Val) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Free(_));
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Free,
            truth: Truth::None,
        }
    }

    pub(crate) fn new_const(func: FuncVal, ctx: Ctx, input: Val, output: Val) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Const(_));
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Const(ctx),
            truth: Truth::None,
        }
    }

    pub(crate) fn new_mutable(
        func: FuncVal,
        before: Ctx,
        input: Val,
        after: Ctx,
        output: Val,
    ) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Mutable(_));
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
            truth: Truth::None,
        }
    }

    pub(crate) fn func(&self) -> &FuncVal {
        &self.func
    }

    pub(crate) fn input(&self) -> &Val {
        &self.input
    }

    pub(crate) fn output(&self) -> &Val {
        &self.output
    }

    pub(crate) fn ctx(&self) -> &PropCtx {
        &self.ctx
    }

    pub(crate) fn new_free_theorem(func: FuncVal, input: Val) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Free(_));
        let output = func.0.evaluator.eval(&mut FreeCtx, input.clone());
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Free,
            truth: Truth::True,
        }
    }

    pub(crate) fn new_const_theorem(func: FuncVal, mut ctx: Ctx, input: Val) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Const(_));
        let output = func
            .0
            .evaluator
            .eval(&mut ConstCtx(&mut ctx), input.clone());
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Const(ctx),
            truth: Truth::True,
        }
    }

    pub(crate) fn new_mutable_theorem(func: FuncVal, before: Ctx, input: Val) -> Self {
        assert_matches!(func.0.evaluator, FuncEval::Mutable(_));
        let mut after = Ctx::clone(&before);
        let output = func
            .0
            .evaluator
            .eval(&mut MutableCtx(&mut after), input.clone());
        Self {
            func,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
            truth: Truth::True,
        }
    }

    pub(crate) fn prove(mut prop: Prop) -> Self {
        match &mut prop.ctx {
            PropCtx::Free => {
                let real_output = prop.func.0.evaluator.eval(&mut FreeCtx, prop.input.clone());
                prop.truth = if prop.output == real_output {
                    Truth::True
                } else {
                    Truth::False
                };
                prop
            }
            PropCtx::Const(before) => {
                let real_output = prop
                    .func
                    .0
                    .evaluator
                    .eval(&mut ConstCtx(before), prop.input.clone());
                prop.truth = if prop.output == real_output {
                    Truth::True
                } else {
                    Truth::False
                };
                prop
            }
            PropCtx::Mutable(before, after) => {
                let mut real_after = Ctx::clone(before);
                let real_output = prop
                    .func
                    .0
                    .evaluator
                    .eval(&mut MutableCtx(&mut real_after), prop.input.clone());
                prop.truth = if prop.output == real_output && *after == real_after {
                    Truth::True
                } else {
                    Truth::False
                };
                prop
            }
        }
    }

    pub(crate) fn truth(&self) -> Truth {
        self.truth
    }
}
