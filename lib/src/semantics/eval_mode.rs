use crate::{
    semantics::{
        ctx_access::CtxAccessor,
        eval::{
            output::OutputBuilder,
            BoolAndBuilder,
            Evaluator,
            OpValBuilder,
            ValBuilder,
        },
        eval_mode::{
            eval::{
                Eval,
                EvalByRef,
                EvalConst,
                EvalConstByRef,
                EvalConstChecker,
                EvalConstCheckerByRef,
                EvalFree,
                EvalFreeByRef,
                EvalFreeChecker,
                EvalFreeCheckerByRef,
            },
            quote::{
                Quote,
                QuoteByRef,
            },
            value::{
                Value,
                ValueByRef,
                ValueFreeConst,
                ValueFreeConstByRef,
                ValueFreeConstChecker,
            },
        },
        Val,
    },
    types::{
        Call,
        List,
        Map,
        Pair,
        Reverse,
    },
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum BasicEvalMode {
    Value,
    Eval,
    Quote,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Any(BasicEvalMode),
    Symbol(BasicEvalMode),
    Pair(Box<Pair<EvalMode, EvalMode>>),
    Call(Box<Call<EvalMode, EvalMode>>),
    Reverse(Box<Reverse<EvalMode, EvalMode>>),
    List(BasicEvalMode),
    ListForAll(Box<EvalMode>),
    ListForSome(List<ListItemEvalMode>),
    Map(BasicEvalMode),
    MapForSome(Map<Val, EvalMode>),
    MapForAll(Box<Pair<EvalMode, EvalMode>>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct ListItemEvalMode {
    pub(crate) eval_mode: EvalMode,
    pub(crate) ellipsis: bool,
}

pub(crate) const QUOTE: Quote<Eval, Value, ValBuilder> = Quote {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

impl<Ctx> Evaluator<Ctx, Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, &Eval, &Value, &QUOTE)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, &Eval, &Value, &QUOTE, &ValBuilder)
    }
}

pub(crate) const QUOTE_BY_REF: QuoteByRef<EvalByRef, ValueByRef, ValBuilder> = QuoteByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_generic(ctx, input, &EvalByRef, &ValueByRef, &QUOTE_BY_REF)
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalByRef,
            &ValueByRef,
            &QUOTE_BY_REF,
            &ValBuilder,
        )
    }
}

pub(crate) const QUOTE_FREE: Quote<EvalFree, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalFree,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, &EvalFree, &ValueFreeConst, &QUOTE_FREE)
    }
}

impl EvalMode {
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFree,
            &ValueFreeConst,
            &QUOTE_FREE,
            &OpValBuilder,
        )
    }
}

pub(crate) const QUOTE_FREE_BY_REF: QuoteByRef<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalFreeByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeByRef,
            &ValueFreeConstByRef,
            &QUOTE_FREE_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalFreeByRef,
            &ValueFreeConstByRef,
            &QUOTE_FREE_BY_REF,
            &OpValBuilder,
        )
    }
}

pub(crate) const QUOTE_CONST: Quote<EvalConst, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalConst,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, &EvalConst, &ValueFreeConst, &QUOTE_CONST)
    }
}

impl EvalMode {
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConst,
            &ValueFreeConst,
            &QUOTE_CONST,
            &OpValBuilder,
        )
    }
}

pub(crate) const QUOTE_CONST_BY_REF: QuoteByRef<EvalConstByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalConstByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstByRef,
            &ValueFreeConstByRef,
            &QUOTE_CONST_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalConstByRef,
            &ValueFreeConstByRef,
            &QUOTE_CONST_BY_REF,
            &OpValBuilder,
        )
    }
}

pub(crate) const QUOTE_FREE_CHECKER: Quote<EvalFreeChecker, ValueFreeConstChecker, BoolAndBuilder> =
    Quote {
        eval: EvalFreeChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    };

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn is_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeChecker,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeChecker,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER,
            &BoolAndBuilder,
        )
    }
}

pub(crate) const QUOTE_FREE_CHECKER_BY_REF: QuoteByRef<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalFreeCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn is_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalFreeCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

pub(crate) const QUOTE_CONST_CHECKER: Quote<
    EvalConstChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Quote {
    eval: EvalConstChecker,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn is_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstChecker,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstChecker,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER,
            &BoolAndBuilder,
        )
    }
}

pub(crate) const QUOTE_CONST_CHECKER_BY_REF: QuoteByRef<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalConstCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn is_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalConstCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

impl BasicEvalMode {
    fn eval_generic<Ctx, Input, Output, Eval, Value, Quote>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        eval: &Eval,
        value: &Value,
        quote: &Quote,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Quote: Evaluator<Ctx, Input, Output>,
    {
        match self {
            BasicEvalMode::Value => value.eval(ctx, input),
            BasicEvalMode::Eval => eval.eval(ctx, input),
            BasicEvalMode::Quote => quote.eval(ctx, input),
        }
    }
}

impl EvalMode {
    #[allow(clippy::too_many_arguments)]
    fn eval_generic<Ctx, Output, Eval, Value, Quote, Builder>(
        &self,
        ctx: &mut Ctx,
        input: Val,
        eval: &Eval,
        value: &Value,
        quote: &Quote,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Val, Output>,
        Value: Evaluator<Ctx, Val, Output>,
        Quote: Evaluator<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match (self, input) {
            (EvalMode::Any(mode), input) => mode.eval_generic(ctx, input, eval, value, quote),
            (EvalMode::Symbol(mode), Val::Symbol(s)) => {
                mode.eval_generic(ctx, Val::Symbol(s), eval, value, quote)
            }
            (EvalMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first =
                    mode_pair
                        .first
                        .eval_generic(ctx, val_pair.first, eval, value, quote, builder);
                let second = mode_pair.second.eval_generic(
                    ctx,
                    val_pair.second,
                    eval,
                    value,
                    quote,
                    builder,
                );
                builder.from_pair(first, second)
            }
            (EvalMode::Call(mode_call), Val::Call(val_call)) => {
                let func =
                    mode_call
                        .func
                        .eval_generic(ctx, val_call.func, eval, value, quote, builder);
                let input =
                    mode_call
                        .input
                        .eval_generic(ctx, val_call.input, eval, value, quote, builder);
                builder.from_call(func, input)
            }
            (EvalMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval_generic(
                    ctx,
                    val_reverse.func,
                    eval,
                    value,
                    quote,
                    builder,
                );
                let output = mode_reverse.output.eval_generic(
                    ctx,
                    val_reverse.output,
                    eval,
                    value,
                    quote,
                    builder,
                );
                builder.from_reverse(func, output)
            }
            (EvalMode::List(mode), Val::List(val_list)) => {
                mode.eval_generic(ctx, Val::List(val_list), eval, value, quote)
            }
            (EvalMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list
                    .into_iter()
                    .map(|v| mode.eval_generic(ctx, v, eval, value, quote, builder));
                builder.from_list(list)
            }
            (EvalMode::ListForSome(mode_list), Val::List(val_list)) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(mode) = mode_iter.next() {
                    if mode.ellipsis {
                        let name_len = mode_iter.len();
                        let val_len = val_iter.len();
                        if val_len > name_len {
                            for _ in 0..(val_len - name_len) {
                                let val = val_iter.next().unwrap();
                                let val = mode
                                    .eval_mode
                                    .eval_generic(ctx, val, eval, value, quote, builder);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode
                            .eval_mode
                            .eval_generic(ctx, val, eval, value, quote, builder);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(eval.eval(ctx, val));
                }
                builder.from_list(list.into_iter())
            }
            (EvalMode::Map(mode), Val::Map(val_map)) => {
                mode.eval_generic(ctx, Val::Map(val_map), eval, value, quote)
            }
            (EvalMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval_generic(ctx, k, eval, value, quote, builder);
                    let v = mode
                        .second
                        .eval_generic(ctx, v, eval, value, quote, builder);
                    (k, v)
                });
                builder.from_map(map)
            }
            (EvalMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.eval_generic(ctx, v, eval, value, quote, builder)
                    } else {
                        eval.eval(ctx, v)
                    };
                    let k = value.eval(ctx, k);
                    (k, v)
                });
                builder.from_map(map)
            }
            (_, input) => eval.eval(ctx, input),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_by_ref_generic<'a, Ctx, Output, Eval, Value, Quote, Builder>(
        &self,
        ctx: &mut Ctx,
        input: &'a Val,
        eval: &Eval,
        value: &Value,
        quote: &Quote,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, &'a Val, Output>,
        Value: Evaluator<Ctx, &'a Val, Output>,
        Quote: Evaluator<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match (self, input) {
            (EvalMode::Any(mode), input) => mode.eval_generic(ctx, input, eval, value, quote),
            (EvalMode::Symbol(mode), Val::Symbol(_)) => {
                mode.eval_generic(ctx, input, eval, value, quote)
            }
            (EvalMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair.first.eval_by_ref_generic(
                    ctx,
                    &val_pair.first,
                    eval,
                    value,
                    quote,
                    builder,
                );
                let second = mode_pair.second.eval_by_ref_generic(
                    ctx,
                    &val_pair.second,
                    eval,
                    value,
                    quote,
                    builder,
                );
                builder.from_pair(first, second)
            }
            (EvalMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval_by_ref_generic(
                    ctx,
                    &val_call.func,
                    eval,
                    value,
                    quote,
                    builder,
                );
                let input = mode_call.input.eval_by_ref_generic(
                    ctx,
                    &val_call.input,
                    eval,
                    value,
                    quote,
                    builder,
                );
                builder.from_call(func, input)
            }
            (EvalMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval_by_ref_generic(
                    ctx,
                    &val_reverse.func,
                    eval,
                    value,
                    quote,
                    builder,
                );
                let output = mode_reverse.output.eval_by_ref_generic(
                    ctx,
                    &val_reverse.output,
                    eval,
                    value,
                    quote,
                    builder,
                );
                builder.from_reverse(func, output)
            }
            (EvalMode::List(mode), Val::List(_)) => {
                mode.eval_generic(ctx, input, eval, value, quote)
            }
            (EvalMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list
                    .into_iter()
                    .map(|v| mode.eval_by_ref_generic(ctx, v, eval, value, quote, builder));
                builder.from_list(list)
            }
            (EvalMode::ListForSome(mode_list), Val::List(val_list)) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(val) = val_iter.next() {
                    if let Some(mode) = mode_iter.next() {
                        let val = mode
                            .eval_mode
                            .eval_by_ref_generic(ctx, val, eval, value, quote, builder);
                        list.push(val);
                        if mode.ellipsis {
                            let name_len = mode_iter.len();
                            let val_len = val_iter.len();
                            if val_len > name_len {
                                for _ in 0..(val_len - name_len) {
                                    let val = val_iter.next().unwrap();
                                    let val = mode
                                        .eval_mode
                                        .eval_by_ref_generic(ctx, val, eval, value, quote, builder);
                                    list.push(val);
                                }
                            }
                        }
                    } else {
                        list.push(eval.eval(ctx, val));
                    }
                }
                builder.from_list(list.into_iter())
            }
            (EvalMode::Map(mode), Val::Map(_)) => mode.eval_generic(ctx, input, eval, value, quote),
            (EvalMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode
                        .first
                        .eval_by_ref_generic(ctx, k, eval, value, quote, builder);
                    let v = mode
                        .second
                        .eval_by_ref_generic(ctx, v, eval, value, quote, builder);
                    (k, v)
                });
                builder.from_map(map)
            }
            (EvalMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(k) {
                        mode.eval_by_ref_generic(ctx, v, eval, value, quote, builder)
                    } else {
                        eval.eval(ctx, v)
                    };
                    let k = value.eval(ctx, k);
                    (k, v)
                });
                builder.from_map(map)
            }
            (_, input) => eval.eval(ctx, input),
        }
    }
}

pub(crate) mod value;

pub(crate) mod quote;

pub(crate) mod eval;
