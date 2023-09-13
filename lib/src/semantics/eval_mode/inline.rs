use crate::{
    semantics::{
        ctx::CtxTrait,
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            output::OutputBuilder,
            DefaultByRef,
            DefaultByVal,
        },
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        Evaluator,
    },
    types::Symbol,
};

pub(crate) struct Inline<Eval, Value, Builder> {
    pub(crate) eval: Eval,
    pub(crate) value: Value,
    pub(crate) builder: Builder,
}

impl<Ctx, Output, Eval, Value, Builder> Evaluator<Ctx, Val, Output> for Inline<Eval, Value, Builder>
where
    Ctx: CtxTrait,
    Eval: ByVal<Ctx, Output>,
    Value: ByVal<Ctx, Output>,
    Builder: OutputBuilder<Output>,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Output {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx, Output, Eval, Value, Builder> ByVal<Ctx, Output> for Inline<Eval, Value, Builder>
where
    Ctx: CtxTrait,
    Eval: ByVal<Ctx, Output>,
    Value: ByVal<Ctx, Output>,
    Builder: OutputBuilder<Output>,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Output {
        self.value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Output {
        self.value.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Output {
        self.value.eval_ref(ctx, ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Output {
        DefaultByVal::eval_pair(self, ctx, first, second, &self.builder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Output {
        DefaultByVal::eval_list(self, ctx, list, &self.builder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Output {
        DefaultByVal::eval_map(self, ctx, map, &self.builder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Output {
        self.eval.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Output {
        DefaultByVal::eval_reverse(self, ctx, func, output, &self.builder)
    }
}

pub(crate) struct InlineByRef<Eval, Value, Builder> {
    pub(crate) eval: Eval,
    pub(crate) value: Value,
    pub(crate) builder: Builder,
}

impl<'a, Ctx, Output, Eval, Value, Builder> Evaluator<Ctx, &'a Val, Output>
    for InlineByRef<Eval, Value, Builder>
where
    Ctx: CtxTrait,
    Eval: ByRef<'a, Ctx, Output>,
    Value: ByRef<'a, Ctx, Output>,
    Builder: OutputBuilder<Output>,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Output {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx, Output, Eval, Value, Builder> ByRef<'a, Ctx, Output>
    for InlineByRef<Eval, Value, Builder>
where
    Ctx: CtxTrait,
    Eval: ByRef<'a, Ctx, Output>,
    Value: ByRef<'a, Ctx, Output>,
    Builder: OutputBuilder<Output>,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Output {
        self.value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Output {
        self.value.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Output {
        self.value.eval_ref(ctx, ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Output {
        DefaultByRef::eval_pair(self, ctx, first, second, &self.builder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Output {
        DefaultByRef::eval_list(self, ctx, list, &self.builder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Output {
        DefaultByRef::eval_map(self, ctx, map, &self.builder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Output {
        self.eval.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Output {
        DefaultByRef::eval_reverse(self, ctx, func, output, &self.builder)
    }
}
