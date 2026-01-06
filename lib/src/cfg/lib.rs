use std::rc::Rc;

use self::bit::BitLib;
use self::byte::ByteLib;
use self::call::CallLib;
use self::cell::CellLib;
use self::cfg::CfgLib;
use self::ctrl::CtrlLib;
use self::ctx::CtxLib;
use self::decimal::DecimalLib;
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
        self.lang.extend(cfg);
    }
}

pub struct FreePrimFn<F> {
    pub raw_input: bool,
    pub f: F,
}

pub struct DynPrimFn<F> {
    pub raw_input: bool,
    pub f: F,
}

impl<F: FreeFn<Cfg, Val, Val> + 'static> FreePrimFn<F> {
    pub fn free(self) -> FreePrimFuncVal {
        let func = FreePrimFunc { raw_input: self.raw_input, fn_: Rc::new(self.f) };
        FreePrimFuncVal::from(func)
    }
}

impl<F: ConstFn<Cfg, Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn const_(self) -> ConstPrimFuncVal {
        let func = ConstPrimFunc { raw_input: self.raw_input, fn_: Rc::new(self.f) };
        ConstPrimFuncVal::from(func)
    }
}

impl<F: MutFn<Cfg, Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn mut_(self) -> MutPrimFuncVal {
        let func = MutPrimFunc { raw_input: self.raw_input, fn_: Rc::new(self.f) };
        MutPrimFuncVal::from(func)
    }
}

pub struct FreeImpl<Cfg, I, O> {
    pub free: fn(&mut Cfg, I) -> O,
}

impl<Cfg, I, O> FreeFn<Cfg, I, O> for FreeImpl<Cfg, I, O> {
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (self.free)(cfg, input)
    }
}

impl<Cfg, I, O> FreeImpl<Cfg, I, O> {
    pub fn new(free: fn(&mut Cfg, I) -> O) -> Self {
        Self { free }
    }

    pub fn default(_cfg: &mut Cfg, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct ConstImpl<Cfg, Ctx, I, O> {
    pub free: fn(&mut Cfg, I) -> O,
    pub const_: fn(&mut Cfg, ConstRef<Ctx>, I) -> O,
}

impl<Cfg, Ctx, I, O> FreeFn<Cfg, I, O> for ConstImpl<Cfg, Ctx, I, O> {
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (self.free)(cfg, input)
    }
}

impl<Cfg, Ctx, I, O> ConstFn<Cfg, Ctx, I, O> for ConstImpl<Cfg, Ctx, I, O> {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O> ConstImpl<Cfg, Ctx, I, O> {
    pub fn new(free: fn(&mut Cfg, I) -> O, const_: fn(&mut Cfg, ConstRef<Ctx>, I) -> O) -> Self {
        Self { free, const_ }
    }

    pub fn default(_cfg: &mut Cfg, _ctx: ConstRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct MutImpl<Cfg, Ctx, I, O> {
    pub free: fn(&mut Cfg, I) -> O,
    pub const_: fn(&mut Cfg, ConstRef<Ctx>, I) -> O,
    pub mut_: fn(&mut Cfg, &mut Ctx, I) -> O,
}

impl<Cfg, Ctx, I, O> FreeFn<Cfg, I, O> for MutImpl<Cfg, Ctx, I, O> {
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (self.free)(cfg, input)
    }
}

impl<Cfg, Ctx, I, O> ConstFn<Cfg, Ctx, I, O> for MutImpl<Cfg, Ctx, I, O> {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O> MutFn<Cfg, Ctx, I, O> for MutImpl<Cfg, Ctx, I, O> {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (self.mut_)(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O> MutImpl<Cfg, Ctx, I, O> {
    pub fn new(
        free: fn(&mut Cfg, I) -> O, const_: fn(&mut Cfg, ConstRef<Ctx>, I) -> O,
        mut_: fn(&mut Cfg, &mut Ctx, I) -> O,
    ) -> Self {
        Self { free, const_, mut_ }
    }

    pub fn default(_cfg: &mut Cfg, _ctx: &mut Ctx, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct DynImpl<Cfg, Ctx, I, O> {
    pub free: fn(&mut Cfg, I) -> O,
    pub dyn_: fn(&mut Cfg, DynRef<Ctx>, I) -> O,
}

impl<Cfg, Ctx, I, O> FreeFn<Cfg, I, O> for DynImpl<Cfg, Ctx, I, O> {
    fn free_call(&self, cfg: &mut Cfg, input: I) -> O {
        (self.free)(cfg, input)
    }
}

impl<Cfg, Ctx, I, O> ConstFn<Cfg, Ctx, I, O> for DynImpl<Cfg, Ctx, I, O> {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.dyn_)(cfg, ctx.into_dyn(), input)
    }
}

impl<Cfg, Ctx, I, O> MutFn<Cfg, Ctx, I, O> for DynImpl<Cfg, Ctx, I, O> {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (self.dyn_)(cfg, DynRef::new_mut(ctx), input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Ctx>, input: I) -> O {
        (self.dyn_)(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O> DynImpl<Cfg, Ctx, I, O> {
    pub fn new(free: fn(&mut Cfg, I) -> O, dyn_: fn(&mut Cfg, DynRef<Ctx>, I) -> O) -> Self {
        Self { free, dyn_ }
    }

    pub fn default(_cfg: &mut Cfg, _ctx: DynRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub fn free_impl(func: fn(&mut Cfg, Val) -> Val) -> FreeImpl<Cfg, Val, Val> {
    FreeImpl::new(func)
}

pub fn const_impl(func: fn(&mut Cfg, ConstRef<Val>, Val) -> Val) -> ConstImpl<Cfg, Val, Val, Val> {
    ConstImpl::new(FreeImpl::default, func)
}

pub fn mut_impl(func: fn(&mut Cfg, &mut Val, Val) -> Val) -> MutImpl<Cfg, Val, Val, Val> {
    MutImpl::new(FreeImpl::default, ConstImpl::default, func)
}

pub fn dyn_impl(func: fn(&mut Cfg, DynRef<Val>, Val) -> Val) -> DynImpl<Cfg, Val, Val, Val> {
    DynImpl::new(FreeImpl::default, func)
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

pub mod lang;
