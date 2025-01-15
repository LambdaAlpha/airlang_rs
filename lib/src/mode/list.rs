use crate::{
    List,
    ListVal,
    Mode,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMode {
    Id(Id),
    Form { head: List<Mode>, tail: Mode },
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            ListMode::Id(mode) => mode.transform_list(ctx, list),
            ListMode::Form { head, tail } => {
                FormCore::transform_list_head_tail(head, tail, ctx, list)
            }
        }
    }
}

impl Default for ListMode {
    fn default() -> Self {
        Self::Form {
            head: List::default(),
            tail: Mode::default(),
        }
    }
}

impl From<UniMode> for ListMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => ListMode::Id(mode),
            UniMode::Form(mode) => ListMode::Form {
                head: List::default(),
                tail: Mode::Uni(UniMode::Form(mode)),
            },
            UniMode::Eval(mode) => ListMode::Form {
                head: List::default(),
                tail: Mode::Uni(UniMode::Eval(mode)),
            },
        }
    }
}
