use crate::{
    semantics::{
        ctx::NameMap,
        eval_mode::EvalMode,
        input_mode::InputMode,
        prelude::{
            named_free_fn,
            Named,
            Prelude,
        },
        val::FuncVal,
        Val,
    },
    types::Unit,
};

#[derive(Clone)]
pub(crate) struct UnitPrelude {
    unit: Named<FuncVal>,
}

impl Default for UnitPrelude {
    fn default() -> Self {
        UnitPrelude { unit: unit() }
    }
}

impl Prelude for UnitPrelude {
    fn put(&self, m: &mut NameMap) {
        self.unit.put(m);
    }
}

fn unit() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Value);
    named_free_fn("'", input_mode, fn_unit)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
