use super::DynFn;
use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::ctx_default_mode;
use super::free_impl;
use super::mut_impl;
use crate::semantics::func::FuncMode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Int;

// todo design add more
#[derive(Clone)]
pub struct BytePrelude {
    pub length: ConstStaticPrimFuncVal,
    pub push: MutStaticPrimFuncVal,
    pub join: FreeStaticPrimFuncVal,
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

pub fn length() -> ConstStaticPrimFuncVal {
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

pub fn push() -> MutStaticPrimFuncVal {
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
pub fn join() -> FreeStaticPrimFuncVal {
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
