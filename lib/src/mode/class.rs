use crate::{
    Class,
    ClassVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassMode {
    pub class: Class<Option<Mode>>,
}

impl Transformer<ClassVal, Val> for ClassMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, class: ClassVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.class.func;
        FormCore::transform_class(func, ctx, class)
    }
}

impl From<UniMode> for ClassMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { class: Class::new(m) }
    }
}
