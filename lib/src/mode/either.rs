use crate::{
    EitherVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EitherMode {
    pub this: Option<Mode>,
    pub that: Option<Mode>,
}

impl Transformer<EitherVal, Val> for EitherMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, either: EitherVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_either(&self.this, &self.that, ctx, either)
    }
}

impl From<UniMode> for EitherMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { this: m.clone(), that: m }
    }
}
