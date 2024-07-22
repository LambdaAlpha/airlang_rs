use crate::{
    ctx::{
        CtxMap,
        DefaultCtx,
    },
    prelude::{
        named_const_fn,
        named_free_fn,
        Named,
        Prelude,
    },
    unit::Unit,
    val::func::FuncVal,
    Bool,
    ConstFnCtx,
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
    fn put(&self, m: &mut CtxMap) {
        self.unit.put(m);
        self.is_unit.put(m);
    }
}

fn unit() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("unit", input_mode, output_mode, true, fn_unit)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}

fn is_unit() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("is_unit", input_mode, output_mode, true, fn_is_unit)
}

fn fn_is_unit(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref(ctx, input, |val| Val::Bool(Bool::new(val.is_unit())))
}
