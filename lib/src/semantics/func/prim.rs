use std::rc::Rc;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::DynFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct PrimFunc {
    pub(crate) fn_: Rc<dyn DynFunc<Cfg, Val, Val, Val>>,
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
    Raw,
    Eval,
}

impl DynFunc<Cfg, Val, Val, Val> for PrimFunc {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.call(cfg, ctx, input)
    }
}

impl PartialEq for PrimFunc {
    fn eq(&self, other: &PrimFunc) -> bool {
        Rc::ptr_eq(&self.fn_, &other.fn_) && self.ctx == other.ctx && self.input == other.input
    }
}

impl Eq for PrimFunc {}

impl Default for PrimFunc {
    fn default() -> Self {
        struct F;
        impl DynFunc<Cfg, Val, Val, Val> for F {
            fn call(&self, _cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
                Val::default()
            }
        }
        Self { fn_: Rc::new(F), ctx: PrimCtx::Free, input: PrimInput::Free }
    }
}

pub struct CtxMutInputEvalFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxMutInputEvalFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxMutInputEvalFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Mut, input: PrimInput::Eval }.into()
    }
}

pub struct CtxMutInputRawFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxMutInputRawFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxMutInputRawFunc<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Mut, input: PrimInput::Raw }.into()
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
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Mut, input: PrimInput::Free }.into()
    }
}

pub struct CtxConstInputEvalFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxConstInputEvalFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxConstInputEvalFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Const_, input: PrimInput::Eval }.into()
    }
}

pub struct CtxConstInputRawFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxConstInputRawFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> CtxConstInputRawFunc<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Const_, input: PrimInput::Raw }.into()
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
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Const_, input: PrimInput::Free }.into()
    }
}

pub struct CtxFreeInputEvalFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxFreeInputEvalFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, input)
    }
}

impl<F> CtxFreeInputEvalFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Free, input: PrimInput::Eval }.into()
    }
}

pub struct CtxFreeInputRawFunc<F> {
    pub fn_: F,
}

impl<F> DynFunc<Cfg, Val, Val, Val> for CtxFreeInputRawFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, input)
    }
}

impl<F> CtxFreeInputRawFunc<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self) -> PrimFuncVal {
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Free, input: PrimInput::Raw }.into()
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
        PrimFunc { fn_: Rc::new(self), ctx: PrimCtx::Free, input: PrimInput::Free }.into()
    }
}
