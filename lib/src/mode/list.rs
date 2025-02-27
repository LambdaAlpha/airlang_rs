use crate::{
    List,
    ListVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListMode {
    pub head: List<Option<Mode>>,
    pub tail: Option<Mode>,
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_list_head_tail(&self.head, &self.tail, ctx, list)
    }
}

impl From<UniMode> for ListMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        ListMode { head: List::default(), tail: m }
    }
}
