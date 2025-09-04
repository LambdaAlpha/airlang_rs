use std::rc::Rc;

use super::CallMode;
use super::CallPrimMode;
use super::CompMode;
use super::ListMode;
use super::MapMode;
use super::Mode;
use super::PairMode;
use super::PrimMode;
use super::SymbolMode;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;

pub struct FuncMode;

const DEFAULT_MODE: PrimMode = PrimMode { symbol: SymbolMode::Ref, call: CallPrimMode::Eval };

const MODE_FUNC_ID: &str = "mode.object";

impl FuncMode {
    pub fn mode_into_func(mode: Mode) -> FuncVal {
        match mode.ctx_access() {
            CtxAccess::Free => FuncVal::FreePrim(Self::mode_into_free_func(mode)),
            CtxAccess::Const => FuncVal::ConstPrim(Self::mode_into_const_func(mode)),
            CtxAccess::Mut => FuncVal::MutPrim(Self::mode_into_mut_func(mode)),
        }
    }

    pub fn mode_into_free_func(mode: Mode) -> FreePrimFuncVal {
        FreePrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
        }
        .into()
    }

    pub fn mode_into_const_func(mode: Mode) -> ConstPrimFuncVal {
        ConstPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
        }
        .into()
    }

    pub fn mode_into_mut_func(mode: Mode) -> MutPrimFuncVal {
        MutPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
        }
        .into()
    }

    pub const fn default_mode() -> Mode {
        let default = DEFAULT_MODE;
        Mode::Comp(CompMode { default, pair: None, call: None, list: None, map: None })
    }

    pub const fn default_prim_mode() -> PrimMode {
        DEFAULT_MODE
    }

    pub fn default_comp_mode() -> CompMode {
        CompMode::from(DEFAULT_MODE)
    }

    pub const fn id_mode() -> Mode {
        Mode::id()
    }

    pub const fn prim_mode(symbol: SymbolMode, call: CallPrimMode) -> Mode {
        let default = PrimMode::new(symbol, call);
        Mode::Comp(CompMode { default, pair: None, call: None, list: None, map: None })
    }

    pub fn pair_mode(some: Map<Val, Mode>, first: Mode, second: Mode) -> Mode {
        let mode = CompMode {
            pair: Some(Box::new(PairMode { some, first, second })),
            ..Self::default_comp_mode()
        };
        Mode::Comp(mode)
    }

    pub fn call_mode(func: Mode, input: Mode) -> Mode {
        let mode = CompMode {
            call: Some(Box::new(CallMode { func, input })),
            ..Self::default_comp_mode()
        };
        Mode::Comp(mode)
    }

    pub fn list_mode(head: List<Mode>, tail: Mode) -> Mode {
        let mode =
            CompMode { list: Some(Box::new(ListMode { head, tail })), ..Self::default_comp_mode() };
        Mode::Comp(mode)
    }

    pub fn map_mode(some: Map<Val, Mode>, else_: Mode) -> Mode {
        let mode =
            CompMode { map: Some(Box::new(MapMode { some, else_ })), ..Self::default_comp_mode() };
        Mode::Comp(mode)
    }
}

trait GetCtxAccess {
    fn ctx_access(&self) -> CtxAccess;
}

impl<T: GetCtxAccess> GetCtxAccess for Box<T> {
    fn ctx_access(&self) -> CtxAccess {
        (**self).ctx_access()
    }
}

impl<T: GetCtxAccess> GetCtxAccess for Option<T> {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            None => CtxAccess::Free,
            Some(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for Mode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            Mode::Comp(mode) => mode.ctx_access(),
            Mode::Func(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for PrimMode {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access() & self.call.ctx_access()
    }
}

impl GetCtxAccess for SymbolMode {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self, SymbolMode::Id) { CtxAccess::Free } else { CtxAccess::Mut }
    }
}

impl GetCtxAccess for CallPrimMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CallPrimMode::Form => CtxAccess::Free,
            CallPrimMode::Eval => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for CompMode {
    fn ctx_access(&self) -> CtxAccess {
        self.default.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for PairMode {
    fn ctx_access(&self) -> CtxAccess {
        let some =
            self.some.values().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
        some & self.first.ctx_access() & self.second.ctx_access()
    }
}

impl GetCtxAccess for CallMode {
    fn ctx_access(&self) -> CtxAccess {
        self.func.ctx_access() & self.input.ctx_access()
    }
}

impl GetCtxAccess for ListMode {
    fn ctx_access(&self) -> CtxAccess {
        let head =
            self.head.iter().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
        let tail = self.tail.ctx_access();
        head & tail
    }
}

impl GetCtxAccess for MapMode {
    fn ctx_access(&self) -> CtxAccess {
        let some =
            self.some.values().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
        let else_ = self.else_.ctx_access();
        some & else_
    }
}
