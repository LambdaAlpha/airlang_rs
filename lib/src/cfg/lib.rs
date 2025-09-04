use std::rc::Rc;

use self::bit::BitLib;
use self::byte::ByteLib;
use self::call::CallLib;
use self::cfg::CfgLib;
use self::ctrl::CtrlLib;
use self::ctx::CtxLib;
use self::func::FuncLib;
use self::int::IntLib;
use self::link::LinkLib;
use self::list::ListLib;
use self::map::MapLib;
use self::meta::MetaLib;
use self::mode::ModeLib;
use self::number::NumberLib;
use self::pair::PairLib;
use self::symbol::SymbolLib;
use self::syntax::SyntaxLib;
use self::text::TextLib;
use self::unit::UnitLib;
use self::value::ValueLib;
use super::Named;
use crate::cfg::CfgMod;
use crate::cfg::mode::FuncMode;
use crate::cfg::mode::Mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Symbol;

pub trait Library: CfgMod {
    fn prelude(&self, ctx: &mut Ctx);
}

#[derive(Default, Clone)]
pub struct CoreLib {
    pub unit: UnitLib,
    pub bool: BitLib,
    pub symbol: SymbolLib,
    pub text: TextLib,
    pub int: IntLib,
    pub number: NumberLib,
    pub byte: ByteLib,
    pub pair: PairLib,
    pub call: CallLib,
    pub list: ListLib,
    pub map: MapLib,
    pub link: LinkLib,
    pub cfg: CfgLib,
    pub ctx: CtxLib,
    pub func: FuncLib,
    pub meta: MetaLib,
    pub syntax: SyntaxLib,
    pub value: ValueLib,
    pub ctrl: CtrlLib,
    pub mode: ModeLib,
}

impl CfgMod for CoreLib {
    fn extend(self, cfg: &Cfg) {
        self.unit.extend(cfg);
        self.bool.extend(cfg);
        self.symbol.extend(cfg);
        self.text.extend(cfg);
        self.int.extend(cfg);
        self.number.extend(cfg);
        self.byte.extend(cfg);
        self.pair.extend(cfg);
        self.call.extend(cfg);
        self.list.extend(cfg);
        self.map.extend(cfg);
        self.link.extend(cfg);
        self.cfg.extend(cfg);
        self.ctx.extend(cfg);
        self.func.extend(cfg);
        self.meta.extend(cfg);
        self.syntax.extend(cfg);
        self.value.extend(cfg);
        self.ctrl.extend(cfg);
        self.mode.extend(cfg);
    }
}

impl Library for CoreLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.unit.prelude(ctx);
        self.bool.prelude(ctx);
        self.symbol.prelude(ctx);
        self.text.prelude(ctx);
        self.int.prelude(ctx);
        self.number.prelude(ctx);
        self.byte.prelude(ctx);
        self.pair.prelude(ctx);
        self.call.prelude(ctx);
        self.list.prelude(ctx);
        self.map.prelude(ctx);
        self.link.prelude(ctx);
        self.cfg.prelude(ctx);
        self.ctx.prelude(ctx);
        self.func.prelude(ctx);
        self.meta.prelude(ctx);
        self.syntax.prelude(ctx);
        self.value.prelude(ctx);
        self.ctrl.prelude(ctx);
        self.mode.prelude(ctx);
    }
}

impl<T: Named + Clone + Into<FuncVal>> Library for T {
    fn prelude(&self, ctx: &mut Ctx) {
        let v = ctx.put(self.name(), Val::Func(self.clone().into()), Contract::None);
        assert!(matches!(v, Ok(None)), "names of preludes should be unique");
    }
}

pub struct FreePrimFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: Mode,
}

pub struct DynPrimFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: Mode,
}

impl<F: FreeFn<Cfg, Val, Val> + 'static> FreePrimFn<F> {
    pub fn free(self) -> FreePrimFuncVal {
        let func = FreePrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(FuncMode::mode_into_func(self.mode)),
        };
        FreePrimFuncVal::from(func)
    }
}

impl<F: ConstFn<Cfg, Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn const_(self) -> ConstPrimFuncVal {
        let func = ConstPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(FuncMode::mode_into_func(self.mode)),
        };
        ConstPrimFuncVal::from(func)
    }
}

impl<F: MutFn<Cfg, Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn mut_(self) -> MutPrimFuncVal {
        let func = MutPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(FuncMode::mode_into_func(self.mode)),
        };
        MutPrimFuncVal::from(func)
    }
}

fn ctx_put_func<V: Clone + Into<FuncVal>>(ctx: &mut Ctx, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let v = ctx.put(name, Val::Func(val.clone().into()), Contract::None);
    assert!(matches!(v, Ok(None)), "names of preludes should be unique");
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

pub mod unit;

pub mod bit;

pub mod symbol;

pub mod text;

pub mod int;

pub mod number;

pub mod byte;

pub mod pair;

pub mod call;

pub mod list;

pub mod map;

pub mod link;

pub mod cfg;

pub mod ctx;

pub mod func;

// -----

pub mod meta;

pub mod syntax;

pub mod value;

pub mod ctrl;

pub mod mode;
