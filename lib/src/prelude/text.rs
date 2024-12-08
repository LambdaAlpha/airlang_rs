use crate::{
    Byte,
    Int,
    Map,
    Mode,
    Pair,
    Symbol,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
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
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_from_utf8;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_into_utf8;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_length;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Text(t) = val else {
            return Val::default();
        };
        let len: Int = t.len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let id = "text.push";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_push;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_push(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Text(t) = pair.second else {
        return Val::default();
    };
    DefaultCtx.with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::Text(text) = val else {
            return;
        };
        text.push_str(&t);
    })
}

fn join() -> Named<FuncVal> {
    let id = "text.join";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_join;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
