use crate::{
    bool::Bool,
    ctx::NameMap,
    eval_mode::EvalMode,
    io_mode::IoMode,
    pair::Pair,
    prelude::{
        named_free_fn,
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct BoolPrelude {
    pub(crate) not: Named<FuncVal>,
    pub(crate) and: Named<FuncVal>,
    pub(crate) or: Named<FuncVal>,
}

impl Default for BoolPrelude {
    fn default() -> Self {
        BoolPrelude {
            not: not(),
            and: and(),
            or: or(),
        }
    }
}

impl Prelude for BoolPrelude {
    fn put(&self, m: &mut NameMap) {
        self.not.put(m);
        self.and.put(m);
        self.or.put(m);
    }
}

fn not() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("not", input_mode, output_mode, fn_not)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

fn and() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("and", input_mode, output_mode, fn_and)
}

fn fn_and(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    if left.bool() {
        let Val::Bool(right) = pair.second else {
            return Val::default();
        };
        Val::Bool(right)
    } else {
        Val::Bool(Bool::f())
    }
}

fn or() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("or", input_mode, output_mode, fn_or)
}

fn fn_or(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    if left.bool() {
        Val::Bool(Bool::t())
    } else {
        let Val::Bool(right) = pair.second else {
            return Val::default();
        };
        Val::Bool(right)
    }
}
