use crate::{
    ctx::{
        default::DefaultCtx,
        CtxValue,
    },
    prelude::{
        named_const_fn,
        named_static_fn,
        Named,
        Prelude,
    },
    unit::Unit,
    val::func::FuncVal,
    Bool,
    ConstFnCtx,
    Map,
    Mode,
    Symbol,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.unit.put(m);
        self.is_unit.put(m);
    }
}

fn unit() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn("unit", input_mode, output_mode, true, fn_unit)
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
