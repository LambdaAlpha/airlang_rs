use log::error;

use self::repr::generate_func;
use self::repr::parse_func;
use self::repr::parse_mode;
use super::DynFn;
use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::func::repr::generate_code;
use super::func::repr::generate_ctx_access;
use super::setup::ctx_default_mode;
use crate::semantics::func::Func;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct FuncPrelude {
    pub new: FreeStaticPrimFuncVal,
    pub repr: FreeStaticPrimFuncVal,
    pub ctx_access: ConstStaticPrimFuncVal,
    pub ctx_explicit: ConstStaticPrimFuncVal,
    pub forward_setup: ConstStaticPrimFuncVal,
    pub reverse_setup: ConstStaticPrimFuncVal,
    pub is_primitive: ConstStaticPrimFuncVal,
    pub is_cell: ConstStaticPrimFuncVal,
    pub id: ConstStaticPrimFuncVal,
    pub code: ConstStaticPrimFuncVal,
    pub ctx: ConstStaticPrimFuncVal,
}

impl Default for FuncPrelude {
    fn default() -> Self {
        FuncPrelude {
            new: new(),
            repr: repr(),
            ctx_access: ctx_access(),
            ctx_explicit: ctx_explicit(),
            forward_setup: forward_setup(),
            reverse_setup: reverse_setup(),
            is_primitive: is_primitive(),
            is_cell: is_cell(),
            id: id(),
            code: code(),
            ctx: ctx(),
        }
    }
}

impl Prelude for FuncPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new.put(ctx);
        self.repr.put(ctx);
        self.ctx_access.put(ctx);
        self.ctx_explicit.put(ctx);
        self.forward_setup.put(ctx);
        self.reverse_setup.put(ctx);
        self.is_primitive.put(ctx);
        self.is_cell.put(ctx);
        self.id.put(ctx);
        self.code.put(ctx);
        self.ctx.put(ctx);
    }
}

pub fn new() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "function",
        f: free_impl(fn_new),
        mode: FuncMode { forward: parse_mode(), reverse: FuncMode::default_mode() },
    }
    .free_static()
}

fn fn_new(input: Val) -> Val {
    let Some(func) = parse_func(input) else {
        error!("parse func failed");
        return Val::default();
    };
    Val::Func(func)
}

pub fn repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "function.represent", f: free_impl(fn_repr), mode: FuncMode::default() }
        .free_static()
}

fn fn_repr(input: Val) -> Val {
    let Val::Func(func) = input else {
        error!("input {input:?} should be a function");
        return Val::default();
    };
    generate_func(func)
}

pub fn ctx_access() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.context_access",
        f: const_impl(fn_ctx_access),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx_access(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let access = generate_ctx_access(func.ctx_access());
    Val::Symbol(Symbol::from_str_unchecked(access))
}

pub fn ctx_explicit() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_context_explicit",
        f: const_impl(fn_ctx_explicit),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx_explicit(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Bit(Bit::new(func.ctx_explicit()))
}

pub fn forward_setup() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.forward_setup",
        f: const_impl(fn_forward_setup),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_forward_setup(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(pre) = func.setup() else {
        return Val::default();
    };
    Val::Func(pre.forward.clone())
}

pub fn reverse_setup() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.reverse_setup",
        f: const_impl(fn_reverse_setup),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_reverse_setup(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(pre) = func.setup() else {
        return Val::default();
    };
    Val::Func(pre.reverse.clone())
}

pub fn is_primitive() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_primitive",
        f: const_impl(fn_is_primitive),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_primitive(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let is_primitive = func.is_primitive();
    Val::Bit(Bit::new(is_primitive))
}

pub fn is_cell() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.is_cell",
        f: const_impl(fn_is_cell),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_is_cell(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    Val::Bit(Bit::new(func.is_cell()))
}

pub fn id() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.id",
        f: const_impl(fn_id),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_id(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(id) = func.id() else {
        return Val::default();
    };
    Val::Symbol(id)
}

pub fn code() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.code",
        f: const_impl(fn_code),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_code(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    generate_code(func)
}

pub fn ctx() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "function.context",
        f: const_impl(fn_ctx),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_ctx(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Func(func) = &*ctx else {
        error!("ctx {ctx:?} should be a function");
        return Val::default();
    };
    let Some(ctx) = func.ctx() else {
        error!("func {func:?} should have an inner ctx");
        return Val::default();
    };
    Val::Ctx(CtxVal::from(ctx.clone()))
}

mod repr;
