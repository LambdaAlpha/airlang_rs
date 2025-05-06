use crate::CallMode;
use crate::CodeMode;
use crate::CompMode;
use crate::List;
use crate::ListMode;
use crate::Map;
use crate::MapMode;
use crate::Mode;
use crate::Pair;
use crate::PairMode;
use crate::SymbolMode;
use crate::UniMode;
use crate::Val;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub call: Option<Mode>,
}

impl FuncMode {
    pub fn default_mode() -> Option<Mode> {
        Some(Mode::Uni(UniMode::new(CodeMode::Eval, SymbolMode::Ref)))
    }

    pub fn default_uni_mode() -> Option<UniMode> {
        Some(UniMode::new(CodeMode::Eval, SymbolMode::Ref))
    }

    pub fn id_func_mode() -> FuncMode {
        FuncMode { call: None }
    }

    pub const fn id_mode() -> Option<Mode> {
        None
    }

    pub const fn uni_mode(code: CodeMode, symbol: SymbolMode) -> Option<Mode> {
        Some(Mode::Uni(UniMode::new(code, symbol)))
    }

    pub fn symbol_mode(symbol: SymbolMode) -> Option<Mode> {
        let mode = CompMode { symbol: Some(symbol), ..CompMode::from(Self::default_uni_mode()) };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn pair_mode(first: Option<Mode>, second: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            pair: Some(PairMode { first, second }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn call_mode(code: CodeMode, func: Option<Mode>, input: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            call: Some(CallMode { code, func, input }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn list_mode(head: List<Option<Mode>>, tail: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            list: Some(ListMode { head, tail }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn map_mode(
        some: Map<Val, Option<Mode>>, key: Option<Mode>, value: Option<Mode>,
    ) -> Option<Mode> {
        let else1 = Pair::new(key, value);
        let mode = CompMode {
            map: Some(MapMode { some, else1 }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }
}

impl Default for FuncMode {
    fn default() -> Self {
        Self { call: Self::default_mode() }
    }
}
