use crate::{
    AskMode,
    AskVal,
    CallMode,
    CallVal,
    CommentMode,
    CommentVal,
    CompositeMode,
    ListMode,
    ListVal,
    MapMode,
    MapVal,
    PairMode,
    PairVal,
    PrimitiveMode,
    Symbol,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SelfMode {
    #[default]
    Self1,
    Primitive(PrimitiveMode),
}

impl Transformer<Val, Val> for CompositeMode<SelfMode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

#[derive(Copy, Clone)]
struct SelfTrans<'a> {
    self1: &'a CompositeMode<SelfMode>,
    mode: SelfMode,
}

impl Transformer<Val, Val> for SelfTrans<'_> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.mode {
            SelfMode::Self1 => FormCore::transform_val(self.self1, ctx, input),
            SelfMode::Primitive(mode) => mode.transform(ctx, input),
        }
    }
}

impl<'a> SelfTrans<'a> {
    fn new(self1: &'a CompositeMode<SelfMode>, mode: SelfMode) -> Self {
        Self { self1, mode }
    }
}

impl ByVal<Val> for CompositeMode<SelfMode> {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.symbol.transform(ctx, s)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.pair {
            PairMode::Id => Id.transform_pair(ctx, pair),
            PairMode::Form(mode) => {
                let first = SelfTrans::new(self, mode.first);
                let second = SelfTrans::new(self, mode.second);
                FormCore::transform_pair(&first, &second, ctx, pair)
            }
        }
    }

    fn transform_comment<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.comment {
            CommentMode::Id => Id.transform_comment(ctx, comment),
            CommentMode::Form(mode) => {
                let meta = SelfTrans::new(self, mode.meta);
                let value = SelfTrans::new(self, mode.value);
                FormCore::transform_comment(&meta, &value, ctx, comment)
            }
            CommentMode::Eval(mode) => {
                let meta = SelfTrans::new(self, mode.meta);
                let value = SelfTrans::new(self, mode.value);
                EvalCore::transform_comment(&meta, &value, ctx, comment)
            }
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match &self.list {
            ListMode::Id => Id.transform_list(ctx, list),
            ListMode::Form { head, tail } => {
                let head = head
                    .iter()
                    .map(|mode| SelfTrans::new(self, *mode))
                    .collect();
                let tail = &SelfTrans::new(self, *tail);
                FormCore::transform_list_head_tail(&head, tail, ctx, list)
            }
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match &self.map {
            MapMode::Id => Id.transform_map(ctx, map),
            MapMode::Form { some, else1 } => {
                let some = some
                    .iter()
                    .map(|(k, v)| (k, SelfTrans::new(self, *v)))
                    .collect();
                let key = SelfTrans::new(self, else1.first);
                let value = SelfTrans::new(self, else1.second);
                FormCore::transform_map_some_else(&some, &key, &value, ctx, map)
            }
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.call {
            CallMode::Id => Id.transform_call(ctx, call),
            CallMode::Form(mode) => {
                let func = SelfTrans::new(self, mode.func);
                let input = SelfTrans::new(self, mode.input);
                FormCore::transform_call(&func, &input, ctx, call)
            }
            CallMode::Eval(mode) => {
                let func = SelfTrans::new(self, mode.func);
                let input = SelfTrans::new(self, mode.input);
                EvalCore::transform_call(&func, &input, ctx, call)
            }
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.ask {
            AskMode::Id => Id.transform_ask(ctx, ask),
            AskMode::Form(mode) => {
                let func = SelfTrans::new(self, mode.func);
                let output = SelfTrans::new(self, mode.output);
                FormCore::transform_ask(&func, &output, ctx, ask)
            }
            AskMode::Eval(mode) => {
                let func = SelfTrans::new(self, mode.func);
                let output = SelfTrans::new(self, mode.output);
                EvalCore::transform_ask(&func, &output, ctx, ask)
            }
        }
    }
}

impl From<PrimitiveMode> for SelfMode {
    fn from(value: PrimitiveMode) -> Self {
        Self::Primitive(value)
    }
}
