use crate::CallMode;
use crate::CodeMode;
use crate::CompMode;
use crate::DataMode;
use crate::List;
use crate::ListMode;
use crate::Map;
use crate::MapMode;
use crate::Mode;
use crate::Pair;
use crate::PairMode;
use crate::PrimMode;
use crate::SymbolMode;
use crate::Val;

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

    pub fn call_mode(code: CodeMode, func: Option<Mode>, input: Option<Mode>) -> Option<Mode> {
        let mode =
            CompMode { call: Some(CallMode { code, func, input }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn list_mode(head: List<Option<Mode>>, tail: Option<Mode>) -> Option<Mode> {
        let mode = CompMode { list: Some(ListMode { head, tail }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn map_mode(
        some: Map<Val, Option<Mode>>, key: Option<Mode>, value: Option<Mode>,
    ) -> Option<Mode> {
        let else1 = Pair::new(key, value);
        let mode = CompMode { map: Some(MapMode { some, else1 }), ..Self::default_comp_mode() };
        Some(Mode::Comp(Box::new(mode)))
    }
}

impl Default for FuncMode {
    fn default() -> Self {
        Self { forward: Self::default_mode(), reverse: Self::default_mode() }
    }
}
