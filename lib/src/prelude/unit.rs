use crate::{
    Bit,
    ConstFnCtx,
    FuncMode,
    Map,
    Mode,
    Pair,
    Symbol,
    Val,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        ref_pair_mode,
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
    let call = ref_pair_mode();
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
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| Val::Bit(Bit::new(val.is_unit())))
}
