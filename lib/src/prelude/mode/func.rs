use std::fmt::Debug;
use std::rc::Rc;

use super::CompMode;
use super::ListMode;
use super::MapMode;
use super::Mode;
use super::PairMode;
use super::PrimMode;
use super::SymbolMode;
use super::TaskMode;
use super::TaskPrimMode;
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
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub call: Mode,
    pub solve: Mode,
}

const DEFAULT_MODE: PrimMode = PrimMode { symbol: SymbolMode::Ref, task: TaskPrimMode::Eval };

const MODE_FUNC_ID: &str = "mode.object";

impl FuncMode {
    pub fn mode_into_func(mode: Mode) -> FuncVal {
        match mode.ctx_access() {
            CtxAccess::Free => FuncVal::FreeStaticPrim(Self::mode_into_free_func(mode)),
            CtxAccess::Const => FuncVal::ConstStaticPrim(Self::mode_into_const_func(mode)),
            CtxAccess::Mut => FuncVal::MutStaticPrim(Self::mode_into_mut_func(mode)),
        }
    }

    pub fn mode_into_free_func(mode: Mode) -> FreeStaticPrimFuncVal {
        FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: Setup::none(),
        }
        .into()
    }

    pub fn mode_into_const_func(mode: Mode) -> ConstStaticPrimFuncVal {
        ConstStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: Setup::none(),
        }
        .into()
    }

    pub fn mode_into_mut_func(mode: Mode) -> MutStaticPrimFuncVal {
        MutStaticPrimFunc {
            id: Symbol::from_str_unchecked(MODE_FUNC_ID),
            fn_: Rc::new(mode),
            setup: Setup::none(),
        }
        .into()
    }

    pub const fn default_mode() -> Mode {
        let default = DEFAULT_MODE;
        Mode::Comp(CompMode { default, pair: None, task: None, list: None, map: None })
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

    pub const fn prim_mode(symbol: SymbolMode, task: TaskPrimMode) -> Mode {
        let default = PrimMode::new(symbol, task);
        Mode::Comp(CompMode { default, pair: None, task: None, list: None, map: None })
    }

    pub fn pair_mode(some: Map<Val, Mode>, first: Mode, second: Mode) -> Mode {
        let mode = CompMode {
            pair: Some(Box::new(PairMode { some, first, second })),
            ..Self::default_comp_mode()
        };
        Mode::Comp(mode)
    }

    pub fn task_mode(func: Mode, ctx: Mode, input: Mode) -> Mode {
        let mode = CompMode {
            task: Some(Box::new(TaskMode { func, ctx, input })),
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

    pub(crate) fn into_setup(self) -> Setup {
        Setup {
            call: Some(FuncMode::mode_into_func(self.call)),
            solve: Some(FuncMode::mode_into_func(self.solve)),
        }
    }
}

impl Default for FuncMode {
    fn default() -> Self {
        Self { call: FuncMode::default_mode(), solve: FuncMode::default_mode() }
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
        self.symbol.ctx_access() & self.task.ctx_access()
    }
}

impl GetCtxAccess for SymbolMode {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self, SymbolMode::Id) { CtxAccess::Free } else { CtxAccess::Mut }
    }
}

impl GetCtxAccess for TaskPrimMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            TaskPrimMode::Form => CtxAccess::Free,
            TaskPrimMode::Eval => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for CompMode {
    fn ctx_access(&self) -> CtxAccess {
        self.default.ctx_access()
            & self.pair.ctx_access()
            & self.task.ctx_access()
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

impl GetCtxAccess for TaskMode {
    fn ctx_access(&self) -> CtxAccess {
        self.func.ctx_access() & self.ctx.ctx_access() & self.input.ctx_access()
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
