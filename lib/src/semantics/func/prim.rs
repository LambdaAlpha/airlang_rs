use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
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

#[derive(Default, Copy, Clone)]
pub enum PrimCtx {
    Free,
    Const_,
    Mut,
    #[default]
    Default,
}

#[derive(Default, Copy, Clone)]
pub enum PrimInput {
    Free,
    #[default]
    Default,
}

impl DynFunc<Cfg, Val, Val, Val> for PrimFunc {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
        self.fn_.call(cfg, ctx, input)
    }
}

pub struct DefaultFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for DefaultFunc<F>
where F: Fn(&mut Cfg, Ctx<Val>, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> DefaultFunc<F>
where F: Fn(&mut Cfg, Ctx<Val>, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Default, input: PrimInput::Default }.into()
    }
}

pub struct InputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for InputFreeFunc<F>
where F: Fn(&mut Cfg, Ctx<Val>) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, _input: Val) -> Val {
        (self.fn_)(cfg, ctx)
    }
}

impl<F> InputFreeFunc<F>
where F: Fn(&mut Cfg, Ctx<Val>) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Default, input: PrimInput::Free }.into()
    }
}

pub struct MutFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for MutFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
        if ctx.const_ {
            return cfg.abort();
        }
        (self.fn_)(cfg, ctx.val, input)
    }
}

impl<F> MutFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Mut, input: PrimInput::Default }.into()
    }
}

pub struct MutInputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for MutInputFreeFunc<F>
where F: Fn(&mut Cfg, &mut Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, _input: Val) -> Val {
        if ctx.const_ {
            return cfg.abort();
        }
        (self.fn_)(cfg, ctx.val)
    }
}

impl<F> MutInputFreeFunc<F>
where F: Fn(&mut Cfg, &mut Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Mut, input: PrimInput::Free }.into()
    }
}

pub struct ConstFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for ConstFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, input: Val) -> Val {
        (self.fn_)(cfg, ctx.val, input)
    }
}

impl<F> ConstFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Const_, input: PrimInput::Default }.into()
    }
}

pub struct ConstInputFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for ConstInputFreeFunc<F>
where F: Fn(&mut Cfg, &Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, _input: Val) -> Val {
        (self.fn_)(cfg, ctx.val)
    }
}

impl<F> ConstInputFreeFunc<F>
where F: Fn(&mut Cfg, &Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Const_, input: PrimInput::Free }.into()
    }
}

pub struct CtxFreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxFreeFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: Ctx<Val>, input: Val) -> Val {
        (self.fn_)(cfg, input)
    }
}

impl<F> CtxFreeFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Free, input: PrimInput::Default }.into()
    }
}

pub struct FreeFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for FreeFunc<F>
where F: Fn(&mut Cfg) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: Ctx<Val>, _input: Val) -> Val {
        (self.fn_)(cfg)
    }
}

impl<F> FreeFunc<F>
where F: Fn(&mut Cfg) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: leak_const(self), ctx: PrimCtx::Free, input: PrimInput::Free }.into()
    }
}
