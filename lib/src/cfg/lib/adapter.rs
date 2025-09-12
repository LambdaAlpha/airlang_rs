use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::adapter::repr::EVAL_EVAL;
use super::adapter::repr::EVAL_ID;
use super::adapter::repr::EVAL_LITERAL;
use super::adapter::repr::EVAL_REF;
use super::adapter::repr::FORM_EVAL;
use super::adapter::repr::FORM_ID;
use super::adapter::repr::FORM_LITERAL;
use super::adapter::repr::FORM_REF;
use super::adapter::repr::parse;
use super::free_impl;
use super::memo_put_func;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::adapter::ADAPTER_FUNC_ID;
use crate::cfg::adapter::CallPrimAdapter;
use crate::cfg::adapter::SymbolAdapter;
use crate::cfg::adapter::adapter_free_func;
use crate::cfg::adapter::adapter_func;
use crate::cfg::adapter::adapter_mut_func;
use crate::cfg::adapter::id_adapter;
use crate::cfg::adapter::prim_adapter;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::memo::Memo;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct AdapterLib {
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

impl Default for AdapterLib {
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

impl CfgMod for AdapterLib {
    fn extend(self, cfg: &Cfg) {
        let new_adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Form);
        CoreCfg::extend_adapter(cfg, &self.new.id, new_adapter);
        self.new.extend(cfg);
        CoreCfg::extend_adapter(cfg, ADAPTER_FUNC_ID, id_adapter());
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
        let apply_adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Form);
        CoreCfg::extend_adapter(cfg, &self.apply.id, apply_adapter);
        self.apply.extend(cfg);
    }
}

impl Library for AdapterLib {
    fn prelude(&self, memo: &mut Memo) {
        memo_put_func(memo, FORM_ID, &self.form_id);
        memo_put_func(memo, FORM_LITERAL, &self.form_literal);
        memo_put_func(memo, FORM_REF, &self.form_ref);
        memo_put_func(memo, FORM_EVAL, &self.form_eval);
        memo_put_func(memo, EVAL_ID, &self.eval_id);
        memo_put_func(memo, EVAL_LITERAL, &self.eval_literal);
        memo_put_func(memo, EVAL_REF, &self.eval_ref);
        memo_put_func(memo, EVAL_EVAL, &self.eval_eval);
        memo_put_func(memo, "apply", &self.apply);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "adapter.new", f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(adapter) = parse(input) else {
        return Val::default();
    };
    let func = adapter_func(adapter);
    Val::Func(func)
}

pub fn form_id() -> FreePrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Id, CallPrimAdapter::Form);
    adapter_free_func(adapter)
}

pub fn form_literal() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Form);
    adapter_mut_func(adapter)
}

pub fn form_ref() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Form);
    adapter_mut_func(adapter)
}

pub fn form_eval() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Eval, CallPrimAdapter::Form);
    adapter_mut_func(adapter)
}

pub fn eval_id() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Id, CallPrimAdapter::Eval);
    adapter_mut_func(adapter)
}

pub fn eval_literal() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Eval);
    adapter_mut_func(adapter)
}

pub fn eval_ref() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Eval);
    adapter_mut_func(adapter)
}

pub fn eval_eval() -> MutPrimFuncVal {
    let adapter = prim_adapter(SymbolAdapter::Eval, CallPrimAdapter::Eval);
    adapter_mut_func(adapter)
}

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn { id: "adapter.apply", f: Eval }.mut_()
}

mod repr;
