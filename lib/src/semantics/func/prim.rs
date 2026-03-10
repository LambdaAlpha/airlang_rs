use crate::semantics::cfg::Cfg;
use crate::semantics::func::DynFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::utils::memory::leak_const;

#[derive(Copy, Clone)]
pub struct PrimFunc {
    pub(crate) fn_: &'static dyn DynFunc<Cfg, Val, Val, Val>,
    pub(crate) ctx: PrimCtx,
    pub(crate) input: PrimInput,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PrimCtx {
    Free,
    Const_,
    Mut,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PrimInput {
    Free,
    Aware,
}

impl DynFunc<Cfg, Val, Val, Val> for PrimFunc {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.call(cfg, ctx, input)
    }
}

impl PartialEq for PrimFunc {
    fn eq(&self, other: &PrimFunc) -> bool {
        std::ptr::eq(&self.fn_, &other.fn_) && self.ctx == other.ctx && self.input == other.input
    }
}

impl Eq for PrimFunc {}

pub struct CtxMutInputAwareFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxMutInputAwareFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxMutInputAwareFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Mut, input: PrimInput::Aware }.into()
    }
}

pub struct CtxMutInputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxMutInputFreeFunc<F>
where F: Fn(&mut Cfg, &mut Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
        (self.fn_)(cfg, ctx)
    }
}

impl<F> CtxMutInputFreeFunc<F>
where F: Fn(&mut Cfg, &mut Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Mut, input: PrimInput::Free }.into()
    }
}

pub struct CtxConstInputAwareFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxConstInputAwareFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxConstInputAwareFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Const_, input: PrimInput::Aware }.into()
    }
}

pub struct CtxConstInputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxConstInputFreeFunc<F>
where F: Fn(&mut Cfg, &Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
        (self.fn_)(cfg, ctx)
    }
}

impl<F> CtxConstInputFreeFunc<F>
where F: Fn(&mut Cfg, &Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Const_, input: PrimInput::Free }.into()
    }
}

pub struct CtxFreeInputAwareFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxFreeInputAwareFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, input)
    }
}

impl<F> CtxFreeInputAwareFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Free, input: PrimInput::Aware }.into()
    }
}

pub struct CtxFreeInputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxFreeInputFreeFunc<F>
where F: Fn(&mut Cfg) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
        (self.fn_)(cfg)
    }
}

impl<F> CtxFreeInputFreeFunc<F>
where F: Fn(&mut Cfg) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Free, input: PrimInput::Free }.into()
    }
}
