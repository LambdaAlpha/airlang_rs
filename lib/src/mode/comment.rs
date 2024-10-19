use crate::{
    Comment,
    CommentVal,
    Mode,
    PrimitiveMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::{
        id::Id,
        recursive::SelfMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommentMode<M> {
    Id,
    Form(Comment<M, M>),
    Eval(M),
}

impl Transformer<CommentVal, Val> for CommentMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            CommentMode::Id => Id.transform_comment(ctx, comment),
            CommentMode::Form(mode) => {
                FormCore::transform_comment(&mode.meta, &mode.value, ctx, comment)
            }
            CommentMode::Eval(mode) => EvalCore::transform_comment(mode, ctx, comment),
        }
    }
}

impl<M: Default> Default for CommentMode<M> {
    fn default() -> Self {
        Self::Eval(M::default())
    }
}

impl From<PrimitiveMode> for CommentMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => CommentMode::Id,
            PrimitiveMode::Form => CommentMode::Form(Comment::new(
                Mode::Primitive(PrimitiveMode::Form),
                Mode::Primitive(PrimitiveMode::Form),
            )),
            PrimitiveMode::Eval => CommentMode::Eval(Mode::Primitive(PrimitiveMode::Eval)),
        }
    }
}

impl From<PrimitiveMode> for CommentMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => CommentMode::Id,
            PrimitiveMode::Form => {
                CommentMode::Form(Comment::new(SelfMode::Self1, SelfMode::Self1))
            }
            PrimitiveMode::Eval => CommentMode::Eval(SelfMode::Self1),
        }
    }
}
