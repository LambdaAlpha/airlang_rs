use super::ListMode;
use super::MapMode;
use super::PairMode;
use super::PrimMode;
use super::TaskMode;
use super::TaskPrimMode;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub default: PrimMode,
    pub pair: Option<Box<PairMode>>,
    pub task: Option<Box<TaskMode>>,
    pub list: Option<Box<ListMode>>,
    pub map: Option<Box<MapMode>>,
}

impl FreeStaticFn<Val, Val> for CompMode {
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

impl ConstStaticFn<Val, Val, Val> for CompMode {
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

impl MutStaticFn<Val, Val, Val> for CompMode {
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

impl FreeStaticFn<Symbol, Val> for CompMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        self.default.symbol.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.default.symbol.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        self.default.symbol.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<PairVal, Val> for CompMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.free_static_call(input);
        };
        pair.form().free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.const_static_call(ctx, input);
        };
        pair.form().const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.mut_static_call(ctx, input);
        };
        pair.form().mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<TaskVal, Val> for CompMode {
    fn free_static_call(&self, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.free_static_call(input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().free_static_call(input),
            TaskPrimMode::Eval => task.eval().free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.const_static_call(ctx, input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().const_static_call(ctx, input),
            TaskPrimMode::Eval => task.eval().const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.mut_static_call(ctx, input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().mut_static_call(ctx, input),
            TaskPrimMode::Eval => task.eval().mut_static_call(ctx, input),
        }
    }
}

impl FreeStaticFn<ListVal, Val> for CompMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.free_static_call(input);
        };
        list.form().free_static_call(input)
    }
}

impl ConstStaticFn<Val, ListVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.const_static_call(ctx, input);
        };
        list.form().const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, ListVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.mut_static_call(ctx, input);
        };
        list.form().mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<MapVal, Val> for CompMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.free_static_call(input);
        };
        map.form().free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.const_static_call(ctx, input);
        };
        map.form().const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.mut_static_call(ctx, input);
        };
        map.form().mut_static_call(ctx, input)
    }
}

impl CompMode {
    pub const fn id() -> Self {
        Self { default: PrimMode::id(), pair: None, task: None, list: None, map: None }
    }

    pub const fn is_id(&self) -> bool {
        self.default.is_id()
            && self.pair.is_none()
            && self.task.is_none()
            && self.list.is_none()
            && self.map.is_none()
    }
}

impl From<PrimMode> for CompMode {
    fn from(default: PrimMode) -> Self {
        Self { default, pair: None, task: None, list: None, map: None }
    }
}
