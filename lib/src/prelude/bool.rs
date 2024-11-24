use crate::{
    Bool,
    ConstFnCtx,
    Map,
    Mode,
    Symbol,
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
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct BoolPrelude {
    pub(crate) is_true: Named<FuncVal>,
    pub(crate) is_false: Named<FuncVal>,
    pub(crate) not: Named<FuncVal>,
    pub(crate) and: Named<FuncVal>,
    pub(crate) or: Named<FuncVal>,
    pub(crate) xor: Named<FuncVal>,
    pub(crate) imply: Named<FuncVal>,
}

impl Default for BoolPrelude {
    fn default() -> Self {
        BoolPrelude {
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

impl Prelude for BoolPrelude {
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_is_true;
    named_const_fn(id, call_mode, ask_mode, cacheable, f)
}

fn fn_is_true(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref(ctx, input, |val| {
        let Val::Bool(b) = val else {
            return Val::Bool(Bool::f());
        };
        Val::Bool(*b)
    })
}

fn is_false() -> Named<FuncVal> {
    let id = "is_false";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_is_false;
    named_const_fn(id, call_mode, ask_mode, cacheable, f)
}

fn fn_is_false(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref(ctx, input, |val| {
        let Val::Bool(b) = val else {
            return Val::Bool(Bool::f());
        };
        Val::Bool(b.not())
    })
}

fn not() -> Named<FuncVal> {
    let id = "not";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_not;
    named_free_fn(id, call_mode, ask_mode, cacheable, f)
}

fn fn_not(input: Val) -> Val {
    let Val::Bool(b) = input else {
        return Val::default();
    };
    Val::Bool(b.not())
}

fn and() -> Named<FuncVal> {
    let id = "and";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_and;
    named_free_fn(id, call_mode, ask_mode, cacheable, f)
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
    let id = "or";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_or;
    named_free_fn(id, call_mode, ask_mode, cacheable, f)
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
    let id = "xor";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_xor;
    named_free_fn(id, call_mode, ask_mode, cacheable, f)
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
    let id = "imply";
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_imply;
    named_free_fn(id, call_mode, ask_mode, cacheable, f)
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
