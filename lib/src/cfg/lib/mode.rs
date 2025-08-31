pub use self::comp::CompMode;
pub use self::func::FuncMode;
pub use self::list::ListMode;
pub use self::map::MapMode;
pub use self::pair::PairMode;
pub use self::prim::PrimMode;
pub use self::symbol::SymbolMode;
pub use self::task::TaskMode;
pub use self::task::TaskPrimMode;

_____!();

use super::FreePrimFn;
use super::Library;
use super::ctx_put_func;
use super::free_impl;
use super::mode::repr::EVAL_EVAL;
use super::mode::repr::EVAL_ID;
use super::mode::repr::EVAL_LITERAL;
use super::mode::repr::EVAL_REF;
use super::mode::repr::FORM_EVAL;
use super::mode::repr::FORM_ID;
use super::mode::repr::FORM_LITERAL;
use super::mode::repr::FORM_REF;
use super::mode::repr::parse;
use crate::cfg::lib::setup::free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

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
        }
    }
}

impl Library for ModeLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.new.prelude(ctx);
        ctx_put_func(ctx, FORM_ID, &self.form_id);
        ctx_put_func(ctx, FORM_LITERAL, &self.form_literal);
        ctx_put_func(ctx, FORM_REF, &self.form_ref);
        ctx_put_func(ctx, FORM_EVAL, &self.form_eval);
        ctx_put_func(ctx, EVAL_ID, &self.eval_id);
        ctx_put_func(ctx, EVAL_LITERAL, &self.eval_literal);
        ctx_put_func(ctx, EVAL_REF, &self.eval_ref);
        ctx_put_func(ctx, EVAL_EVAL, &self.eval_eval);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn {
        id: "mode",
        f: free_impl(fn_new),
        mode: free_mode(FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Form)),
    }
    .free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = FuncMode::mode_into_func(mode);
    Val::Func(func)
}

pub fn form_id() -> FreePrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, TaskPrimMode::Form);
    FuncMode::mode_into_free_func(mode)
}

pub fn form_literal() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_ref() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_eval() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_id() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_literal() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_ref() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_eval() -> MutPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Comp(CompMode),
    Func(FuncVal),
}

impl FreeFn<Cfg, Val, Val> for Mode {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.free_call(cfg, input),
            Mode::Func(func) => func.free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Mode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.const_call(cfg, ctx, input),
            Mode::Func(func) => func.const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Mode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.mut_call(cfg, ctx, input),
            Mode::Func(func) => func.mut_call(cfg, ctx, input),
        }
    }
}

impl Mode {
    pub const fn id() -> Self {
        Mode::Comp(CompMode::id())
    }

    pub fn is_id(&self) -> bool {
        let Mode::Comp(mode) = self else {
            return false;
        };
        mode.is_id()
    }
}

impl From<PrimMode> for Mode {
    fn from(mode: PrimMode) -> Self {
        Mode::Comp(CompMode::from(mode))
    }
}

mod prim;

mod comp;

mod func;

mod repr;

mod symbol;

mod pair;

mod task;

mod list;

mod map;
