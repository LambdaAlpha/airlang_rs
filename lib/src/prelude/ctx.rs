use self::pattern::PatternCtx;
use self::pattern::assign_pattern;
use self::pattern::match_pattern;
use self::pattern::parse_pattern;
use self::repr::generate_contract;
use self::repr::generate_ctx;
use self::repr::parse_contract;
use self::repr::parse_ctx;
use self::repr::parse_mode;
use super::DynFn;
use super::FreeFn;
use super::FreeStaticImpl;
use super::FuncMode;
use super::MutStaticImpl;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::initial_ctx;
use super::mode::CodeMode;
use super::mode::SymbolMode;
use super::mut_impl;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CtxPrelude {
    pub read: ConstStaticPrimFuncVal,
    pub move_: MutStaticPrimFuncVal,
    pub assign: MutStaticPrimFuncVal,
    pub contract: ConstStaticPrimFuncVal,
    pub set_contract: MutStaticPrimFuncVal,
    pub is_locked: ConstStaticPrimFuncVal,
    pub is_null: ConstStaticPrimFuncVal,
    pub is_const: MutStaticPrimFuncVal,
    pub with_ctx: MutStaticPrimFuncVal,
    pub ctx_new: FreeStaticPrimFuncVal,
    pub ctx_repr: FreeStaticPrimFuncVal,
    pub ctx_prelude: FreeStaticPrimFuncVal,
    pub ctx_self: ConstStaticPrimFuncVal,
}

impl Default for CtxPrelude {
    fn default() -> Self {
        CtxPrelude {
            read: read(),
            move_: move_(),
            assign: assign(),
            contract: contract(),
            set_contract: set_contract(),
            is_locked: is_locked(),
            is_null: is_null(),
            is_const: is_const(),
            with_ctx: with_ctx(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_prelude: ctx_prelude(),
            ctx_self: ctx_self(),
        }
    }
}

impl Prelude for CtxPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.read.put(ctx);
        self.move_.put(ctx);
        self.assign.put(ctx);
        self.contract.put(ctx);
        self.set_contract.put(ctx);
        self.is_locked.put(ctx);
        self.is_null.put(ctx);
        self.is_const.put(ctx);
        self.with_ctx.put(ctx);
        self.ctx_new.put(ctx);
        self.ctx_repr.put(ctx);
        self.ctx_prelude.put(ctx);
        self.ctx_self.put(ctx);
    }
}

pub fn read() -> ConstStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "read",
        f: const_impl(fn_read),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .const_static()
}

fn fn_read(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    ctx.variables().get_ref(s).cloned().unwrap_or_default()
}

pub fn move_() -> MutStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "move",
        f: mut_impl(fn_move),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_move(ctx: &mut Val, input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    ctx.variables_mut().remove(s).unwrap_or_default()
}

pub fn assign() -> MutStaticPrimFuncVal {
    DynFn {
        id: "=",
        f: mut_impl(fn_assign),
        mode: FuncMode {
            forward: FuncMode::pair_mode(
                FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form),
                FuncMode::default_mode(),
            ),
            reverse: FuncMode::default_mode(),
        },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_assign(ctx: &mut Val, input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let pattern_ctx = PatternCtx::default();
    let Some(pattern) = parse_pattern(pattern_ctx, pair.first) else {
        return Val::default();
    };
    let val = pair.second;
    if match_pattern(&pattern, &val) { assign_pattern(ctx, pattern, val) } else { Val::default() }
}

pub fn contract() -> ConstStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "contract",
        f: const_impl(fn_contract),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .const_static()
}

fn fn_contract(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Some(contract) = ctx.variables().get_contract(s) else {
        return Val::default();
    };
    generate_contract(contract)
}

pub fn set_contract() -> MutStaticPrimFuncVal {
    DynFn {
        id: "set_contract",
        f: mut_impl(fn_set_contract),
        mode: FuncMode {
            forward: FuncMode::pair_mode(
                FuncMode::symbol_mode(SymbolMode::Literal),
                FuncMode::symbol_mode(SymbolMode::Literal),
            ),
            reverse: FuncMode::default_mode(),
        },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_set_contract(ctx: &mut Val, input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Some(contract) = parse_contract(pair.second) else {
        return Val::default();
    };
    let _ = ctx.variables_mut().set_contract(s, contract);
    Val::default()
}

pub fn is_locked() -> ConstStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "is_locked",
        f: const_impl(fn_is_locked),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .const_static()
}

fn fn_is_locked(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Some(locked) = ctx.variables().is_locked(s) else {
        return Val::default();
    };
    Val::Bit(Bit::new(locked))
}

pub fn is_null() -> ConstStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "is_null",
        f: const_impl(fn_is_null),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .const_static()
}

fn fn_is_null(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    Val::Bit(Bit::new(ctx.variables().is_null(s)))
}

pub fn is_const() -> MutStaticPrimFuncVal {
    DynFn {
        id: "is_constant",
        f: MutStaticImpl::new(FreeStaticImpl::default, fn_const, fn_mut),
        mode: FuncMode::default(),
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_const(_ctx: ConstRef<Val>, _input: Val) -> Val {
    Val::Bit(Bit::true_())
}

fn fn_mut(_ctx: &mut Val, _input: Val) -> Val {
    Val::Bit(Bit::false_())
}

pub fn with_ctx() -> MutStaticPrimFuncVal {
    DynFn {
        id: "|",
        f: MutStaticImpl::new(fn_with_ctx_free, fn_with_ctx_const, fn_with_ctx_mut),
        mode: FuncMode {
            forward: FuncMode::pair_mode(
                FuncMode::symbol_mode(SymbolMode::Literal),
                FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form),
            ),
            reverse: FuncMode::default_mode(),
        },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_with_ctx_free(input: Val) -> Val {
    Eval.free_static_call(input)
}

fn fn_with_ctx_const(ctx: ConstRef<Val>, input: Val) -> Val {
    Eval.const_static_call(ctx, input)
}

fn fn_with_ctx_mut(ctx: &mut Val, input: Val) -> Val {
    Eval.mut_static_call(ctx, input)
}

pub fn ctx_new() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "context",
        f: free_impl(fn_ctx_new),
        mode: FuncMode { forward: parse_mode(), reverse: FuncMode::default_mode() },
    }
    .free_static()
}

fn fn_ctx_new(input: Val) -> Val {
    match parse_ctx(input) {
        Some(ctx) => Val::Ctx(ctx),
        None => Val::default(),
    }
}

pub fn ctx_repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "context.represent", f: free_impl(fn_ctx_repr), mode: FuncMode::default() }
        .free_static()
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        return Val::default();
    };
    generate_ctx(ctx)
}

pub fn ctx_prelude() -> FreeStaticPrimFuncVal {
    FreeFn { id: "prelude", f: free_impl(fn_ctx_prelude), mode: FuncMode::default() }.free_static()
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

pub fn ctx_self() -> ConstStaticPrimFuncVal {
    DynFn { id: "self", f: const_impl(fn_ctx_self), mode: FuncMode::default(), ctx_explicit: false }
        .const_static()
}

fn fn_ctx_self(ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}

pub(super) mod pattern;

pub(super) mod repr;

pub(super) mod ref_;
