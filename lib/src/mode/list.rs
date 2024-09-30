use crate::{
    List,
    ListVal,
    Mode,
    Val,
    ctx::ref1::CtxMeta,
    transformer::Transformer,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ListMode {
    pub head: List<Mode>,
    pub tail: Mode,
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_list = List::from(val_list);
        let mut list = Vec::with_capacity(val_list.len());
        let mut val_iter = val_list.into_iter();
        for mode in &self.head {
            let Some(val) = val_iter.next() else {
                break;
            };
            let val = mode.transform(ctx.reborrow(), val);
            list.push(val);
        }
        for val in val_iter {
            list.push(self.tail.transform(ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }
}
