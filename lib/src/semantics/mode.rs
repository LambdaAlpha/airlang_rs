pub use call::CallMode;
pub use comp::CompMode;
pub use list::ListMode;
pub use map::MapMode;
pub use pair::PairMode;
pub use prim::CodeMode;
pub use prim::DataMode;
pub use prim::PrimMode;
pub use symbol::SymbolMode;

_____!();

pub(crate) use symbol::LITERAL;
pub(crate) use symbol::LITERAL_CHAR;
pub(crate) use symbol::MOVE;
pub(crate) use symbol::MOVE_CHAR;
pub(crate) use symbol::REF;
pub(crate) use symbol::REF_CHAR;

_____!();

use super::func::ConstCellFn;
use super::func::ConstStaticFn;
use super::func::FreeCellFn;
use super::func::FreeStaticFn;
use super::func::MutCellFn;
use super::func::MutStaticFn;
use super::val::FuncVal;
use super::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Prim(PrimMode),
    Comp(Box<CompMode>),
    Func(FuncVal),
}

pub(in crate::semantics) trait ModeFn {}

impl ModeFn for Mode {}

impl FreeStaticFn<Val, Val> for Mode {
    fn free_static_call(&self, input: Val) -> Val {
        match self {
            Mode::Prim(prim) => prim.free_static_call(input),
            Mode::Comp(comp) => comp.free_static_call(input),
            Mode::Func(func) => func.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for Mode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            Mode::Prim(prim) => prim.const_static_call(ctx, input),
            Mode::Comp(comp) => comp.const_static_call(ctx, input),
            Mode::Func(func) => func.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Val, Val> for Mode {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match self {
            Mode::Prim(prim) => prim.mut_static_call(ctx, input),
            Mode::Comp(comp) => comp.mut_static_call(ctx, input),
            Mode::Func(func) => func.mut_static_call(ctx, input),
        }
    }
}

impl<I, O, T> FreeStaticFn<I, O> for Option<T>
where
    T: FreeStaticFn<I, O> + ModeFn,
    I: Into<O>,
{
    fn free_static_call(&self, input: I) -> O {
        match self {
            Some(t) => t.free_static_call(input),
            None => input.into(),
        }
    }
}

impl<I, O, T> FreeCellFn<I, O> for Option<T>
where
    T: FreeCellFn<I, O> + ModeFn,
    I: Into<O>,
{
    fn free_cell_call(&mut self, input: I) -> O {
        match self {
            Some(t) => t.free_cell_call(input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for Option<T>
where
    T: ConstStaticFn<Ctx, I, O> + ModeFn,
    Option<T>: FreeStaticFn<I, O>,
    I: Into<O>,
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_static_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> ConstCellFn<Ctx, I, O> for Option<T>
where
    T: ConstCellFn<Ctx, I, O> + ModeFn,
    Option<T>: FreeCellFn<I, O>,
    I: Into<O>,
{
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: I) -> O {
        match self {
            Some(t) => t.const_cell_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> MutStaticFn<Ctx, I, O> for Option<T>
where
    T: MutStaticFn<Ctx, I, O> + ModeFn,
    Option<T>: ConstStaticFn<Ctx, I, O>,
    I: Into<O>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_static_call(ctx, input),
            None => input.into(),
        }
    }
}

impl<Ctx, I, O, T> MutCellFn<Ctx, I, O> for Option<T>
where
    T: MutCellFn<Ctx, I, O> + ModeFn,
    Option<T>: ConstCellFn<Ctx, I, O>,
    I: Into<O>,
{
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: I) -> O {
        match self {
            Some(t) => t.mut_cell_call(ctx, input),
            None => input.into(),
        }
    }
}

impl From<PrimMode> for Mode {
    fn from(mode: PrimMode) -> Self {
        Mode::Prim(mode)
    }
}

mod prim;

mod comp;

mod symbol;

mod pair;

mod call;

mod list;

mod map;
