use crate::Byte;
use crate::ConstRef;
use crate::Ctx;
use crate::FuncMode;
use crate::Int;
use crate::Pair;
use crate::ctx::main::MainCtx;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;
use crate::prelude::ref_pair_mode;
use crate::text::Text;
use crate::val::Val;
use crate::val::func::FuncVal;

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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.from_utf8.put(ctx);
        self.into_utf8.put(ctx);
        self.length.put(ctx);
        self.push.put(ctx);
        self.join.put(ctx);
    }
}

fn from_utf8() -> Named<FuncVal> {
    let id = "text.from_utf8";
    let f = free_impl(fn_from_utf8);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
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
    let f = free_impl(fn_into_utf8);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
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
    let f = const_impl(fn_length);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_const_fn(id, f, mode)
}

fn fn_length(ctx: ConstRef<Ctx>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(&ctx, pair.first, |val| {
        let Val::Text(t) = val else {
            return Val::default();
        };
        let len: Int = t.len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let id = "text.push";
    let f = mut_impl(fn_push);
    let forward = ref_pair_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    named_mut_fn(id, f, mode)
}

fn fn_push(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Text(t) = pair.second else {
        return Val::default();
    };
    MainCtx::with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::Text(text) = val else {
            return;
        };
        text.push_str(&t);
    })
}

fn join() -> Named<FuncVal> {
    let id = "text.join";
    let f = free_impl(fn_join);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
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
