use crate::{
    FuncMode,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_free_fn,
    },
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct BitPrelude {
    pub(crate) not: Named<FuncVal>,
    pub(crate) and: Named<FuncVal>,
    pub(crate) or: Named<FuncVal>,
    pub(crate) xor: Named<FuncVal>,
    pub(crate) imply: Named<FuncVal>,
}

impl Default for BitPrelude {
    fn default() -> Self {
        BitPrelude { not: not(), and: and(), or: or(), xor: xor(), imply: imply() }
    }
}

impl Prelude for BitPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.not.put(ctx);
        self.and.put(ctx);
        self.or.put(ctx);
        self.xor.put(ctx);
        self.imply.put(ctx);
    }
}

fn not() -> Named<FuncVal> {
    let id = "not";
    let f = fn_not;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_not(input: Val) -> Val {
    let Val::Bit(b) = input else {
        return Val::default();
    };
    Val::Bit(b.not())
}

fn and() -> Named<FuncVal> {
    let id = "and";
    let f = fn_and;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_and(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.and(right))
}

fn or() -> Named<FuncVal> {
    let id = "or";
    let f = fn_or;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_or(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.or(right))
}

fn xor() -> Named<FuncVal> {
    let id = "xor";
    let f = fn_xor;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_xor(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.xor(right))
}

fn imply() -> Named<FuncVal> {
    let id = "imply";
    let f = fn_imply;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_imply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.imply(right))
}
