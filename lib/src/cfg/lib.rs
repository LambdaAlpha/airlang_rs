use std::rc::Rc;

use self::bit::BitLib;
use self::byte::ByteLib;
use self::call::CallLib;
use self::cell::CellLib;
use self::cfg::CfgLib;
use self::ctrl::CtrlLib;
use self::ctx::CtxLib;
use self::decimal::DecimalLib;
use self::error::ErrorLib;
use self::func::FuncLib;
use self::int::IntLib;
use self::key::KeyLib;
use self::lang::LangLib;
use self::link::LinkLib;
use self::list::ListLib;
use self::map::MapLib;
use self::pair::PairLib;
use self::resource::ResourceLib;
use self::text::TextLib;
use self::unit::UnitLib;
use self::value::ValueLib;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;

#[derive(Default, Clone)]
pub struct CoreLib {
    pub unit: UnitLib,
    pub bit: BitLib,
    pub key: KeyLib,
    pub text: TextLib,
    pub int: IntLib,
    pub decimal: DecimalLib,
    pub byte: ByteLib,
    pub cell: CellLib,
    pub pair: PairLib,
    pub call: CallLib,
    pub list: ListLib,
    pub map: MapLib,
    pub link: LinkLib,
    pub cfg: CfgLib,
    pub func: FuncLib,
    pub ctx: CtxLib,
    pub ctrl: CtrlLib,
    pub value: ValueLib,
    pub resource: ResourceLib,
    pub error: ErrorLib,
    pub lang: LangLib,
}

impl CfgMod for CoreLib {
    fn extend(self, cfg: &Cfg) {
        self.unit.extend(cfg);
        self.bit.extend(cfg);
        self.key.extend(cfg);
        self.text.extend(cfg);
        self.int.extend(cfg);
        self.decimal.extend(cfg);
        self.byte.extend(cfg);
        self.cell.extend(cfg);
        self.pair.extend(cfg);
        self.call.extend(cfg);
        self.list.extend(cfg);
        self.map.extend(cfg);
        self.link.extend(cfg);
        self.cfg.extend(cfg);
        self.func.extend(cfg);
        self.ctx.extend(cfg);
        self.ctrl.extend(cfg);
        self.value.extend(cfg);
        self.resource.extend(cfg);
        self.error.extend(cfg);
        self.lang.extend(cfg);
    }
}

pub struct FreeImpl<Free> {
    pub free: Free,
}

impl<Free> FreeFn<Cfg, Val, Val> for FreeImpl<Free>
where Free: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        (self.free)(cfg, input)
    }
}

impl<Free> FreeImpl<Free>
where Free: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self) -> FreePrimFuncVal {
        FreePrimFunc { raw_input: false, fn_: Rc::new(self) }.into()
    }

    pub fn build_with(self, raw_input: bool) -> FreePrimFuncVal {
        FreePrimFunc { raw_input, fn_: Rc::new(self) }.into()
    }
}

pub struct ConstImpl<Free, Const> {
    pub free: Free,
    pub const_: Const,
}

impl<Free, Const> FreeFn<Cfg, Val, Val> for ConstImpl<Free, Const>
where Free: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        (self.free)(cfg, input)
    }
}

impl<Free, Const> ConstFn<Cfg, Val, Val, Val> for ConstImpl<Free, Const>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Const: Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static,
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        (self.const_)(cfg, ctx, input)
    }
}

impl<Free, Const> ConstImpl<Free, Const>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Const: Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static,
{
    pub fn build(self) -> ConstPrimFuncVal {
        ConstPrimFunc { raw_input: false, fn_: Rc::new(self) }.into()
    }

    pub fn build_with(self, raw_input: bool) -> ConstPrimFuncVal {
        ConstPrimFunc { raw_input, fn_: Rc::new(self) }.into()
    }
}

pub struct MutImpl<Free, Const, Mut> {
    pub free: Free,
    pub const_: Const,
    pub mut_: Mut,
}

impl<Free, Const, Mut> FreeFn<Cfg, Val, Val> for MutImpl<Free, Const, Mut>
where Free: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        (self.free)(cfg, input)
    }
}

impl<Free, Const, Mut> ConstFn<Cfg, Val, Val, Val> for MutImpl<Free, Const, Mut>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Const: Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static,
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        (self.const_)(cfg, ctx, input)
    }
}

impl<Free, Const, Mut> MutFn<Cfg, Val, Val, Val> for MutImpl<Free, Const, Mut>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Const: Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static,
    Mut: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.mut_)(cfg, ctx, input)
    }
}

impl<Free, Const, Mut> MutImpl<Free, Const, Mut>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Const: Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static,
    Mut: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static,
{
    pub fn build(self) -> MutPrimFuncVal {
        MutPrimFunc { raw_input: false, fn_: Rc::new(self) }.into()
    }

    pub fn build_with(self, raw_input: bool) -> MutPrimFuncVal {
        MutPrimFunc { raw_input, fn_: Rc::new(self) }.into()
    }
}

pub struct DynImpl<Free, Dyn> {
    pub free: Free,
    pub dyn_: Dyn,
}

impl<Free, Dyn> FreeFn<Cfg, Val, Val> for DynImpl<Free, Dyn>
where Free: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        (self.free)(cfg, input)
    }
}

impl<Free, Dyn> ConstFn<Cfg, Val, Val, Val> for DynImpl<Free, Dyn>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Dyn: Fn(&mut Cfg, DynRef<Val>, Val) -> Val + 'static,
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        (self.dyn_)(cfg, ctx.into_dyn(), input)
    }
}

impl<Free, Dyn> MutFn<Cfg, Val, Val, Val> for DynImpl<Free, Dyn>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Dyn: Fn(&mut Cfg, DynRef<Val>, Val) -> Val + 'static,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.dyn_)(cfg, DynRef::new_mut(ctx), input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
        (self.dyn_)(cfg, ctx, input)
    }
}

impl<Free, Dyn> DynImpl<Free, Dyn>
where
    Free: Fn(&mut Cfg, Val) -> Val + 'static,
    Dyn: Fn(&mut Cfg, DynRef<Val>, Val) -> Val + 'static,
{
    pub fn build(self) -> MutPrimFuncVal {
        MutPrimFunc { raw_input: false, fn_: Rc::new(self) }.into()
    }

    pub fn build_with(self, raw_input: bool) -> MutPrimFuncVal {
        MutPrimFunc { raw_input, fn_: Rc::new(self) }.into()
    }
}

pub fn abort_free(key: &'static str) -> impl Fn(&mut Cfg, Val) -> Val + 'static {
    move |cfg: &mut Cfg, _val: Val| {
        let msg = format!("function {key} should not be called in a free context");
        abort_bug_with_msg(cfg, &msg)
    }
}

pub fn abort_const(key: &'static str) -> impl Fn(&mut Cfg, ConstRef<Val>, Val) -> Val + 'static {
    move |cfg: &mut Cfg, _ctx: ConstRef<Val>, _val: Val| {
        let msg = format!("function {key} should not be called in a constant context");
        abort_bug_with_msg(cfg, &msg)
    }
}

pub mod unit;

pub mod bit;

pub mod key;

pub mod text;

pub mod int;

pub mod decimal;

pub mod byte;

pub mod cell;

pub mod pair;

pub mod call;

pub mod list;

pub mod map;

pub mod link;

pub mod cfg;

pub mod func;

// -----

pub mod ctx;

pub mod ctrl;

pub mod value;

pub mod resource;

pub mod error;

pub mod lang;
