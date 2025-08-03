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

use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
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
use crate::prelude::setup::free_mode;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct ModePrelude {
    pub new: FreeStaticPrimFuncVal,
    pub form_id: FreeStaticPrimFuncVal,
    pub form_literal: MutStaticPrimFuncVal,
    pub form_ref: MutStaticPrimFuncVal,
    pub form_eval: MutStaticPrimFuncVal,
    pub eval_id: MutStaticPrimFuncVal,
    pub eval_literal: MutStaticPrimFuncVal,
    pub eval_ref: MutStaticPrimFuncVal,
    pub eval_eval: MutStaticPrimFuncVal,
}

impl Default for ModePrelude {
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

impl Prelude for ModePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new.put(ctx);
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

pub fn new() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "mode",
        f: free_impl(fn_new),
        mode: free_mode(FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Form)),
    }
    .free_static()
}

fn fn_new(input: Val) -> Val {
    let Some(mode) = parse(input) else {
        return Val::default();
    };
    let func = FuncMode::mode_into_func(mode);
    Val::Func(func)
}

pub fn form_id() -> FreeStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, TaskPrimMode::Form);
    FuncMode::mode_into_free_func(mode)
}

pub fn form_literal() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_ref() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn form_eval() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, TaskPrimMode::Form);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_id() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Id, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_literal() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Literal, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_ref() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Ref, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

pub fn eval_eval() -> MutStaticPrimFuncVal {
    let mode = FuncMode::prim_mode(SymbolMode::Eval, TaskPrimMode::Eval);
    FuncMode::mode_into_mut_func(mode)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Comp(CompMode),
    Func(FuncVal),
}

impl FreeStaticFn<Val, Val> for Mode {
    fn free_static_call(&self, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.free_static_call(input),
            Mode::Func(func) => func.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for Mode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.const_static_call(ctx, input),
            Mode::Func(func) => func.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Val, Val> for Mode {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match self {
            Mode::Comp(comp) => comp.mut_static_call(ctx, input),
            Mode::Func(func) => func.mut_static_call(ctx, input),
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
