use crate::{
    FuncMode,
    Int,
    Map,
    Symbol,
    ctx::CtxValue,
    pair::Pair,
    prelude::{
        Named,
        Prelude,
        named_free_fn,
    },
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct IntPrelude {
    pub(crate) add: Named<FuncVal>,
    pub(crate) subtract: Named<FuncVal>,
    pub(crate) multiply: Named<FuncVal>,
    pub(crate) divide: Named<FuncVal>,
    pub(crate) remainder: Named<FuncVal>,
    pub(crate) divide_remainder: Named<FuncVal>,
    pub(crate) less_than: Named<FuncVal>,
    pub(crate) less_equal: Named<FuncVal>,
    pub(crate) greater_than: Named<FuncVal>,
    pub(crate) greater_equal: Named<FuncVal>,
    pub(crate) less_greater: Named<FuncVal>,
}

impl Default for IntPrelude {
    fn default() -> Self {
        IntPrelude {
            add: add(),
            subtract: subtract(),
            multiply: multiply(),
            divide: divide(),
            remainder: remainder(),
            divide_remainder: divide_remainder(),
            less_than: less_than(),
            less_equal: less_equal(),
            greater_than: greater_than(),
            greater_equal: greater_equal(),
            less_greater: less_greater(),
        }
    }
}

impl Prelude for IntPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.add.put(m);
        self.subtract.put(m);
        self.multiply.put(m);
        self.divide.put(m);
        self.remainder.put(m);
        self.divide_remainder.put(m);
        self.less_than.put(m);
        self.less_equal.put(m);
        self.greater_than.put(m);
        self.greater_equal.put(m);
        self.less_greater.put(m);
    }
}

fn add() -> Named<FuncVal> {
    let id = "+";
    let f = fn_add;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_add(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.add(i2).into())
}

fn subtract() -> Named<FuncVal> {
    let id = "-";
    let f = fn_subtract;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_subtract(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.subtract(i2).into())
}

fn multiply() -> Named<FuncVal> {
    let id = "*";
    let f = fn_multiply;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_multiply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.multiply(i2).into())
}

fn divide() -> Named<FuncVal> {
    let id = "/";
    let f = fn_divide;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_divide(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

fn remainder() -> Named<FuncVal> {
    let id = "%";
    let f = fn_remainder;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

fn divide_remainder() -> Named<FuncVal> {
    let id = "/%";
    let f = fn_divide_remainder;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_divide_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

fn less_than() -> Named<FuncVal> {
    let id = "<";
    let f = fn_less_than;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_less_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_than(&i2))
}

fn less_equal() -> Named<FuncVal> {
    let id = "<=";
    let f = fn_less_equal;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_less_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_equal(&i2))
}

fn greater_than() -> Named<FuncVal> {
    let id = ">";
    let f = fn_greater_than;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_greater_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.greater_than(&i2))
}

fn greater_equal() -> Named<FuncVal> {
    let id = ">=";
    let f = fn_greater_equal;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_greater_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.greater_equal(&i2))
}

fn less_greater() -> Named<FuncVal> {
    let id = "<>";
    let f = fn_less_greater;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_less_greater(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_greater(&i2))
}
