pub use self::call::CallAdapter;
pub use self::call::CallPrimAdapter;
pub use self::comp::CompAdapter;
pub use self::core::CoreAdapter;
pub use self::list::ListAdapter;
pub use self::map::MapAdapter;
pub use self::pair::PairAdapter;
pub use self::prim::PrimAdapter;
pub use self::symbol::SymbolAdapter;

_____!();

use std::rc::Rc;

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

const DEFAULT_ADAPTER: PrimAdapter =
    PrimAdapter { symbol: SymbolAdapter::Ref, call: CallPrimAdapter::Eval };

pub(crate) const ADAPTER_FUNC_ID: &str = "adapter.object";

pub fn adapter_func(adapter: CoreAdapter) -> FuncVal {
    match adapter.ctx_access() {
        CtxAccess::Free => FuncVal::FreePrim(adapter_free_func(adapter)),
        CtxAccess::Const => FuncVal::ConstPrim(adapter_const_func(adapter)),
        CtxAccess::Mut => FuncVal::MutPrim(adapter_mut_func(adapter)),
    }
}

pub fn adapter_free_func(adapter: CoreAdapter) -> FreePrimFuncVal {
    FreePrimFunc { id: Symbol::from_str_unchecked(ADAPTER_FUNC_ID), fn_: Rc::new(adapter) }.into()
}

pub fn adapter_const_func(adapter: CoreAdapter) -> ConstPrimFuncVal {
    ConstPrimFunc { id: Symbol::from_str_unchecked(ADAPTER_FUNC_ID), fn_: Rc::new(adapter) }.into()
}

pub fn adapter_mut_func(adapter: CoreAdapter) -> MutPrimFuncVal {
    MutPrimFunc { id: Symbol::from_str_unchecked(ADAPTER_FUNC_ID), fn_: Rc::new(adapter) }.into()
}

pub const fn default_adapter() -> CoreAdapter {
    let default = DEFAULT_ADAPTER;
    CoreAdapter::Comp(CompAdapter { default, pair: None, call: None, list: None, map: None })
}

pub const fn default_prim_adapter() -> PrimAdapter {
    DEFAULT_ADAPTER
}

pub fn default_comp_adapter() -> CompAdapter {
    CompAdapter::from(DEFAULT_ADAPTER)
}

pub const fn id_adapter() -> CoreAdapter {
    CoreAdapter::id()
}

pub const fn prim_adapter(symbol: SymbolAdapter, call: CallPrimAdapter) -> CoreAdapter {
    let default = PrimAdapter::new(symbol, call);
    CoreAdapter::Comp(CompAdapter { default, pair: None, call: None, list: None, map: None })
}

pub fn pair_adapter(
    some: Map<Val, CoreAdapter>, first: CoreAdapter, second: CoreAdapter,
) -> CoreAdapter {
    let adapter = CompAdapter {
        pair: Some(Box::new(PairAdapter { some, first, second })),
        ..default_comp_adapter()
    };
    CoreAdapter::Comp(adapter)
}

pub fn call_adapter(func: CoreAdapter, input: CoreAdapter) -> CoreAdapter {
    let adapter =
        CompAdapter { call: Some(Box::new(CallAdapter { func, input })), ..default_comp_adapter() };
    CoreAdapter::Comp(adapter)
}

pub fn list_adapter(head: List<CoreAdapter>, tail: CoreAdapter) -> CoreAdapter {
    let adapter =
        CompAdapter { list: Some(Box::new(ListAdapter { head, tail })), ..default_comp_adapter() };
    CoreAdapter::Comp(adapter)
}

pub fn map_adapter(some: Map<Val, CoreAdapter>, else_: CoreAdapter) -> CoreAdapter {
    let adapter =
        CompAdapter { map: Some(Box::new(MapAdapter { some, else_ })), ..default_comp_adapter() };
    CoreAdapter::Comp(adapter)
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
            Some(t) => t.ctx_access(),
        }
    }
}

impl GetCtxAccess for CoreAdapter {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CoreAdapter::Comp(comp) => comp.ctx_access(),
            CoreAdapter::Func(func) => func.ctx_access(),
        }
    }
}

impl GetCtxAccess for PrimAdapter {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access() & self.call.ctx_access()
    }
}

impl GetCtxAccess for SymbolAdapter {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self, SymbolAdapter::Id) { CtxAccess::Free } else { CtxAccess::Mut }
    }
}

impl GetCtxAccess for CallPrimAdapter {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CallPrimAdapter::Form => CtxAccess::Free,
            CallPrimAdapter::Eval => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for CompAdapter {
    fn ctx_access(&self) -> CtxAccess {
        self.default.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for PairAdapter {
    fn ctx_access(&self) -> CtxAccess {
        let some = self
            .some
            .values()
            .fold(CtxAccess::Free, |access, adapter| access & adapter.ctx_access());
        some & self.first.ctx_access() & self.second.ctx_access()
    }
}

impl GetCtxAccess for CallAdapter {
    fn ctx_access(&self) -> CtxAccess {
        self.func.ctx_access() & self.input.ctx_access()
    }
}

impl GetCtxAccess for ListAdapter {
    fn ctx_access(&self) -> CtxAccess {
        let head =
            self.head.iter().fold(CtxAccess::Free, |access, adapter| access & adapter.ctx_access());
        let tail = self.tail.ctx_access();
        head & tail
    }
}

impl GetCtxAccess for MapAdapter {
    fn ctx_access(&self) -> CtxAccess {
        let some = self
            .some
            .values()
            .fold(CtxAccess::Free, |access, adapter| access & adapter.ctx_access());
        let else_ = self.else_.ctx_access();
        some & else_
    }
}

mod core;

mod prim;

mod comp;

mod symbol;

mod pair;

mod call;

mod list;

mod map;
