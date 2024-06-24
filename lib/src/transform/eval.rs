use crate::{
    ctx::{
        ref1::CtxMeta,
        DefaultCtx,
    },
    problem::solve,
    symbol::Symbol,
    transform::{
        id::Id,
        SYMBOL_MOVE_PREFIX,
        SYMBOL_READ_PREFIX,
    },
    transformer::{
        input::ByVal,
        DefaultByVal,
        Transformer,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    Ask,
    AskVal,
    Call,
    CallVal,
    Comment,
    CommentVal,
    PairVal,
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
                let s = Symbol::from_str(&s[1..]);
                ctx.remove(s).unwrap_or_default()
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
        if let Val::Func(func) = &func {
            let input = func.input_mode.transform(ctx.reborrow(), input);
            func.transform(ctx, input)
        } else {
            let input = self.transform(ctx, input);
            let call = Call::new(func, input);
            Val::Call(call.into())
        }
    }

    pub(crate) fn eval_input<'a, Ctx>(&self, ctx: Ctx, func: &Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            func.input_mode.transform(ctx, input)
        } else {
            self.transform(ctx, input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = &func {
            func.transform(ctx, input)
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
            let output = func.output_mode.transform(ctx.reborrow(), output);
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
            f.output_mode.transform(ctx, output)
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
