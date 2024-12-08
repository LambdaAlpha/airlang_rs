use crate::{
    Bit,
    ConstFnCtx,
    Map,
    Mode,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        default::DefaultCtx,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
    },
    unit::Unit,
    val::func::FuncVal,
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
    let id = "unit";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_unit;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}

fn is_unit() -> Named<FuncVal> {
    let id = "is_unit";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_is_unit;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_is_unit(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref(ctx, input, |val| Val::Bit(Bit::new(val.is_unit())))
}
