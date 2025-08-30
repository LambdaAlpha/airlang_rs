use derive_more::Constructor;

use super::Mode;
use super::SymbolMode;
use super::TaskPrimMode;
use crate::semantics::cfg::Cfg;
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

impl FreeFn<Cfg, Val, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(cfg, symbol),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Task(task) => self.free_call(cfg, task),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => v,
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Task(task) => self.const_call(cfg, ctx, task),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Task(task) => self.mut_call(cfg, ctx, task),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl FreeFn<Cfg, Symbol, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        self.symbol.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.symbol.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        self.symbol.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        if self.is_id() {
            return Val::Pair(input);
        }
        let some = &Map::<Val, Mode>::default();
        PairForm { some, first: self, second: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, TaskVal, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => {
                TaskForm { func: self, ctx: self, input: self }.free_call(cfg, input)
            }
            TaskPrimMode::Eval => TaskEval { func: self }.free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, TaskVal, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => {
                TaskForm { func: self, ctx: self, input: self }.const_call(cfg, ctx, input)
            }
            TaskPrimMode::Eval => TaskEval { func: self }.const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, TaskVal, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: TaskVal) -> Val {
        if self.is_id() {
            return Val::Task(input);
        }
        match self.task {
            TaskPrimMode::Form => {
                TaskForm { func: self, ctx: self, input: self }.mut_call(cfg, ctx, input)
            }
            TaskPrimMode::Eval => TaskEval { func: self }.mut_call(cfg, ctx, input),
        }
    }
}

impl FreeFn<Cfg, ListVal, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        if self.is_id() {
            return Val::List(input);
        }
        let head = &List::<Mode>::default();
        ListForm { head, tail: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for PrimMode {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for PrimMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for PrimMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        if self.is_id() {
            return Val::Map(input);
        }
        let some = &Map::<Val, Mode>::default();
        MapForm { some, else_: self }.mut_call(cfg, ctx, input)
    }
}
