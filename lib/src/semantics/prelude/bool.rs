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
    types::{
        Bool,
        Pair,
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
    let input_mode = InputMode::Any(EvalMode::More);
    named_free_fn("not", input_mode, fn_not)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

fn and() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::More),
        InputMode::Any(EvalMode::More),
    )));
    named_free_fn("and", input_mode, fn_and)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::More),
        InputMode::Any(EvalMode::More),
    )));
    named_free_fn("or", input_mode, fn_or)
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
