use crate::{
    ctx::ref1::CtxMeta,
    transformer::Transformer,
    Mode,
    Pair,
    PairVal,
    Val,
};

pub type PairMode = Pair<Mode, Mode>;

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, input: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let input = Pair::from(input);
        let first = self.first.transform(ctx.reborrow(), input.first);
        let second = self.second.transform(ctx, input.second);
        let pair = Pair::new(first, second);
        Val::Pair(pair.into())
    }
}
