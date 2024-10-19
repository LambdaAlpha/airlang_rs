use crate::{
    List,
    ListVal,
    Mode,
    PrimitiveMode,
    Val,
    core::FormCore,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMode<M> {
    Id,
    Form { head: List<M>, tail: M },
}

impl Transformer<ListVal, Val> for ListMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            ListMode::Id => Id.transform_list(ctx, list),
            ListMode::Form { head, tail } => {
                FormCore::transform_list_head_tail(head, tail, ctx, list)
            }
        }
    }
}

impl<M: Default> Default for ListMode<M> {
    fn default() -> Self {
        Self::Form {
            head: List::default(),
            tail: M::default(),
        }
    }
}

impl From<PrimitiveMode> for ListMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => ListMode::Id,
            PrimitiveMode::Form => ListMode::Form {
                head: List::default(),
                tail: Mode::Primitive(PrimitiveMode::Form),
            },
            PrimitiveMode::Eval => ListMode::Form {
                head: List::default(),
                tail: Mode::Primitive(PrimitiveMode::Eval),
            },
        }
    }
}

impl From<PrimitiveMode> for ListMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => ListMode::Id,
            PrimitiveMode::Form => ListMode::Form {
                head: List::default(),
                tail: SelfMode::Self1,
            },
            PrimitiveMode::Eval => ListMode::Form {
                head: List::default(),
                tail: SelfMode::Self1,
            },
        }
    }
}
