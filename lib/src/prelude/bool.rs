use crate::{
    ctx::NameMap,
    prelude::{
        default_mode,
        named_free_fn,
        pair_mode,
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
    pub(crate) xor: Named<FuncVal>,
    pub(crate) imply: Named<FuncVal>,
}

impl Default for BoolPrelude {
    fn default() -> Self {
        BoolPrelude {
            not: not(),
            and: and(),
            or: or(),
            xor: xor(),
            imply: imply(),
        }
    }
}

impl Prelude for BoolPrelude {
    fn put(&self, m: &mut NameMap) {
        self.not.put(m);
        self.and.put(m);
        self.or.put(m);
        self.xor.put(m);
        self.imply.put(m);
    }
}

fn not() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("not", input_mode, output_mode, fn_not)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

fn and() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("and", input_mode, output_mode, fn_and)
}

fn fn_and(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    let Val::Bool(right) = pair.second else {
        return Val::default();
    };
    Val::Bool(left.and(right))
}

fn or() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("or", input_mode, output_mode, fn_or)
}

fn fn_or(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    let Val::Bool(right) = pair.second else {
        return Val::default();
    };
    Val::Bool(left.or(right))
}

fn xor() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("xor", input_mode, output_mode, fn_xor)
}

fn fn_xor(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    let Val::Bool(right) = pair.second else {
        return Val::default();
    };
    Val::Bool(left.xor(right))
}

fn imply() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("imply", input_mode, output_mode, fn_imply)
}

fn fn_imply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bool(left) = pair.first else {
        return Val::default();
    };
    let Val::Bool(right) = pair.second else {
        return Val::default();
    };
    Val::Bool(left.imply(right))
}
