use crate::{
    Byte,
    ConstFnCtx,
    FuncMode,
    Int,
    Map,
    Pair,
    Symbol,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        mut1::MutFnCtx,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    text::Text,
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct TextPrelude {
    pub(crate) from_utf8: Named<FuncVal>,
    pub(crate) into_utf8: Named<FuncVal>,
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) join: Named<FuncVal>,
}

impl Default for TextPrelude {
    fn default() -> Self {
        TextPrelude {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            length: length(),
            push: push(),
            join: join(),
        }
    }
}

impl Prelude for TextPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.from_utf8.put(m);
        self.into_utf8.put(m);
        self.length.put(m);
        self.push.put(m);
        self.join.put(m);
    }
}

fn from_utf8() -> Named<FuncVal> {
    let id = "text.from_utf8";
    let f = fn_from_utf8;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_from_utf8(input: Val) -> Val {
    let Val::Byte(byte) = input else {
        return Val::default();
    };
    let byte = Byte::from(byte);
    if let Ok(str) = String::from_utf8(byte.into()) {
        Val::Text(Text::from(str).into())
    } else {
        Val::default()
    }
}

fn into_utf8() -> Named<FuncVal> {
    let id = "text.into_utf8";
    let f = fn_into_utf8;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_into_utf8(input: Val) -> Val {
    let Val::Text(text) = input else {
        return Val::default();
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

fn length() -> Named<FuncVal> {
    let id = "text.length";
    let f = fn_length;
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

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Text(t) = val else {
            return Val::default();
        };
        let len: Int = t.len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let id = "text.push";
    let f = fn_push;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_push(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Text(t) = pair.second else {
        return Val::default();
    };
    DefaultCtx::with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::Text(text) = val else {
            return;
        };
        text.push_str(&t);
    })
}

fn join() -> Named<FuncVal> {
    let id = "text.join";
    let f = fn_join;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_join(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let separator = match &pair.first {
        Val::Unit(_) => "",
        Val::Text(t) => t,
        _ => return Val::default(),
    };
    let Val::List(texts) = &pair.second else {
        return Val::default();
    };
    let texts: Option<Vec<&str>> = texts
        .iter()
        .map(|v| {
            let Val::Text(t) = v else {
                return None;
            };
            let s: &str = t;
            Some(s)
        })
        .collect();
    let Some(texts) = texts else {
        return Val::default();
    };
    let text = texts.join(separator);
    Val::Text(Text::from(text).into())
}
