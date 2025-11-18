use log::error;

use self::repr::generate_contract;
use self::repr::generate_memo;
use self::repr::parse_contract;
use self::repr::parse_memo;
use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct MemoLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub length: ConstPrimFuncVal,
    pub reverse: FreePrimFuncVal,
    pub remove: MutPrimFuncVal,
    pub contract: ConstPrimFuncVal,
    pub set_contract: MutPrimFuncVal,
    pub exist: ConstPrimFuncVal,
}

impl Default for MemoLib {
    fn default() -> Self {
        MemoLib {
            new: new(),
            repr: repr(),
            length: length(),
            reverse: reverse(),
            remove: remove(),
            contract: contract(),
            set_contract: set_contract(),
            exist: exist(),
        }
    }
}

impl CfgMod for MemoLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.length.extend(cfg);
        self.reverse.extend(cfg);
        self.remove.extend(cfg);
        self.contract.extend(cfg);
        self.set_contract.extend(cfg);
        self.exist.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "_memory.new", raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Some(memo) = parse_memo(input) else {
        error!("parse_memo failed");
        return illegal_input(cfg);
    };
    Val::Memo(memo)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "_memory.represent", raw_input: false, f: free_impl(fn_repr) }.free()
}

fn fn_repr(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Memo(memo) = input else {
        error!("input {input:?} should be a memo");
        return illegal_input(cfg);
    };
    generate_memo(memo)
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "_memory.length", raw_input: false, f: const_impl(fn_length) }.const_()
}

fn fn_length(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return illegal_ctx(cfg);
    };
    Val::Int(Int::from(memo.len()).into())
}

pub fn reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "_memory.reverse", raw_input: false, f: free_impl(fn_reverse) }.free()
}

fn fn_reverse(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Memo(memo) = input else {
        error!("input {input:?} should be a memo");
        return illegal_input(cfg);
    };
    let ctx = Memo::from(memo);
    let reverse = ctx.reverse();
    Val::Memo(reverse.into())
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "_memory.remove", raw_input: false, f: mut_impl(fn_remove) }.mut_()
}

fn fn_remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Memo(memo) = ctx else {
        error!("ctx {ctx:?} should be a memo");
        return illegal_ctx(cfg);
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return illegal_input(cfg);
    };
    memo.remove(s).unwrap_or_default()
}

pub fn contract() -> ConstPrimFuncVal {
    DynPrimFn { id: "_memory.contract", raw_input: false, f: const_impl(fn_contract) }.const_()
}

fn fn_contract(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return illegal_ctx(cfg);
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return illegal_input(cfg);
    };
    let Some(contract) = memo.get_contract(s.clone()) else {
        error!("variable {s:?} should exist");
        return fail(cfg);
    };
    generate_contract(contract)
}

pub fn set_contract() -> MutPrimFuncVal {
    DynPrimFn { id: "_memory.set_contract", raw_input: false, f: mut_impl(fn_set_contract) }.mut_()
}

fn fn_set_contract(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Memo(memo) = ctx else {
        error!("ctx {ctx:?} should be a memo");
        return illegal_ctx(cfg);
    };
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        error!("input.first {:?} should be a symbol", pair.first);
        return illegal_input(cfg);
    };
    let Some(contract) = parse_contract(&pair.second) else {
        error!("parse contract failed");
        return illegal_input(cfg);
    };
    if memo.set_contract(s, contract).is_err() {
        return fail(cfg);
    }
    Val::default()
}

pub fn exist() -> ConstPrimFuncVal {
    DynPrimFn { id: "_memory.exist", raw_input: false, f: const_impl(fn_exist) }.const_()
}

fn fn_exist(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return illegal_ctx(cfg);
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return illegal_input(cfg);
    };
    Val::Bit(Bit::from(memo.exist(s)))
}

pub(in crate::cfg) mod repr;
