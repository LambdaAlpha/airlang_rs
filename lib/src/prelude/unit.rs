use crate::{
    FuncMode,
    Val,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_free_fn,
    },
    unit::Unit,
    val::func::FuncVal,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.unit.put(ctx);
    }
}

fn unit() -> Named<FuncVal> {
    let id = "unit";
    let f = fn_unit;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
