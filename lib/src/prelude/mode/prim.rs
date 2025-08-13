use derive_more::Constructor;

use super::Mode;
use super::SymbolMode;
use super::TaskPrimMode;
use crate::semantics::core::ListForm;
use crate::semantics::core::MapForm;
use crate::semantics::core::PairForm;
use crate::semantics::core::TaskEval;
use crate::semantics::core::TaskForm;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Constructor)]
pub struct PrimMode {
    pub symbol: SymbolMode,
    pub task: TaskPrimMode,
}

impl PrimMode {
    pub const fn id() -> Self {
        Self { symbol: SymbolMode::Id, task: TaskPrimMode::Form }
    }

    pub const fn is_id(&self) -> bool {
        matches!(self.symbol, SymbolMode::Id) && matches!(self.task, TaskPrimMode::Form)
    }
}

impl FreeFn<Val, Val> for PrimMode {
    fn free_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(symbol),
            Val::Pair(pair) => self.free_call(pair),
            Val::Task(task) => self.free_call(task),
            Val::List(list) => self.free_call(list),
            Val::Map(map) => self.free_call(map),
            v => v,
        }
    }
}

impl ConstFn<Val, Val, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(ctx, symbol),
            Val::Pair(pair) => self.const_call(ctx, pair),
            Val::Task(task) => self.const_call(ctx, task),
            Val::List(list) => self.const_call(ctx, list),
            Val::Map(map) => self.const_call(ctx, map),
            v => v,
        }
    }
}

impl MutFn<Val, Val, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(ctx, symbol),
            Val::Pair(pair) => self.mut_call(ctx, pair),
            Val::Task(task) => self.mut_call(ctx, task),
            Val::List(list) => self.mut_call(ctx, list),
            Val::Map(map) => self.mut_call(ctx, map),
            v => v,
        }
    }
}

impl FreeFn<Symbol, Val> for PrimMode {
    fn free_call(&self, input: Symbol) -> Val {
        self.symbol.free_call(input)
    }
}

impl ConstFn<Val, Symbol, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.symbol.const_call(ctx, input)
    }
}

impl MutFn<Val, Symbol, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        self.symbol.mut_call(ctx, input)
    }
}

impl FreeFn<PairVal, Val> for PrimMode {
    fn free_call(&self, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.free_call(input)
    }
}

impl ConstFn<Val, PairVal, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, PairVal, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.mut_call(ctx, input)
    }
}

impl FreeFn<TaskVal, Val> for PrimMode {
    fn free_call(&self, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => TaskForm { func: self, ctx: self, input: self }.free_call(input),
            TaskPrimMode::Eval => TaskEval { func: self }.free_call(input),
        }
    }
}

impl ConstFn<Val, TaskVal, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => {
                TaskForm { func: self, ctx: self, input: self }.const_call(ctx, input)
            }
            TaskPrimMode::Eval => TaskEval { func: self }.const_call(ctx, input),
        }
    }
}

impl MutFn<Val, TaskVal, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => {
                TaskForm { func: self, ctx: self, input: self }.mut_call(ctx, input)
            }
            TaskPrimMode::Eval => TaskEval { func: self }.mut_call(ctx, input),
        }
    }
}

impl FreeFn<ListVal, Val> for PrimMode {
    fn free_call(&self, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.free_call(input)
    }
}

impl ConstFn<Val, ListVal, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, ListVal, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.mut_call(ctx, input)
    }
}

impl FreeFn<MapVal, Val> for PrimMode {
    fn free_call(&self, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.free_call(input)
    }
}

impl ConstFn<Val, MapVal, Val> for PrimMode {
    fn const_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, MapVal, Val> for PrimMode {
    fn mut_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.mut_call(ctx, input)
    }
}
