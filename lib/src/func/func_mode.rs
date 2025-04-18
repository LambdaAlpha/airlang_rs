use crate::{
    AbstractMode,
    CallMode,
    ChangeMode,
    CodeMode,
    CompMode,
    EitherMode,
    EquivMode,
    GenerateMode,
    InverseMode,
    List,
    ListMode,
    Map,
    MapMode,
    Mode,
    Pair,
    PairMode,
    Reify,
    ReifyMode,
    SymbolMode,
    UniMode,
    Val,
};

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

    pub fn either_mode(this: Option<Mode>, that: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            either: Some(EitherMode { this, that }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn change_mode(from: Option<Mode>, to: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            change: Some(ChangeMode { from, to }),
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

    pub fn reify_mode(func: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            reify: Some(ReifyMode { reify: Reify::new(func) }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn equiv_mode(func: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            equiv: Some(EquivMode { func }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn inverse_mode(func: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            inverse: Some(InverseMode { func }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn abstract_mode(func: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            abstract1: Some(AbstractMode { func }),
            ..CompMode::from(Self::default_uni_mode())
        };
        Some(Mode::Comp(Box::new(mode)))
    }

    pub fn generate_mode(func: Option<Mode>) -> Option<Mode> {
        let mode = CompMode {
            generate: Some(GenerateMode { func }),
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
