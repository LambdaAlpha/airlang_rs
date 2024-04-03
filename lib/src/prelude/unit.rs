use crate::{
    ctx::NameMap,
    prelude::{
        default_mode,
        named_free_fn,
        Named,
        Prelude,
    },
    transform::Transform,
    unit::Unit,
    val::func::FuncVal,
    Bool,
    Mode,
    Val,
};

#[derive(Clone)]
pub(crate) struct UnitPrelude {
    pub(crate) unit: Named<FuncVal>,
    pub(crate) is_unit: Named<FuncVal>,
}

impl Default for UnitPrelude {
    fn default() -> Self {
        UnitPrelude {
            unit: unit(),
            is_unit: is_unit(),
        }
    }
}

impl Prelude for UnitPrelude {
    fn put(&self, m: &mut NameMap) {
        self.unit.put(m);
        self.is_unit.put(m);
    }
}

fn unit() -> Named<FuncVal> {
    let input_mode = Mode::Generic(Transform::Id);
    let output_mode = Mode::Generic(Transform::Id);
    named_free_fn("unit", input_mode, output_mode, fn_unit)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}

fn is_unit() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("is_unit", input_mode, output_mode, fn_is_unit)
}

fn fn_is_unit(input: Val) -> Val {
    Val::Bool(Bool::new(input.is_unit()))
}
