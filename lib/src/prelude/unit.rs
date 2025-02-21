use crate::{
    FuncMode,
    Map,
    Symbol,
    Val,
    ctx::map::CtxValue,
    prelude::{
        Named,
        Prelude,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.unit.put(m);
    }
}

fn unit() -> Named<FuncVal> {
    let id = "unit";
    let f = fn_unit;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
