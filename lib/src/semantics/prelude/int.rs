use crate::{
    semantics::{
        ctx::NameMap,
        eval_mode::EvalMode,
        input_mode::InputMode,
        prelude::{
            named_free_fn,
            Named,
            Prelude,
        },
        val::{
            FuncVal,
            Val,
        },
    },
    types::Pair,
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
    fn put(&self, m: &mut NameMap) {
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("+", input_mode, fn_add)
}

fn fn_add(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.add(i2))
}

fn subtract() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("-", input_mode, fn_subtract)
}

fn fn_subtract(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.subtract(i2))
}

fn multiply() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("*", input_mode, fn_multiply)
}

fn fn_multiply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Int(i1.multiply(i2))
}

fn divide() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("/", input_mode, fn_divide)
}

fn fn_divide(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

fn remainder() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("%", input_mode, fn_remainder)
}

fn fn_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i)
}

fn divide_remainder() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("/%", input_mode, fn_divide_remainder)
}

fn fn_divide_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Box::new(Pair::new(Val::Int(quotient), Val::Int(rem))))
}

fn less_than() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("<", input_mode, fn_less_than)
}

fn fn_less_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_than(&i2))
}

fn less_equal() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("<=", input_mode, fn_less_equal)
}

fn fn_less_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_equal(&i2))
}

fn greater_than() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn(">", input_mode, fn_greater_than)
}

fn fn_greater_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.greater_than(&i2))
}

fn greater_equal() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn(">=", input_mode, fn_greater_equal)
}

fn fn_greater_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.greater_equal(&i2))
}

fn less_greater() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    named_free_fn("<>", input_mode, fn_less_greater)
}

fn fn_less_greater(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bool(i1.less_greater(&i2))
}
