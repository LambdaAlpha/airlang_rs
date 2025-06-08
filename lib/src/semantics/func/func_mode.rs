use crate::semantics::mode::CallMode;
use crate::semantics::mode::CodeMode;
use crate::semantics::mode::CompMode;
use crate::semantics::mode::DataMode;
use crate::semantics::mode::ListMode;
use crate::semantics::mode::MapMode;
use crate::semantics::mode::Mode;
use crate::semantics::mode::PairMode;
use crate::semantics::mode::PrimMode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub forward: Option<Mode>,
    pub reverse: Option<Mode>,
}

pub(crate) const DEFAULT_MODE: PrimMode = PrimMode {
    symbol: Some(SymbolMode::Ref),
    pair: Some(DataMode),
    call: Some(CodeMode::Eval),
    list: Some(DataMode),
    map: Some(DataMode),
};

impl FuncMode {
    pub const fn default_mode() -> Option<Mode> {
        Some(Mode::Prim(DEFAULT_MODE))
    }

    pub const fn default_prim_mode() -> PrimMode {
        DEFAULT_MODE
    }

    pub fn default_comp_mode() -> CompMode {
        CompMode::from(DEFAULT_MODE)
    }

    pub const fn id_func_mode() -> FuncMode {
        FuncMode { forward: None, reverse: None }
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
