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
use crate::semantics::cfg::Cfg;
use crate::semantics::func::CtxFn;
use crate::semantics::func::CtxPrimFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::val::CtxPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;

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

pub struct FreeImpl<F> {
    pub fn_: F,
}

impl<F> FreeFn<Cfg, Val, Val> for FreeImpl<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        (self.fn_)(cfg, input)
    }
}

impl<F> FreeImpl<F>
where F: Fn(&mut Cfg, Val) -> Val + 'static
{
    pub fn build(self, extra: ImplExtra) -> FreePrimFuncVal {
        FreePrimFunc { raw_input: extra.raw_input, fn_: Rc::new(self) }.into()
    }
}

pub struct ConstImpl<F> {
    pub fn_: F,
}

impl<F> CtxFn<Cfg, Val, Val, Val> for ConstImpl<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> ConstImpl<F>
where F: Fn(&mut Cfg, &Val, Val) -> Val + 'static
{
    pub fn build(self, extra: ImplExtra) -> CtxPrimFuncVal {
        CtxPrimFunc { raw_input: extra.raw_input, fn_: Rc::new(self), const_: true }.into()
    }
}

pub struct MutImpl<F> {
    pub fn_: F,
}

impl<F> CtxFn<Cfg, Val, Val, Val> for MutImpl<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        (self.fn_)(cfg, ctx, input)
    }
}

impl<F> MutImpl<F>
where F: Fn(&mut Cfg, &mut Val, Val) -> Val + 'static
{
    pub fn build(self, extra: ImplExtra) -> CtxPrimFuncVal {
        CtxPrimFunc { raw_input: extra.raw_input, fn_: Rc::new(self), const_: false }.into()
    }
}

#[derive(Copy, Clone)]
pub struct ImplExtra {
    pub raw_input: bool,
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
