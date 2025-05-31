use crate::Byte;
use crate::ConstRef;
use crate::FuncMode;
use crate::FuncVal;
use crate::Int;
use crate::Val;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;

#[derive(Clone)]
pub(crate) struct BytePrelude {
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) join: Named<FuncVal>,
}

impl Default for BytePrelude {
    fn default() -> Self {
        BytePrelude { length: length(), push: push(), join: join() }
    }
}

impl Prelude for BytePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.length.put(ctx);
        self.push.put(ctx);
        self.join.put(ctx);
    }
}

fn length() -> Named<FuncVal> {
    let id = "byte.length";
    let f = const_impl(fn_length);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Byte(byte) = &*ctx else {
        return Val::default();
    };
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

fn push() -> Named<FuncVal> {
    let id = "byte.push";
    let f = mut_impl(fn_push);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_push(ctx: &mut Val, input: Val) -> Val {
    let Val::Byte(byte) = ctx else {
        return Val::default();
    };
    let Val::Byte(b) = input else {
        return Val::default();
    };
    byte.push(&b);
    Val::default()
}

fn join() -> Named<FuncVal> {
    let id = "byte.join";
    let f = free_impl(fn_join);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_join(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let separator: &[u8] = match &pair.first {
        Val::Unit(_) => &[],
        Val::Byte(b) => b,
        _ => return Val::default(),
    };
    let Val::List(bytes) = &pair.second else {
        return Val::default();
    };
    let bytes: Option<Vec<&[u8]>> = bytes
        .iter()
        .map(|v| {
            let Val::Byte(b) = v else {
                return None;
            };
            let byte: &[u8] = b;
            Some(byte)
        })
        .collect();
    let Some(bytes) = bytes else {
        return Val::default();
    };
    let byte = bytes.join(separator);
    Val::Byte(Byte::from(byte).into())
}
