use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncVal;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Val;
use crate::mode::comp::CompMode;
use crate::mode::prim::PrimMode;
use crate::mode::united::UniMode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Uni(UniMode),
    Prim(PrimMode),
    Comp(Box<CompMode>),
    Func(FuncVal),
}

pub(crate) const ID: &str = "id";

pub(crate) trait ModeFn {}

impl ModeFn for Mode {}

impl FreeStaticFn<Val, Val> for Mode {
    fn free_static_call(&self, input: Val) -> Val {
        match self {
            Mode::Uni(uni) => uni.free_static_call(input),
            Mode::Prim(prim) => prim.free_static_call(input),
            Mode::Comp(comp) => comp.free_static_call(input),
            Mode::Func(func) => func.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Ctx, Val, Val> for Mode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        match self {
            Mode::Uni(uni) => uni.const_static_call(ctx, input),
            Mode::Prim(prim) => prim.const_static_call(ctx, input),
            Mode::Comp(comp) => comp.const_static_call(ctx, input),
            Mode::Func(func) => func.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Ctx, Val, Val> for Mode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            Mode::Uni(uni) => uni.mut_static_call(ctx, input),
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

impl From<UniMode> for Mode {
    fn from(mode: UniMode) -> Self {
        Mode::Uni(mode)
    }
}

pub(crate) mod united;

pub(crate) mod prim;

pub(crate) mod comp;

pub(crate) mod symbol;

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod repr;
