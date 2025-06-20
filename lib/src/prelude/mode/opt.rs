use crate::semantics::func::ConstCellFn;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutCellFn;
use crate::semantics::func::MutStaticFn;
use crate::type_::ConstRef;

pub(crate) trait ModeFn {}

impl<T> ModeFn for &T where T: ModeFn {}

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
