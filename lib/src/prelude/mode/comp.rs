use super::ListMode;
use super::MapMode;
use super::PairMode;
use super::PrimMode;
use super::TaskMode;
use super::TaskPrimMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
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

impl FreeFn<Cfg, Val, Val> for CompMode {
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

impl ConstFn<Cfg, Val, Val, Val> for CompMode {
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

impl MutFn<Cfg, Val, Val, Val> for CompMode {
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

impl FreeFn<Cfg, Symbol, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        self.default.symbol.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.default.symbol.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        self.default.symbol.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.free_call(cfg, input);
        };
        pair.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.const_call(cfg, ctx, input);
        };
        pair.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.mut_call(cfg, ctx, input);
        };
        pair.form().mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, TaskVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.free_call(cfg, input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().free_call(cfg, input),
            TaskPrimMode::Eval => task.eval().free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, TaskVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.const_call(cfg, ctx, input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().const_call(cfg, ctx, input),
            TaskPrimMode::Eval => task.eval().const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, TaskVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: TaskVal) -> Val {
        let Some(task) = &self.task else {
            return self.default.mut_call(cfg, ctx, input);
        };
        match self.default.task {
            TaskPrimMode::Form => task.form().mut_call(cfg, ctx, input),
            TaskPrimMode::Eval => task.eval().mut_call(cfg, ctx, input),
        }
    }
}

impl FreeFn<Cfg, ListVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.free_call(cfg, input);
        };
        list.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.const_call(cfg, ctx, input);
        };
        list.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.mut_call(cfg, ctx, input);
        };
        list.form().mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.free_call(cfg, input);
        };
        map.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.const_call(cfg, ctx, input);
        };
        map.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.mut_call(cfg, ctx, input);
        };
        map.form().mut_call(cfg, ctx, input)
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
