use crate::{
    Ask,
    AskVal,
    Call,
    CallVal,
    Comment,
    CommentVal,
    ConstCtx,
    FuncVal,
    MutCtx,
    MutFnCtx,
    PairVal,
    ctx::{
        default::DefaultCtx,
        map::CtxMapRef,
        ref1::CtxMeta,
    },
    mode::{
        SYMBOL_MOVE_PREFIX,
        SYMBOL_READ_PREFIX,
        id::Id,
    },
    problem::solve,
    symbol::Symbol,
    transformer::{
        DefaultByVal,
        Transformer,
        input::ByVal,
    },
    val::{
        Val,
        list::ListVal,
        map::MapVal,
    },
};

#[derive(Copy, Clone)]
pub(crate) struct Eval;

impl Transformer<Val, Val> for Eval {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Eval {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match s.chars().next() {
            Some(Symbol::ID_PREFIX) => {
                let s = Symbol::from_str(&s[1..]);
                Id.transform_symbol(ctx, s)
            }
            Some(SYMBOL_READ_PREFIX) => {
                let s = Symbol::from_str(&s[1..]);
                DefaultCtx.get_or_default(ctx, s)
            }
            Some(SYMBOL_MOVE_PREFIX) => {
                let Ok(variables) = ctx.get_variables_mut() else {
                    return Val::default();
                };
                let s = Symbol::from_str(&s[1..]);
                variables.remove(s).unwrap_or_default()
            }
            _ => DefaultCtx.get_or_default(ctx, s),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_pair(self, ctx, pair)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_map(self, ctx, map)
    }

    fn transform_call<'a, Ctx>(&self, mut ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let call = Call::from(call);
        let func = self.transform(ctx.reborrow(), call.func);
        self.eval_input_then_call(ctx, func, call.input)
    }

    fn transform_ask<'a, Ctx>(&self, mut ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let ask = Ask::from(ask);
        let func = self.transform(ctx.reborrow(), ask.func);
        self.eval_output_then_solve(ctx, func, ask.output)
    }

    fn transform_comment<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let comment = Comment::from(comment);
        self.transform(ctx, comment.value)
    }
}

impl Eval {
    pub(crate) fn eval_input_then_call<'a, Ctx>(&self, mut ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match func {
            Val::Func(mut func) => {
                let input = func.call_mode().transform(ctx.reborrow(), input);
                func.transform_mut(ctx, input)
            }
            Val::Symbol(s) => self.call_free(ctx, s, input),
            _ => {
                let input = self.transform(ctx, input);
                let call = Call::new(func, input);
                Val::Call(call.into())
            }
        }
    }

    pub(crate) fn call_free<'a, Ctx>(&self, ctx: Ctx, func_name: Symbol, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_mut_fn() {
            MutFnCtx::Free(_) => Val::default(),
            MutFnCtx::Const(ctx) => self.call_free_const(ctx, func_name, input),
            MutFnCtx::Mut(ctx) => self.call_free_mut(ctx, func_name, input),
        }
    }

    fn call_free_const(&self, mut ctx: ConstCtx, func_name: Symbol, input: Val) -> Val {
        let Ok(val) = ctx.reborrow().get_ctx_ref().variables().get_ref(func_name) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Free(func)) = val else {
            return Val::default();
        };
        let mut func = func.clone();
        let input = func.call_mode().transform(ctx, input);
        func.transform_mut(input)
    }

    fn call_free_mut(&self, ctx: MutCtx, func_name: Symbol, input: Val) -> Val {
        let ctx = ctx.unwrap();
        let Ok(val) = ctx.variables().get_ref(func_name.clone()) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Free(func)) = val else {
            return Val::default();
        };
        let input = func.call_mode().clone().transform(MutCtx::new(ctx), input);
        let Ok(val) = ctx.variables_mut().get_ref_dyn(func_name) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Free(func)) = val.ref1 else {
            return Val::default();
        };
        if val.is_const {
            func.transform(input)
        } else {
            func.transform_mut(input)
        }
    }

    pub(crate) fn eval_input<'a, Ctx>(&self, ctx: Ctx, func: &Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            func.call_mode().transform(ctx, input)
        } else {
            self.transform(ctx, input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(mut func) = func {
            func.transform_mut(ctx, input)
        } else {
            let call = Call::new(func, input);
            Val::Call(call.into())
        }
    }

    pub(crate) fn eval_output_then_solve<'a, Ctx>(
        &self,
        mut ctx: Ctx,
        func: Val,
        output: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            let output = func.ask_mode().transform(ctx.reborrow(), output);
            Val::Answer(solve(ctx, func, output))
        } else {
            let output = self.transform(ctx, output);
            let ask = Ask::new(func, output);
            Val::Ask(ask.into())
        }
    }

    pub(crate) fn eval_output<'a, Ctx>(&self, ctx: Ctx, func: &Val, output: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(f) = func {
            f.ask_mode().transform(ctx, output)
        } else {
            self.transform(ctx, output)
        }
    }

    pub(crate) fn solve<'a, Ctx>(ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            Val::Answer(solve(ctx, func, output))
        } else {
            let ask = Ask::new(func, output);
            Val::Ask(ask.into())
        }
    }
}
