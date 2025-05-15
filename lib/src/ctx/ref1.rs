use crate::Ctx;
use crate::CtxError;
use crate::ctx::const1::ConstFnCtx;
use crate::ctx::map::CtxMap;
use crate::ctx::map::DynRef;
use crate::ctx::mut1::MutFnCtx;

pub(crate) trait CtxRef<'a> {
    fn get_variables(self) -> Result<&'a CtxMap, CtxError>;

    fn get_variables_mut(self) -> Result<&'a mut CtxMap, CtxError>;

    fn get_variables_dyn(self) -> Result<DynRef<'a, CtxMap>, CtxError>;
}

#[expect(clippy::wrong_self_convention)]
pub(crate) trait CtxMeta<'a>: CtxRef<'a> {
    type Reborrow<'b>: CtxMeta<'b>
    where Self: 'b;

    fn reborrow(&mut self) -> Self::Reborrow<'_>;

    fn borrow(&self) -> Option<&Ctx>;

    #[expect(dead_code)]
    fn is_ctx_free(self) -> bool;

    #[expect(dead_code)]
    fn is_ctx_const(self) -> bool;

    fn for_const_fn(self) -> ConstFnCtx<'a>;

    fn for_mut_fn(self) -> MutFnCtx<'a>;
}
