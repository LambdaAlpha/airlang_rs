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
use crate::cfg::CoreCfg;
use crate::cfg::lib::memo::repr::parse_adapter;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct MemoLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
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
        CoreCfg::extend_adapter(cfg, &self.new.id, parse_adapter());
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.reverse.extend(cfg);
        self.remove.extend(cfg);
        self.contract.extend(cfg);
        self.set_contract.extend(cfg);
        self.exist.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "memory.new", f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(memo) = parse_memo(input) else {
        error!("parse_memo failed");
        return Val::default();
    };
    Val::Memo(memo)
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "memory.represent", f: free_impl(fn_repr) }.free()
}

fn fn_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Memo(memo) = input else {
        error!("input {input:?} should be a memo");
        return Val::default();
    };
    generate_memo(memo)
}

pub fn reverse() -> FreePrimFuncVal {
    FreePrimFn { id: "memory.reverse", f: free_impl(fn_reverse) }.free()
}

fn fn_reverse(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Memo(memo) = input else {
        error!("input {input:?} should be a memo");
        return Val::default();
    };
    let ctx = Memo::from(memo);
    let reverse = ctx.reverse();
    Val::Memo(reverse.into())
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "memory.remove", f: mut_impl(fn_remove) }.mut_()
}

fn fn_remove(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Memo(memo) = ctx else {
        error!("ctx {ctx:?} should be a memo");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    memo.remove(s).unwrap_or_default()
}

pub fn contract() -> ConstPrimFuncVal {
    DynPrimFn { id: "memory.contract", f: const_impl(fn_contract) }.const_()
}

fn fn_contract(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    let Some(contract) = memo.get_contract(s.clone()) else {
        error!("variable {s:?} should exist");
        return Val::default();
    };
    generate_contract(contract)
}

pub fn set_contract() -> MutPrimFuncVal {
    DynPrimFn { id: "memory.set_contract", f: mut_impl(fn_set_contract) }.mut_()
}

fn fn_set_contract(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Memo(memo) = ctx else {
        error!("ctx {ctx:?} should be a memo");
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        error!("input.first {:?} should be a symbol", pair.first);
        return Val::default();
    };
    let Some(contract) = parse_contract(&pair.second) else {
        error!("parse contract failed");
        return Val::default();
    };
    let _ = memo.set_contract(s, contract);
    Val::default()
}

pub fn exist() -> ConstPrimFuncVal {
    DynPrimFn { id: "memory.exist", f: const_impl(fn_exist) }.const_()
}

fn fn_exist(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Memo(memo) = &*ctx else {
        error!("ctx {ctx:?} should be a memo");
        return Val::default();
    };
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    Val::Bit(Bit::from(memo.exist(s)))
}

pub(in crate::cfg) mod repr;
