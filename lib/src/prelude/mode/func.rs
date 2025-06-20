use std::fmt::Debug;
use std::rc::Rc;

use super::CallMode;
use super::CodeMode;
use super::CompMode;
use super::DataMode;
use super::ListMode;
use super::MapMode;
use super::Mode;
use super::PairMode;
use super::PrimMode;
use super::SymbolMode;
use super::opt::ModeFn;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstStaticPrimFunc;
use crate::semantics::func::FreeStaticPrimFunc;
use crate::semantics::func::MutStaticPrimFunc;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

impl ModeFn for FuncVal {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub forward: Option<Mode>,
    pub reverse: Option<Mode>,
}

const DEFAULT_MODE: PrimMode = PrimMode {
    symbol: Some(SymbolMode::Ref),
    pair: Some(DataMode),
    call: Some(CodeMode::Eval),
    list: Some(DataMode),
    map: Some(DataMode),
};

const MODE_FUNC_ID: &str = "mode.object";

impl FuncMode {
    pub fn into_setup(self) -> Setup {
        Setup {
            forward: Self::mode_into_func(self.forward),
            reverse: Self::mode_into_func(self.reverse),
        }
    }

    pub fn mode_into_func(mode: Option<Mode>) -> FuncVal {
        match mode.ctx_access() {
            CtxAccess::Free => FuncVal::FreeStaticPrim(Self::mode_into_free_func(mode)),
            CtxAccess::Const => FuncVal::ConstStaticPrim(Self::mode_into_const_func(mode)),
            CtxAccess::Mut => FuncVal::MutStaticPrim(Self::mode_into_mut_func(mode)),
        }
    }

    pub fn mode_into_free_func(mode: Option<Mode>) -> FreeStaticPrimFuncVal {
        FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
        }
        .into()
    }

    pub fn mode_into_const_func(mode: Option<Mode>) -> ConstStaticPrimFuncVal {
        ConstStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
            ctx_explicit: false,
        }
        .into()
    }

    pub fn mode_into_mut_func(mode: Option<Mode>) -> MutStaticPrimFuncVal {
        MutStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: None,
            ctx_explicit: false,
        }
        .into()
    }

    pub const fn default_mode() -> Option<Mode> {
        Some(Mode::Prim(DEFAULT_MODE))
    }

    pub const fn default_prim_mode() -> PrimMode {
        DEFAULT_MODE
    }

    pub fn default_comp_mode() -> CompMode {
        CompMode::from(DEFAULT_MODE)
    }

    pub const fn id_mode() -> Option<Mode> {
        None
    }

    pub const fn prim_mode(symbol: SymbolMode, call: CodeMode) -> Option<Mode> {
        Some(Mode::Prim(PrimMode::symbol_call(symbol, call)))
    }

    pub fn symbol_mode(symbol: SymbolMode) -> Option<Mode> {
        let mode = CompMode { symbol: Some(symbol), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn pair_mode(first: Option<Mode>, second: Option<Mode>) -> Option<Mode> {
        let mode = CompMode { pair: Some(PairMode { first, second }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn call_mode(
        func: Option<Mode>, input: Option<Mode>, some: Option<Map<Val, Option<Mode>>>,
    ) -> Option<Mode> {
        let mode =
            CompMode { call: Some(CallMode { func, input, some }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn list_mode(head: List<Option<Mode>>, tail: Option<Mode>) -> Option<Mode> {
        let mode = CompMode { list: Some(ListMode { head, tail }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn map_mode(
        some: Map<Val, Option<Mode>>, key: Option<Mode>, value: Option<Mode>,
    ) -> Option<Mode> {
        let else_ = Pair::new(key, value);
        let mode = CompMode { map: Some(MapMode { some, else_ }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }
}

impl Default for FuncMode {
    fn default() -> Self {
        Self { forward: Self::default_mode(), reverse: Self::default_mode() }
    }
}

trait GetCtxAccess {
    fn ctx_access(&self) -> CtxAccess;
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
            Mode::Prim(mode) => mode.ctx_access(),
            Mode::Comp(mode) => mode.ctx_access(),
            Mode::Func(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for PrimMode {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for DataMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Free
    }
}

impl GetCtxAccess for CodeMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CodeMode::Form => CtxAccess::Free,
            CodeMode::Eval => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for CompMode {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for SymbolMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Mut
    }
}

impl GetCtxAccess for PairMode {
    fn ctx_access(&self) -> CtxAccess {
        self.first.ctx_access() & self.second.ctx_access()
    }
}

impl GetCtxAccess for CallMode {
    fn ctx_access(&self) -> CtxAccess {
        match &self.some {
            None => CtxAccess::Mut,
            Some(some) => {
                let some =
                    some.values().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
                let else_ = self.func.ctx_access() & self.input.ctx_access();
                some & else_
            }
        }
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
        let else_ = self.else_.first.ctx_access() & self.else_.second.ctx_access();
        some & else_
    }
}
