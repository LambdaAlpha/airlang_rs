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
    string::Str,
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
pub(crate) struct StrPrelude {
    pub(crate) from_utf8: Named<FuncVal>,
    pub(crate) into_utf8: Named<FuncVal>,
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) concat: Named<FuncVal>,
}

impl Default for StrPrelude {
    fn default() -> Self {
        StrPrelude {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            length: length(),
            push: push(),
            concat: concat(),
        }
    }
}

impl Prelude for StrPrelude {
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
    named_free_fn("string.from_utf8", input_mode, output_mode, fn_from_utf8)
}

fn fn_from_utf8(input: Val) -> Val {
    let Val::Bytes(bytes) = input else {
        return Val::default();
    };
    let bytes = Bytes::from(bytes);
    if let Ok(str) = String::from_utf8(bytes.into()) {
        Val::String(Str::from(str).into())
    } else {
        Val::default()
    }
}

fn into_utf8() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("string.into_utf8", input_mode, output_mode, fn_into_utf8)
}

fn fn_into_utf8(input: Val) -> Val {
    let Val::String(str) = input else {
        return Val::default();
    };
    let str = Str::from(str);
    let bytes = Bytes::from(String::from(str).into_bytes());
    Val::Bytes(bytes.into())
}

fn length() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("string.length", input_mode, output_mode, fn_length)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::String(s) = val else {
            return Val::default();
        };
        let len: Int = s.len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("string.push", input_mode, output_mode, fn_push)
}

fn fn_push(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::String(s) = pair.second else {
        return Val::default();
    };
    DefaultCtx.with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::String(str) = val else {
            return;
        };
        str.push_str(&s);
    })
}

fn concat() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("string.concat", input_mode, output_mode, fn_concat)
}

fn fn_concat(input: Val) -> Val {
    let Val::List(strings) = input else {
        return Val::default();
    };
    let strings = List::from(strings);
    let mut ret = String::new();
    for str in strings {
        let Val::String(str) = str else {
            return Val::default();
        };
        ret.push_str(&str);
    }
    Val::String(Str::from(ret).into())
}
