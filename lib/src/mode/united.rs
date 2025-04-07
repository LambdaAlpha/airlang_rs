use const_format::concatcp;

use crate::{
    AbstractVal,
    CallVal,
    ChangeVal,
    CodeMode,
    EquivVal,
    GenerateVal,
    InverseVal,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    SymbolMode,
    Val,
    ctx::ref1::CtxMeta,
    mode::{
        eval::Eval,
        form::Form,
        symbol::{
            LITERAL_CHAR,
            MOVE_CHAR,
            REF_CHAR,
        },
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

pub(crate) const FORM: &str = "form";
pub(crate) const EVAL: &str = "eval";

pub(crate) const FORM_LITERAL: &str = concatcp!(FORM, LITERAL_CHAR);
pub(crate) const FORM_REF: &str = concatcp!(FORM, REF_CHAR);
pub(crate) const FORM_MOVE: &str = concatcp!(FORM, MOVE_CHAR);
pub(crate) const EVAL_LITERAL: &str = concatcp!(EVAL, LITERAL_CHAR);
pub(crate) const EVAL_REF: &str = concatcp!(EVAL, REF_CHAR);
pub(crate) const EVAL_MOVE: &str = concatcp!(EVAL, MOVE_CHAR);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UniMode {
    pub code: CodeMode,
    pub symbol: SymbolMode,
}

impl Transformer<Val, Val> for UniMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform(ctx, input),
            CodeMode::Eval => Eval::new(self.symbol).transform(ctx, input),
        }
    }
}

impl ByVal<Val> for UniMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_default(ctx, input),
            CodeMode::Eval => Eval::new(self.symbol).transform_default(ctx, input),
        }
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_symbol(ctx, symbol),
            CodeMode::Eval => Eval::new(self.symbol).transform_symbol(ctx, symbol),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_pair(ctx, pair),
            CodeMode::Eval => Eval::new(self.symbol).transform_pair(ctx, pair),
        }
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_change(ctx, change),
            CodeMode::Eval => Eval::new(self.symbol).transform_change(ctx, change),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_call(ctx, call),
            CodeMode::Eval => Eval::new(self.symbol).transform_call(ctx, call),
        }
    }

    fn transform_equiv<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_equiv(ctx, equiv),
            CodeMode::Eval => Eval::new(self.symbol).transform_equiv(ctx, equiv),
        }
    }

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_inverse(ctx, inverse),
            CodeMode::Eval => Eval::new(self.symbol).transform_inverse(ctx, inverse),
        }
    }

    fn transform_generate<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_generate(ctx, generate),
            CodeMode::Eval => Eval::new(self.symbol).transform_generate(ctx, generate),
        }
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_abstract(ctx, abstract1),
            CodeMode::Eval => Eval::new(self.symbol).transform_abstract(ctx, abstract1),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_list(ctx, list),
            CodeMode::Eval => Eval::new(self.symbol).transform_list(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => Form::new(self.symbol).transform_map(ctx, map),
            CodeMode::Eval => Eval::new(self.symbol).transform_map(ctx, map),
        }
    }
}

impl UniMode {
    pub const fn new(code: CodeMode, symbol: SymbolMode) -> Self {
        Self { code, symbol }
    }
}
