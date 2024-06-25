use crate::{
    ctx::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        CtxMap,
        DefaultCtx,
    },
    prelude::{
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    text::Text,
    val::{
        func::FuncVal,
        Val,
    },
    Bytes,
    Int,
    List,
    Mode,
    Pair,
};

#[derive(Clone)]
pub(crate) struct TextPrelude {
    pub(crate) from_utf8: Named<FuncVal>,
    pub(crate) into_utf8: Named<FuncVal>,
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) concat: Named<FuncVal>,
}

impl Default for TextPrelude {
    fn default() -> Self {
        TextPrelude {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            length: length(),
            push: push(),
            concat: concat(),
        }
    }
}

impl Prelude for TextPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.from_utf8.put(m);
        self.into_utf8.put(m);
        self.length.put(m);
        self.push.put(m);
        self.concat.put(m);
    }
}

fn from_utf8() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("text.from_utf8", input_mode, output_mode, fn_from_utf8)
}

fn fn_from_utf8(input: Val) -> Val {
    let Val::Bytes(bytes) = input else {
        return Val::default();
    };
    let bytes = Bytes::from(bytes);
    if let Ok(str) = String::from_utf8(bytes.into()) {
        Val::Text(Text::from(str).into())
    } else {
        Val::default()
    }
}

fn into_utf8() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("text.into_utf8", input_mode, output_mode, fn_into_utf8)
}

fn fn_into_utf8(input: Val) -> Val {
    let Val::Text(text) = input else {
        return Val::default();
    };
    let text = Text::from(text);
    let bytes = Bytes::from(String::from(text).into_bytes());
    Val::Bytes(bytes.into())
}

fn length() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("text.length", input_mode, output_mode, fn_length)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Text(t) = val else {
            return Val::default();
        };
        let len: Int = t.len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("text.push", input_mode, output_mode, fn_push)
}

fn fn_push(ctx: CtxForMutableFn, input: Val) -> Val {
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

fn concat() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("text.concat", input_mode, output_mode, fn_concat)
}

fn fn_concat(input: Val) -> Val {
    let Val::List(texts) = input else {
        return Val::default();
    };
    let texts = List::from(texts);
    let mut ret = String::new();
    for text in texts {
        let Val::Text(text) = text else {
            return Val::default();
        };
        ret.push_str(&text);
    }
    Val::Text(Text::from(ret).into())
}
