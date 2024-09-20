use crate::{
    answer::{
        Answer,
        CACHE,
        MAYBE,
        NEVER,
        NONE,
    },
    ctx::{
        const1::ConstFnCtx,
        default::DefaultCtx,
        CtxValue,
    },
    prelude::{
        named_const_fn,
        named_static_fn,
        Named,
        Prelude,
    },
    utils::val::symbol,
    AnswerVal,
    Bool,
    CaseVal,
    FuncVal,
    Map,
    Mode,
    Symbol,
    Val,
};

#[derive(Clone)]
pub(crate) struct AnswerPrelude {
    pub(crate) none: Named<AnswerVal>,
    pub(crate) never: Named<AnswerVal>,
    pub(crate) maybe: Named<FuncVal>,
    pub(crate) cache: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) type_of: Named<FuncVal>,
    pub(crate) is_none: Named<FuncVal>,
    pub(crate) is_never: Named<FuncVal>,
    pub(crate) is_maybe: Named<FuncVal>,
    pub(crate) is_cache: Named<FuncVal>,
}

impl Default for AnswerPrelude {
    fn default() -> Self {
        AnswerPrelude {
            none: none(),
            never: never(),
            maybe: maybe(),
            cache: cache(),
            repr: repr(),
            type_of: type_of(),
            is_none: is_none(),
            is_never: is_never(),
            is_maybe: is_maybe(),
            is_cache: is_cache(),
        }
    }
}

impl Prelude for AnswerPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.none.put(m);
        self.never.put(m);
        self.maybe.put(m);
        self.cache.put(m);
        self.repr.put(m);
        self.type_of.put(m);
        self.is_none.put(m);
        self.is_never.put(m);
        self.is_maybe.put(m);
        self.is_cache.put(m);
    }
}

fn none() -> Named<AnswerVal> {
    Named::new("answer.none", AnswerVal::from(Answer::None))
}

fn never() -> Named<AnswerVal> {
    Named::new("answer.never", AnswerVal::from(Answer::Never))
}

fn maybe() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_static_fn("answer.maybe", call_mode, ask_mode, true, fn_maybe)
}

fn fn_maybe(input: Val) -> Val {
    Val::Answer(AnswerVal::from(Answer::Maybe(input)))
}

fn cache() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_static_fn("answer.cache", call_mode, ask_mode, true, fn_cached)
}

fn fn_cached(input: Val) -> Val {
    let Val::Case(case) = input else {
        return Val::default();
    };
    let CaseVal::Cache(cache) = case else {
        return Val::default();
    };
    Val::Answer(Answer::Cache(cache).into())
}

fn repr() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_static_fn("answer.represent", call_mode, ask_mode, true, fn_repr)
}

const TYPE: &str = "type";
const VALUE: &str = "value";

fn fn_repr(input: Val) -> Val {
    let Val::Answer(answer) = input else {
        return Val::default();
    };
    let mut repr = Map::<Val, Val>::default();
    let answer = Answer::from(answer);
    match answer {
        Answer::None => {
            repr.insert(symbol(TYPE), symbol(NONE));
        }
        Answer::Never => {
            repr.insert(symbol(TYPE), symbol(NEVER));
        }
        Answer::Maybe(val) => {
            repr.insert(symbol(TYPE), symbol(MAYBE));
            repr.insert(symbol(VALUE), val);
        }
        Answer::Cache(cache) => {
            repr.insert(symbol(TYPE), symbol(CACHE));
            repr.insert(symbol(VALUE), Val::Case(CaseVal::Cache(cache)));
        }
    }
    Val::Map(repr.into())
}

fn type_of() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("answer.type_of", call_mode, ask_mode, true, fn_type_of)
}

fn fn_type_of(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let s = match &**answer {
            Answer::None => NONE,
            Answer::Never => NEVER,
            Answer::Maybe(_) => MAYBE,
            Answer::Cache(_) => CACHE,
        };
        symbol(s)
    })
}

fn is_none() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("answer.is_none", call_mode, ask_mode, true, fn_is_none)
}

fn fn_is_none(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_none = matches!(&**answer, Answer::None);
        Val::Bool(Bool::new(is_none))
    })
}

fn is_never() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("answer.is_never", call_mode, ask_mode, true, fn_is_never)
}

fn fn_is_never(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_never = matches!(&**answer, Answer::Never);
        Val::Bool(Bool::new(is_never))
    })
}

fn is_maybe() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("answer.is_maybe", call_mode, ask_mode, true, fn_is_maybe)
}

fn fn_is_maybe(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_maybe = matches!(&**answer, Answer::Maybe(_));
        Val::Bool(Bool::new(is_maybe))
    })
}

fn is_cache() -> Named<FuncVal> {
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_const_fn("answer.is_cache", call_mode, ask_mode, true, fn_is_cache)
}

fn fn_is_cache(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Answer(answer) = val else {
            return Val::default();
        };
        let is_cache = matches!(&**answer, Answer::Cache(_));
        Val::Bool(Bool::new(is_cache))
    })
}
