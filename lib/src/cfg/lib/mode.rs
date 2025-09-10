use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::MutImpl;
use super::free_impl;
use super::memo_put_func;
use super::mode::repr::EVAL_EVAL;
use super::mode::repr::EVAL_ID;
use super::mode::repr::EVAL_LITERAL;
use super::mode::repr::EVAL_REF;
use super::mode::repr::FORM_EVAL;
use super::mode::repr::FORM_ID;
use super::mode::repr::FORM_LITERAL;
use super::mode::repr::FORM_REF;
use super::mode::repr::parse;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::mode::CallPrimMode;
use crate::cfg::mode::FuncMode;
use crate::cfg::mode::MODE_FUNC_ID;
use crate::cfg::mode::SymbolMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::memo::Memo;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct ModeLib {
    pub new: FreePrimFuncVal,
    pub form_id: FreePrimFuncVal,
    pub form_literal: MutPrimFuncVal,
    pub form_ref: MutPrimFuncVal,
    pub form_eval: MutPrimFuncVal,
    pub eval_id: MutPrimFuncVal,
    pub eval_literal: MutPrimFuncVal,
    pub eval_ref: MutPrimFuncVal,
    pub eval_eval: MutPrimFuncVal,
    pub apply: MutPrimFuncVal,
}

impl Default for ModeLib {
    fn default() -> Self {
        Self {
            new: new(),
            form_id: form_id(),
            form_literal: form_literal(),
            form_ref: form_ref(),
            form_eval: form_eval(),
            eval_literal: eval_literal(),
            eval_id: eval_id(),
            eval_ref: eval_ref(),
            eval_eval: eval_eval(),
            apply: apply(),
        }
    }
}

impl CfgMod for ModeLib {
    fn extend(self, cfg: &Cfg) {
        let new_setup = FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Form);
        CoreCfg::extend_setup_mode(cfg, &self.new.id, new_setup);
        self.new.extend(cfg);
        CoreCfg::extend_setup_mode(cfg, MODE_FUNC_ID, FuncMode::id_mode());
        cfg.extend_scope(Symbol::from_str_unchecked(FORM_ID), Val::Func(self.form_id.into()));
        cfg.extend_scope(
            Symbol::from_str_unchecked(FORM_LITERAL),
            Val::Func(self.form_literal.into()),
        );
        cfg.extend_scope(Symbol::from_str_unchecked(FORM_REF), Val::Func(self.form_ref.into()));
        cfg.extend_scope(Symbol::from_str_unchecked(FORM_EVAL), Val::Func(self.form_eval.into()));
        cfg.extend_scope(Symbol::from_str_unchecked(EVAL_ID), Val::Func(self.eval_id.into()));
        cfg.extend_scope(
            Symbol::from_str_unchecked(EVAL_LITERAL),
            Val::Func(self.eval_literal.into()),
        );
        cfg.extend_scope(Symbol::from_str_unchecked(EVAL_REF), Val::Func(self.eval_ref.into()));
        cfg.extend_scope(Symbol::from_str_unchecked(EVAL_EVAL), Val::Func(self.eval_eval.into()));
        let apply_setup = FuncMode::prim_mode(SymbolMode::Ref, CallPrimMode::Form);
        CoreCfg::extend_setup_mode(cfg, &self.apply.id, apply_setup);
        self.apply.extend(cfg);
    }
}

impl Library for ModeLib {
    fn prelude(&self, memo: &mut Memo) {
        self.new.prelude(memo);
        memo_put_func(memo, FORM_ID, &self.form_id);
        memo_put_func(memo, FORM_LITERAL, &self.form_literal);
        memo_put_func(memo, FORM_REF, &self.form_ref);
        memo_put_func(memo, FORM_EVAL, &self.form_eval);
        memo_put_func(memo, EVAL_ID, &self.eval_id);
        memo_put_func(memo, EVAL_LITERAL, &self.eval_literal);
        memo_put_func(memo, EVAL_REF, &self.eval_ref);
        memo_put_func(memo, EVAL_EVAL, &self.eval_eval);
        self.apply.prelude(memo);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "mode", f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = FuncMode::mode_into_func(mode);
    Val::Func(func)
}

pub fn form_id() -> FreePrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, CallPrimMode::Form);
    FuncMode::mode_into_free_func(mode)
}

pub fn form_literal() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_ref() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CallPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_eval() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, CallPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_id() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, CallPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_literal() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_ref() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, CallPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_eval() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, CallPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn { id: "apply", f: MutImpl::new(fn_eval_free, fn_eval_const, fn_eval_mut) }.mut_()
}

fn fn_eval_free(cfg: &mut Cfg, input: Val) -> Val {
    Eval.free_call(cfg, input)
}

fn fn_eval_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    Eval.const_call(cfg, ctx, input)
}

fn fn_eval_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    Eval.mut_call(cfg, ctx, input)
}

mod repr;
