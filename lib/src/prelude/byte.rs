use crate::Byte;
use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Int;
use crate::MutStaticPrimFuncVal;
use crate::Val;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;

// todo design add more
#[derive(Clone)]
pub(crate) struct BytePrelude {
    pub(crate) length: ConstStaticPrimFuncVal,
    pub(crate) push: MutStaticPrimFuncVal,
    pub(crate) join: FreeStaticPrimFuncVal,
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

fn length() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "byte.length",
        f: const_impl(fn_length),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Byte(byte) = &*ctx else {
        return Val::default();
    };
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

fn push() -> MutStaticPrimFuncVal {
    DynFn {
        id: "byte.push",
        f: mut_impl(fn_push),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

// todo design
fn join() -> FreeStaticPrimFuncVal {
    FreeFn { id: "byte.join", f: free_impl(fn_join), mode: FuncMode::default() }.free_static()
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
