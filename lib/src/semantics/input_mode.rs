use crate::{
    semantics::{
        ctx_access::CtxAccessor,
        eval::{
            output::OutputBuilder,
            Evaluator,
        },
        eval_mode::{
            mix::{
                Mix,
                MixByRef,
            },
            ByRefEvaluators,
            ByValEvaluators,
            EvalMode,
            BY_REF,
            BY_VAL,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InputMode {
    Any(EvalMode),
    Symbol(EvalMode),
    Pair(Box<Pair<InputMode, InputMode>>),
    Call(Box<Call<InputMode, InputMode>>),
    Reverse(Box<Reverse<InputMode, InputMode>>),
    List(EvalMode),
    ListForAll(Box<InputMode>),
    ListForSome(List<ListItemInputMode>),
    Map(EvalMode),
    MapForAll(Box<Pair<InputMode, InputMode>>),
    MapForSome(Map<Val, InputMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct ListItemInputMode {
    pub(crate) input_mode: InputMode,
    pub(crate) ellipsis: bool,
}

impl<Ctx> Evaluator<Ctx, Val, Val> for InputMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, BY_VAL)
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for InputMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_by_ref_generic(ctx, input, BY_REF)
    }
}

impl InputMode {
    fn eval_generic<Ctx, Output, Eval, Value, Builder>(
        &self,
        ctx: &mut Ctx,
        input: Val,
        evaluators: ByValEvaluators<Eval, Value, Builder>,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Val, Output>,
        Value: Evaluator<Ctx, Val, Output>,
        Mix<Eval, Value, Builder>: Evaluator<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
        ByValEvaluators<Eval, Value, Builder>: Copy,
    {
        match (self, input) {
            (InputMode::Any(mode), input) => mode.eval_generic(ctx, input, evaluators),
            (InputMode::Symbol(mode), Val::Symbol(s)) => {
                mode.eval_generic(ctx, Val::Symbol(s), evaluators)
            }
            (InputMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair
                    .first
                    .eval_generic(ctx, val_pair.first, evaluators);
                let second = mode_pair
                    .second
                    .eval_generic(ctx, val_pair.second, evaluators);
                evaluators.mix.builder.from_pair(first, second)
            }
            (InputMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval_generic(ctx, val_call.func, evaluators);
                let input = mode_call
                    .input
                    .eval_generic(ctx, val_call.input, evaluators);
                evaluators.mix.builder.from_call(func, input)
            }
            (InputMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse
                    .func
                    .eval_generic(ctx, val_reverse.func, evaluators);
                let output = mode_reverse
                    .output
                    .eval_generic(ctx, val_reverse.output, evaluators);
                evaluators.mix.builder.from_reverse(func, output)
            }
            (InputMode::List(mode), Val::List(val_list)) => {
                mode.eval_generic(ctx, Val::List(val_list), evaluators)
            }
            (InputMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list
                    .into_iter()
                    .map(|v| mode.eval_generic(ctx, v, evaluators));
                evaluators.mix.builder.from_list(list)
            }
            (InputMode::ListForSome(mode_list), Val::List(val_list)) => {
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
                                let val = mode.input_mode.eval_generic(ctx, val, evaluators);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.input_mode.eval_generic(ctx, val, evaluators);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(evaluators.eval.eval(ctx, val));
                }
                evaluators.mix.builder.from_list(list.into_iter())
            }
            (InputMode::Map(mode), Val::Map(val_map)) => {
                mode.eval_generic(ctx, Val::Map(val_map), evaluators)
            }
            (InputMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval_generic(ctx, k, evaluators);
                    let v = mode.second.eval_generic(ctx, v, evaluators);
                    (k, v)
                });
                evaluators.mix.builder.from_map(map)
            }
            (InputMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.eval_generic(ctx, v, evaluators)
                    } else {
                        evaluators.eval.eval(ctx, v)
                    };
                    let k = evaluators.value.eval(ctx, k);
                    (k, v)
                });
                evaluators.mix.builder.from_map(map)
            }
            (_, input) => evaluators.eval.eval(ctx, input),
        }
    }

    fn eval_by_ref_generic<'a, Ctx, Output, Eval, Value, Builder>(
        &self,
        ctx: &mut Ctx,
        input: &'a Val,
        evaluators: ByRefEvaluators<Eval, Value, Builder>,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, &'a Val, Output>,
        Value: Evaluator<Ctx, &'a Val, Output>,
        MixByRef<Eval, Value, Builder>: Evaluator<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
        ByRefEvaluators<Eval, Value, Builder>: Copy,
    {
        match (self, input) {
            (InputMode::Any(mode), input) => mode.eval_generic(ctx, input, evaluators),
            (InputMode::Symbol(mode), Val::Symbol(_)) => mode.eval_generic(ctx, input, evaluators),
            (InputMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair
                    .first
                    .eval_by_ref_generic(ctx, &val_pair.first, evaluators);
                let second =
                    mode_pair
                        .second
                        .eval_by_ref_generic(ctx, &val_pair.second, evaluators);
                evaluators.mix.builder.from_pair(first, second)
            }
            (InputMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call
                    .func
                    .eval_by_ref_generic(ctx, &val_call.func, evaluators);
                let input = mode_call
                    .input
                    .eval_by_ref_generic(ctx, &val_call.input, evaluators);
                evaluators.mix.builder.from_call(func, input)
            }
            (InputMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func =
                    mode_reverse
                        .func
                        .eval_by_ref_generic(ctx, &val_reverse.func, evaluators);
                let output =
                    mode_reverse
                        .output
                        .eval_by_ref_generic(ctx, &val_reverse.output, evaluators);
                evaluators.mix.builder.from_reverse(func, output)
            }
            (InputMode::List(mode), Val::List(_)) => mode.eval_generic(ctx, input, evaluators),
            (InputMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list
                    .into_iter()
                    .map(|v| mode.eval_by_ref_generic(ctx, v, evaluators));
                evaluators.mix.builder.from_list(list)
            }
            (InputMode::ListForSome(mode_list), Val::List(val_list)) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(val) = val_iter.next() {
                    if let Some(mode) = mode_iter.next() {
                        let val = mode.input_mode.eval_by_ref_generic(ctx, val, evaluators);
                        list.push(val);
                        if mode.ellipsis {
                            let name_len = mode_iter.len();
                            let val_len = val_iter.len();
                            if val_len > name_len {
                                for _ in 0..(val_len - name_len) {
                                    let val = val_iter.next().unwrap();
                                    let val =
                                        mode.input_mode.eval_by_ref_generic(ctx, val, evaluators);
                                    list.push(val);
                                }
                            }
                        }
                    } else {
                        list.push(evaluators.eval.eval(ctx, val));
                    }
                }
                evaluators.mix.builder.from_list(list.into_iter())
            }
            (InputMode::Map(mode), Val::Map(_)) => mode.eval_generic(ctx, input, evaluators),
            (InputMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval_by_ref_generic(ctx, k, evaluators);
                    let v = mode.second.eval_by_ref_generic(ctx, v, evaluators);
                    (k, v)
                });
                evaluators.mix.builder.from_map(map)
            }
            (InputMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(k) {
                        mode.eval_by_ref_generic(ctx, v, evaluators)
                    } else {
                        evaluators.eval.eval(ctx, v)
                    };
                    let k = evaluators.value.eval(ctx, k);
                    (k, v)
                });
                evaluators.mix.builder.from_map(map)
            }
            (_, input) => evaluators.eval.eval(ctx, input),
        }
    }
}
