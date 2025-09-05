pub use self::comp::CompMode;
pub use self::func::FuncMode;
pub use self::list::ListMode;
pub use self::map::MapMode;
pub use self::pair::PairMode;
pub use self::prim::PrimMode;
pub use self::symbol::SymbolMode;
pub use self::task::TaskMode;
pub use self::task::TaskPrimMode;

_____!();

use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Comp(CompMode),
    Func(FuncVal),
}

impl FreeFn<Cfg, Val, Val> for Mode {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.free_call(cfg, input),
            Mode::Func(func) => func.free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Mode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.const_call(cfg, ctx, input),
            Mode::Func(func) => func.const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Mode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.mut_call(cfg, ctx, input),
            Mode::Func(func) => func.mut_call(cfg, ctx, input),
        }
    }
}

impl Mode {
    pub const fn id() -> Self {
        Mode::Comp(CompMode::id())
    }

    pub fn is_id(&self) -> bool {
        let Mode::Comp(mode) = self else {
            return false;
        };
        mode.is_id()
    }
}

impl From<PrimMode> for Mode {
    fn from(mode: PrimMode) -> Self {
        Mode::Comp(CompMode::from(mode))
    }
}

pub fn free_mode(mode: Mode) -> FuncMode {
    FuncMode { call: mode, solve: FuncMode::default_mode() }
}

pub fn default_free_mode() -> FuncMode {
    free_mode(FuncMode::default_mode())
}

pub fn dyn_mode(mode: Mode) -> FuncMode {
    FuncMode { call: mode, solve: FuncMode::default_mode() }
}

pub fn default_dyn_mode() -> FuncMode {
    dyn_mode(FuncMode::default_mode())
}

mod prim;

mod comp;

mod func;

mod symbol;

mod pair;

mod task;

mod list;

mod map;
