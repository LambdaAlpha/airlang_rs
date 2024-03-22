use crate::{
    ctx::NameMap,
    io_mode::IoMode,
    prelude::{
        named_free_fn,
        Named,
        Prelude,
    },
    transform::Transform,
    unit::Unit,
    val::func::FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct UnitPrelude {
    pub(crate) unit: Named<FuncVal>,
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
    let input_mode = IoMode::Transform(Transform::Id);
    let output_mode = IoMode::Transform(Transform::Id);
    named_free_fn("unit", input_mode, output_mode, fn_unit)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
