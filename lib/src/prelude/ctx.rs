use crate::CodeMode;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticFn;
use crate::FreeStaticImpl;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::MutStaticFn;
use crate::MutStaticImpl;
use crate::MutStaticPrimFuncVal;
use crate::SymbolMode;
use crate::bit::Bit;
use crate::ctx::main::MainCtx;
use crate::ctx::pattern::PatternCtx;
use crate::ctx::pattern::assign_pattern;
use crate::ctx::pattern::match_pattern;
use crate::ctx::pattern::parse_pattern;
use crate::ctx::repr::generate_contract;
use crate::ctx::repr::generate_ctx;
use crate::ctx::repr::parse_contract;
use crate::ctx::repr::parse_ctx;
use crate::ctx::repr::parse_mode;
use crate::func::func_mode::DEFAULT_MODE;
use crate::pair::Pair;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::free_impl;
use crate::prelude::initial_ctx;
use crate::prelude::mut_impl;
use crate::val::Val;
use crate::val::ctx::CtxVal;

#[derive(Clone)]
pub(crate) struct CtxPrelude {
    pub(crate) read: ConstStaticPrimFuncVal,
    pub(crate) move1: MutStaticPrimFuncVal,
    pub(crate) assign: MutStaticPrimFuncVal,
    pub(crate) contract: ConstStaticPrimFuncVal,
    pub(crate) set_contract: MutStaticPrimFuncVal,
    pub(crate) is_locked: ConstStaticPrimFuncVal,
    pub(crate) is_null: ConstStaticPrimFuncVal,
    pub(crate) is_const: MutStaticPrimFuncVal,
    pub(crate) with_ctx: MutStaticPrimFuncVal,
    pub(crate) ctx_in_ctx_out: FreeStaticPrimFuncVal,
    pub(crate) ctx_new: FreeStaticPrimFuncVal,
    pub(crate) ctx_repr: FreeStaticPrimFuncVal,
    pub(crate) ctx_prelude: FreeStaticPrimFuncVal,
    pub(crate) ctx_self: ConstStaticPrimFuncVal,
}

impl Default for CtxPrelude {
    fn default() -> Self {
        CtxPrelude {
            read: read(),
            move1: move1(),
            assign: assign(),
            contract: contract(),
            set_contract: set_contract(),
            is_locked: is_locked(),
            is_null: is_null(),
            is_const: is_const(),
            with_ctx: with_ctx(),
            ctx_in_ctx_out: ctx_in_ctx_out(),
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
        self.move1.put(ctx);
        self.assign.put(ctx);
        self.contract.put(ctx);
        self.set_contract.put(ctx);
        self.is_locked.put(ctx);
        self.is_null.put(ctx);
        self.is_const.put(ctx);
        self.with_ctx.put(ctx);
        self.ctx_in_ctx_out.put(ctx);
        self.ctx_new.put(ctx);
        self.ctx_repr.put(ctx);
        self.ctx_prelude.put(ctx);
        self.ctx_self.put(ctx);
    }
}

fn read() -> ConstStaticPrimFuncVal {
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
    MainCtx::get_or_default(ctx, s)
}

fn move1() -> MutStaticPrimFuncVal {
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

fn assign() -> MutStaticPrimFuncVal {
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

fn contract() -> ConstStaticPrimFuncVal {
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

fn set_contract() -> MutStaticPrimFuncVal {
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

fn is_locked() -> ConstStaticPrimFuncVal {
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

fn is_null() -> ConstStaticPrimFuncVal {
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

fn is_const() -> MutStaticPrimFuncVal {
    DynFn {
        id: "is_constant",
        f: MutStaticImpl::new(FreeStaticImpl::default, fn_const, fn_mut),
        mode: FuncMode::default(),
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_const(_ctx: ConstRef<Val>, _input: Val) -> Val {
    Val::Bit(Bit::true1())
}

fn fn_mut(_ctx: &mut Val, _input: Val) -> Val {
    Val::Bit(Bit::false1())
}

fn with_ctx() -> MutStaticPrimFuncVal {
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
    DEFAULT_MODE.free_static_call(input)
}

fn fn_with_ctx_const(ctx: ConstRef<Val>, input: Val) -> Val {
    DEFAULT_MODE.const_static_call(ctx, input)
}

fn fn_with_ctx_mut(ctx: &mut Val, input: Val) -> Val {
    DEFAULT_MODE.mut_static_call(ctx, input)
}

fn ctx_in_ctx_out() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "|:",
        f: free_impl(fn_ctx_in_ctx_out),
        mode: FuncMode {
            forward: FuncMode::pair_mode(
                FuncMode::default_mode(),
                FuncMode::prim_mode(SymbolMode::Ref, CodeMode::Form),
            ),
            reverse: FuncMode::default_mode(),
        },
    }
    .free_static()
}

fn fn_ctx_in_ctx_out(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_input = Pair::from(pair);
    let mut ctx = ctx_input.first;
    let input = ctx_input.second;
    let output = DEFAULT_MODE.mut_static_call(&mut ctx, input);
    let pair = Pair::new(ctx, output);
    Val::Pair(pair.into())
}

fn ctx_new() -> FreeStaticPrimFuncVal {
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

fn ctx_repr() -> FreeStaticPrimFuncVal {
    FreeFn { id: "context.represent", f: free_impl(fn_ctx_repr), mode: FuncMode::default() }
        .free_static()
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        return Val::default();
    };
    generate_ctx(ctx)
}

fn ctx_prelude() -> FreeStaticPrimFuncVal {
    FreeFn { id: "prelude", f: free_impl(fn_ctx_prelude), mode: FuncMode::default() }.free_static()
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_self() -> ConstStaticPrimFuncVal {
    DynFn { id: "self", f: const_impl(fn_ctx_self), mode: FuncMode::default(), ctx_explicit: false }
        .const_static()
}

fn fn_ctx_self(ctx: ConstRef<Val>, _input: Val) -> Val {
    ctx.unwrap().clone()
}
