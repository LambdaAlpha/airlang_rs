use super::Mode;
use super::SymbolMode;
use crate::semantics::core::ListForm;
use crate::semantics::core::MapForm;
use crate::semantics::core::PairForm;
use crate::semantics::core::TaskEval;
use crate::semantics::core::TaskForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::SetupFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<DataMode>,
    pub task: Option<CodeMode>,
    pub list: Option<DataMode>,
    pub map: Option<DataMode>,
}

impl SetupFn for PrimMode {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DataMode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CodeMode {
    Form,
    Eval,
}

impl FreeStaticFn<Val, Val> for PrimMode {
    fn free_static_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_static_call(symbol),
            Val::Pair(pair) => self.free_static_call(pair),
            Val::Task(task) => self.free_static_call(task),
            Val::List(list) => self.free_static_call(list),
            Val::Map(map) => self.free_static_call(map),
            v => v,
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_static_call(ctx, symbol),
            Val::Pair(pair) => self.const_static_call(ctx, pair),
            Val::Task(task) => self.const_static_call(ctx, task),
            Val::List(list) => self.const_static_call(ctx, list),
            Val::Map(map) => self.const_static_call(ctx, map),
            v => v,
        }
    }
}

impl MutStaticFn<Val, Val, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_static_call(ctx, symbol),
            Val::Pair(pair) => self.mut_static_call(ctx, pair),
            Val::Task(task) => self.mut_static_call(ctx, task),
            Val::List(list) => self.mut_static_call(ctx, list),
            Val::Map(map) => self.mut_static_call(ctx, map),
            v => v,
        }
    }
}

impl FreeStaticFn<Symbol, Val> for PrimMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, Symbol, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Symbol, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.mut_static_call(ctx, input),
        }
    }
}

impl FreeStaticFn<PairVal, Val> for PrimMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        if self.pair.is_none() {
            Val::Pair(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            PairForm { some, first: self, second: self }.free_static_call(input)
        }
    }
}

impl ConstStaticFn<Val, PairVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        if self.pair.is_none() {
            Val::Pair(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            PairForm { some, first: self, second: self }.const_static_call(ctx, input)
        }
    }
}

impl MutStaticFn<Val, PairVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        if self.pair.is_none() {
            Val::Pair(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            PairForm { some, first: self, second: self }.mut_static_call(ctx, input)
        }
    }
}

impl FreeStaticFn<TaskVal, Val> for PrimMode {
    fn free_static_call(&self, input: TaskVal) -> Val {
        match self.task {
            None => Val::Task(input),
            Some(code) => match code {
                CodeMode::Form => {
                    TaskForm { func: self, ctx: self, input: self }.free_static_call(input)
                }
                CodeMode::Eval => TaskEval { func: self }.free_static_call(input),
            },
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        match self.task {
            None => Val::Task(input),
            Some(code) => match code {
                CodeMode::Form => {
                    TaskForm { func: self, ctx: self, input: self }.const_static_call(ctx, input)
                }
                CodeMode::Eval => TaskEval { func: self }.const_static_call(ctx, input),
            },
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        match self.task {
            None => Val::Task(input),
            Some(code) => match code {
                CodeMode::Form => {
                    TaskForm { func: self, ctx: self, input: self }.mut_static_call(ctx, input)
                }
                CodeMode::Eval => TaskEval { func: self }.mut_static_call(ctx, input),
            },
        }
    }
}

impl FreeStaticFn<ListVal, Val> for PrimMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        if self.list.is_none() {
            Val::List(input)
        } else {
            let head = &List::<Option<Mode>>::default();
            ListForm { head, tail: self }.free_static_call(input)
        }
    }
}

impl ConstStaticFn<Val, ListVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        if self.list.is_none() {
            Val::List(input)
        } else {
            let head = &List::<Option<Mode>>::default();
            ListForm { head, tail: self }.const_static_call(ctx, input)
        }
    }
}

impl MutStaticFn<Val, ListVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        if self.list.is_none() {
            Val::List(input)
        } else {
            let head = &List::<Option<Mode>>::default();
            ListForm { head, tail: self }.mut_static_call(ctx, input)
        }
    }
}

impl FreeStaticFn<MapVal, Val> for PrimMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        if self.map.is_none() {
            Val::Map(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            MapForm { some, else_: self }.free_static_call(input)
        }
    }
}

impl ConstStaticFn<Val, MapVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        if self.map.is_none() {
            Val::Map(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            MapForm { some, else_: self }.const_static_call(ctx, input)
        }
    }
}

impl MutStaticFn<Val, MapVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        if self.map.is_none() {
            Val::Map(input)
        } else {
            let some = &Map::<Val, Option<Mode>>::default();
            MapForm { some, else_: self }.mut_static_call(ctx, input)
        }
    }
}

impl PrimMode {
    pub(crate) const fn symbol_task(symbol: SymbolMode, task: CodeMode) -> PrimMode {
        PrimMode {
            symbol: Some(symbol),
            task: Some(task),
            pair: Some(DataMode),
            list: Some(DataMode),
            map: Some(DataMode),
        }
    }
}
