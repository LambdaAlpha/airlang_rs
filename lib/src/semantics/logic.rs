use crate::{
    semantics::{
        ctx::{
            InvariantTag,
            TaggedVal,
        },
        ctx_access::{
            constant::ConstCtx,
            free::FreeCtx,
            mutable::MutableCtx,
        },
        eval::Evaluator,
        eval_mode::EvalMode,
        val::{
            CtxVal,
            RefVal,
        },
        Ctx,
        Val,
    },
    types::Keeper,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prop {
    eval_mode: EvalMode,
    input: RefVal,
    output: RefVal,
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
    Const(RefVal),
    Mutable(RefVal, RefVal),
}

impl Prop {
    pub(crate) fn new_free(eval_mode: EvalMode, input: RefVal, output: RefVal) -> Option<Self> {
        if !Self::verify_val(&input) || !Self::verify_val(&output) {
            return None;
        }
        Some(Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Free,
        })
    }

    pub(crate) fn new_const(
        eval_mode: EvalMode,
        ctx: RefVal,
        input: RefVal,
        output: RefVal,
    ) -> Option<Self> {
        if !Self::verify_val(&input) || !Self::verify_val(&output) || !Self::verify_ctx(&ctx) {
            return None;
        }
        Some(Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Const(ctx),
        })
    }

    pub(crate) fn new_mutable(
        eval_mode: EvalMode,
        before: RefVal,
        input: RefVal,
        after: RefVal,
        output: RefVal,
    ) -> Option<Self> {
        if !Self::verify_val(&input)
            || !Self::verify_val(&output)
            || !Self::verify_ctx(&before)
            || !Self::verify_ctx(&after)
        {
            return None;
        }
        Some(Self {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
        })
    }

    pub(crate) fn relax_to_const(&self, ctx: RefVal) -> Option<Self> {
        let PropCtx::Free = &self.ctx else {
            return None;
        };
        if !Self::verify_ctx(&ctx) {
            return None;
        }
        let ctx = PropCtx::Const(ctx);
        Some(Self {
            eval_mode: self.eval_mode,
            input: self.input.clone(),
            output: self.output.clone(),
            ctx,
        })
    }

    pub(crate) fn relax_to_mutable(&self, ctx: Option<RefVal>) -> Option<Self> {
        let ctx = match &self.ctx {
            PropCtx::Free => {
                let ctx = ctx?;
                if !Self::verify_ctx(&ctx) {
                    return None;
                }
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

    fn verify_val(val: &RefVal) -> bool {
        let Ok(owner) = Keeper::owner(&val.0) else {
            return false;
        };
        if !matches!(owner.tag, InvariantTag::Const) {
            return false;
        }
        true
    }

    fn verify_ctx(ctx: &RefVal) -> bool {
        let Ok(mut ctx_owner) = Keeper::owner(&ctx.0) else {
            return false;
        };
        if !matches!(ctx_owner.tag, InvariantTag::Const) {
            return false;
        }
        let Val::Ctx(_) = &mut ctx_owner.val else {
            return false;
        };
        true
    }

    pub(crate) fn eval_mode(&self) -> EvalMode {
        self.eval_mode
    }

    pub(crate) fn input(&self) -> &RefVal {
        &self.input
    }

    pub(crate) fn output(&self) -> &RefVal {
        &self.output
    }

    pub(crate) fn ctx(&self) -> &PropCtx {
        &self.ctx
    }
}

impl Theorem {
    pub(crate) fn new_free(eval_mode: EvalMode, input: RefVal) -> Option<Self> {
        let Ok(owner) = Keeper::owner(&input.0) else {
            return None;
        };
        if !matches!(owner.tag, InvariantTag::Const) {
            return None;
        }
        let output = eval_mode.eval_free_by_ref(&mut FreeCtx, &owner.val)?;
        let output = RefVal::from(Keeper::new(TaggedVal::new_const(output)));
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Free,
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn new_const(eval_mode: EvalMode, ctx_ref: RefVal, input: RefVal) -> Option<Self> {
        let Ok(input_owner) = Keeper::owner(&input.0) else {
            return None;
        };
        if !matches!(input_owner.tag, InvariantTag::Const) {
            return None;
        }
        let Ok(mut ctx_owner) = Keeper::owner(&ctx_ref.0) else {
            return None;
        };
        if !matches!(ctx_owner.tag, InvariantTag::Const) {
            return None;
        }
        let Val::Ctx(CtxVal(ctx)) = &mut ctx_owner.val else {
            return None;
        };
        let output = eval_mode.eval_const_by_ref(&mut ConstCtx(ctx), &input_owner.val)?;
        let output = RefVal::from(Keeper::new(TaggedVal::new_const(output)));
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Const(ctx_ref),
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn new_mutable(eval_mode: EvalMode, before: RefVal, input: RefVal) -> Option<Self> {
        let Ok(input_owner) = Keeper::owner(&input.0) else {
            return None;
        };
        if !matches!(input_owner.tag, InvariantTag::Const) {
            return None;
        }
        let Ok(ctx_owner) = Keeper::owner(&before.0) else {
            return None;
        };
        if !matches!(ctx_owner.tag, InvariantTag::Const) {
            return None;
        }
        let Val::Ctx(CtxVal(input_ctx)) = &ctx_owner.val else {
            return None;
        };
        let mut after = Ctx::clone(input_ctx);
        let output = eval_mode.eval(&mut MutableCtx(&mut after), &input_owner.val);
        let output = RefVal::from(Keeper::new(TaggedVal::new_const(output)));
        let after = Val::Ctx(CtxVal::from(Box::new(after)));
        let after = RefVal::from(Keeper::new(TaggedVal::new_const(after)));
        let prop = Prop {
            eval_mode,
            input,
            output,
            ctx: PropCtx::Mutable(before, after),
        };
        Some(Self { prop, truth: true })
    }

    pub(crate) fn prove(prop: Prop) -> Option<Self> {
        let Ok(input_owner) = Keeper::owner(&prop.input.0) else {
            return None;
        };
        let Ok(output_owner) = Keeper::owner(&prop.output.0) else {
            return None;
        };
        match &prop.ctx {
            PropCtx::Free => {
                let Some(real_output) = prop
                    .eval_mode
                    .eval_free_by_ref(&mut FreeCtx, &input_owner.val)
                else {
                    return None;
                };
                let truth = output_owner.val == real_output;
                Some(Self { prop, truth })
            }
            PropCtx::Const(ctx) => {
                let Ok(mut ctx_owner) = Keeper::owner(&ctx.0) else {
                    return None;
                };
                let Val::Ctx(CtxVal(before)) = &mut ctx_owner.val else {
                    return None;
                };
                let Some(real_output) = prop
                    .eval_mode
                    .eval_const_by_ref(&mut ConstCtx(before), &input_owner.val)
                else {
                    return None;
                };
                let truth = output_owner.val == real_output;
                Some(Self { prop, truth })
            }
            PropCtx::Mutable(before, after) => {
                let Ok(before_owner) = Keeper::owner(&before.0) else {
                    return None;
                };
                let Val::Ctx(CtxVal(before)) = &before_owner.val else {
                    return None;
                };
                let Ok(mut after) = Keeper::owner(&after.0) else {
                    return None;
                };
                let Val::Ctx(CtxVal(after)) = &mut after.val else {
                    return None;
                };
                let mut real_after = Ctx::clone(before);
                let real_output = prop
                    .eval_mode
                    .eval(&mut MutableCtx(&mut real_after), &input_owner.val);
                let truth = output_owner.val == real_output && **after == real_after;
                Some(Self { prop, truth })
            }
        }
    }

    pub(crate) fn relax_to_const(&self, ctx: RefVal) -> Option<Self> {
        if !self.truth {
            return None;
        }
        let prop = self.prop.relax_to_const(ctx)?;
        Some(Self { prop, truth: true })
    }

    pub(crate) fn relax_to_mutable(&self, ctx: Option<RefVal>) -> Option<Self> {
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
