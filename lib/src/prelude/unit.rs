use crate::{
    Bit,
    ConstFnCtx,
    FuncMode,
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
        id_mode,
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
    let f = fn_unit;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}

fn is_unit() -> Named<FuncVal> {
    let id = "is_unit";
    let f = fn_is_unit;
    let call = id_mode();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_unit(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx::with_ref(ctx, input, |val| Val::Bit(Bit::new(val.is_unit())))
}
