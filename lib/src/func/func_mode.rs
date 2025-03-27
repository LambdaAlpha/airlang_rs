use crate::{
    Call,
    CallMode,
    Change,
    ChangeMode,
    CodeMode,
    CompMode,
    List,
    ListMode,
    Map,
    MapMode,
    Mode,
    Optimize,
    OptimizeMode,
    Pair,
    PairMode,
    Solve,
    SolveMode,
    SymbolMode,
    UniMode,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncMode {
    pub call: Option<Mode>,
    pub optimize: Option<Mode>,
    pub solve: Option<Mode>,
}

impl FuncMode {
    pub fn default_mode() -> Option<Mode> {
        Some(Mode::Uni(UniMode::new(CodeMode::Eval, SymbolMode::Ref)))
    }

    pub fn default_uni_mode() -> Option<UniMode> {
        Some(UniMode::new(CodeMode::Eval, SymbolMode::Ref))
    }

    pub fn id_func_mode() -> FuncMode {
        FuncMode { call: None, optimize: None, solve: None }
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
            pair: Some(PairMode { pair: Pair::new(first, second) }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn call_mode(code: CodeMode, func: Option<Mode>, input: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            call: Some(CallMode { code, call: Call::new(func, input) }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn optimize_mode(code: CodeMode, func: Option<Mode>, input: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            optimize: Some(OptimizeMode { code, optimize: Optimize::new(func, input) }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn solve_mode(code: CodeMode, func: Option<Mode>, input: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            solve: Some(SolveMode { code, solve: Solve::new(func, input) }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn change_mode(from: Option<Mode>, to: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            change: Some(ChangeMode { change: Change::new(from, to) }),
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
        Self {
            call: Self::default_mode(),
            optimize: Self::default_mode(),
            solve: Self::default_mode(),
        }
    }
}
