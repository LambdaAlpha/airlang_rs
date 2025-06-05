use crate::Byte;
use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Int;
use crate::MutStaticPrimFuncVal;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::text::Text;
use crate::val::Val;

// todo design add more
#[derive(Clone)]
pub(crate) struct TextPrelude {
    pub(crate) from_utf8: FreeStaticPrimFuncVal,
    pub(crate) into_utf8: FreeStaticPrimFuncVal,
    pub(crate) length: ConstStaticPrimFuncVal,
    pub(crate) push: MutStaticPrimFuncVal,
    pub(crate) join: FreeStaticPrimFuncVal,
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

fn from_utf8() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.from_utf8", f: free_impl(fn_from_utf8), mode: FuncMode::default() }
        .free_static()
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

fn into_utf8() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.into_utf8", f: free_impl(fn_into_utf8), mode: FuncMode::default() }
        .free_static()
}

fn fn_into_utf8(input: Val) -> Val {
    let Val::Text(text) = input else {
        return Val::default();
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

fn length() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "text.length",
        f: const_impl(fn_length),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Text(t) = &*ctx else {
        return Val::default();
    };
    let len: Int = t.len().into();
    Val::Int(len.into())
}

fn push() -> MutStaticPrimFuncVal {
    DynFn {
        id: "text.push",
        f: mut_impl(fn_push),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_push(ctx: &mut Val, input: Val) -> Val {
    let Val::Text(text) = ctx else {
        return Val::default();
    };
    let Val::Text(t) = input else {
        return Val::default();
    };
    text.push_str(&t);
    Val::default()
}

// todo design
fn join() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.join", f: free_impl(fn_join), mode: FuncMode::default() }.free_static()
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
