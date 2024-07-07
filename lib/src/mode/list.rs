use crate::{
    ctx::ref1::CtxMeta,
    mode::eval::Eval,
    transformer::Transformer,
    List,
    ListVal,
    Mode,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMode {
    All(Mode),
    Some(List<ListItemMode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemMode {
    pub mode: Mode,
    pub ellipsis: bool,
}

impl Default for ListMode {
    fn default() -> Self {
        ListMode::All(Default::default())
    }
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_list = List::from(val_list);
        match self {
            ListMode::All(mode) => {
                let list: List<Val> = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx.reborrow(), val))
                    .collect();
                Val::List(list.into())
            }
            ListMode::Some(mode_list) => {
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
                                let val = mode.mode.transform(ctx.reborrow(), val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.mode.transform(ctx.reborrow(), val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(Eval.transform(ctx.reborrow(), val));
                }
                Val::List(List::from(list).into())
            }
        }
    }
}
