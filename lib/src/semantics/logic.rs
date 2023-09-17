use crate::semantics::{
    ctx_access::{
        constant::ConstCtx,
        free::FreeCtx,
        mutable::MutableCtx,
    },
    eval::Evaluator,
    eval_mode::EvalMode,
    Ctx,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prop {
    eval_mode: EvalMode,
    input: Val,
    output: Val,
    ctx: PropCtx,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Theorem {
    prop: Prop,
    truth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PropCtx {
    Free,
    Const(Ctx),
    Mutable(Ctx, Ctx),
}

impl Prop {
    pub(crate) fn new_free(eval_mode: EvalMode, input: Val, output: Val) -> Self {
        Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Free,
        }
    }

    pub(crate) fn new_const(eval_mode: EvalMode, ctx: Ctx, input: Val, output: Val) -> Self {
        Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Const(ctx),
        }
    }

    pub(crate) fn new_mutable(
        eval_mode: EvalMode,
        before: Ctx,
        input: Val,
        after: Ctx,
        output: Val,
    ) -> Self {
        Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
        }
    }

    pub(crate) fn relax_to_const(&self, ctx: Ctx) -> Option<Self> {
        let PropCtx::Free = &self.ctx else {
            return None;
        };
        let ctx = PropCtx::Const(ctx);
        Some(Self {
            eval_mode: self.eval_mode,
            input: self.input.clone(),
            output: self.output.clone(),
            ctx,
        })
    }

    pub(crate) fn relax_to_mutable(&self, ctx: Option<Ctx>) -> Option<Self> {
        let ctx = match &self.ctx {
            PropCtx::Free => {
                let ctx = ctx?;
                PropCtx::Mutable(ctx.clone(), ctx)
            }
            PropCtx::Const(current) => {
                if ctx.is_some() {
                    return None;
                }
                PropCtx::Mutable(current.clone(), current.clone())
            }
            PropCtx::Mutable(_, _) => return None,
        };
        Some(Self {
            eval_mode: self.eval_mode,
            input: self.input.clone(),
            output: self.output.clone(),
            ctx,
        })
    }

    pub(crate) fn eval_mode(&self) -> EvalMode {
        self.eval_mode
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
}

impl Theorem {
    pub(crate) fn new_free(eval_mode: EvalMode, input: Val) -> Option<Self> {
        let output = eval_mode.eval_free_by_ref(&mut FreeCtx, &input)?;
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Free,
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn new_const(eval_mode: EvalMode, mut ctx: Ctx, input: Val) -> Option<Self> {
        let output = eval_mode.eval_const_by_ref(&mut ConstCtx(&mut ctx), &input)?;
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Const(ctx),
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn new_mutable(eval_mode: EvalMode, before: Ctx, input: Val) -> Option<Self> {
        let mut after = Ctx::clone(&before);
        let output = eval_mode.eval(&mut MutableCtx(&mut after), &input);
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn prove(mut prop: Prop) -> Option<Self> {
        match &mut prop.ctx {
            PropCtx::Free => {
                let Some(real_output) = prop.eval_mode.eval_free_by_ref(&mut FreeCtx, &prop.input)
                else {
                    return None;
                };
                let truth = prop.output == real_output;
                Some(Self { prop, truth })
            }
            PropCtx::Const(before) => {
                let Some(real_output) = prop
                    .eval_mode
                    .eval_const_by_ref(&mut ConstCtx(before), &prop.input)
                else {
                    return None;
                };
                let truth = prop.output == real_output;
                Some(Self { prop, truth })
            }
            PropCtx::Mutable(before, after) => {
                let mut real_after = Ctx::clone(before);
                let real_output = prop
                    .eval_mode
                    .eval(&mut MutableCtx(&mut real_after), &prop.input);
                let truth = prop.output == real_output && *after == real_after;
                Some(Self { prop, truth })
            }
        }
    }

    pub(crate) fn relax_to_const(&self, ctx: Ctx) -> Option<Self> {
        if !self.truth {
            return None;
        }
        let prop = self.prop.relax_to_const(ctx)?;
        Some(Self { prop, truth: true })
    }

    pub(crate) fn relax_to_mutable(&self, ctx: Option<Ctx>) -> Option<Self> {
        if !self.truth {
            return None;
        }
        let prop = self.prop.relax_to_mutable(ctx)?;
        Some(Self { prop, truth: true })
    }

    pub(crate) fn is_true(&self) -> bool {
        self.truth
    }

    pub(crate) fn prop(&self) -> &Prop {
        &self.prop
    }
}
