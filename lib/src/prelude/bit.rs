use crate::{
    Bit,
    ConstFnCtx,
    FuncMode,
    Map,
    Pair,
    Symbol,
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
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct BitPrelude {
    pub(crate) is_true: Named<FuncVal>,
    pub(crate) is_false: Named<FuncVal>,
    pub(crate) not: Named<FuncVal>,
    pub(crate) and: Named<FuncVal>,
    pub(crate) or: Named<FuncVal>,
    pub(crate) xor: Named<FuncVal>,
    pub(crate) imply: Named<FuncVal>,
}

impl Default for BitPrelude {
    fn default() -> Self {
        BitPrelude {
            is_true: is_true(),
            is_false: is_false(),
            not: not(),
            and: and(),
            or: or(),
            xor: xor(),
            imply: imply(),
        }
    }
}

impl Prelude for BitPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.is_true.put(m);
        self.is_false.put(m);
        self.not.put(m);
        self.and.put(m);
        self.or.put(m);
        self.xor.put(m);
        self.imply.put(m);
    }
}

fn is_true() -> Named<FuncVal> {
    let id = "is_true";
    let f = fn_is_true;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_true(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Bit(b) = val else {
            return Val::Bit(Bit::false1());
        };
        Val::Bit(*b)
    })
}

fn is_false() -> Named<FuncVal> {
    let id = "is_false";
    let f = fn_is_false;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_is_false(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Bit(b) = val else {
            return Val::Bit(Bit::false1());
        };
        Val::Bit(b.not())
    })
}

fn not() -> Named<FuncVal> {
    let id = "not";
    let f = fn_not;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
