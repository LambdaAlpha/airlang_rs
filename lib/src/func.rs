use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    mem::swap,
};

use crate::{
    ctx::{
        mut1::MutCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        Ctx,
        CtxValue,
        Invariant,
    },
    mode::Mode,
    symbol::Symbol,
    transform::eval::Eval,
    transformer::Transformer,
    val::{
        ctx::CtxVal,
        Val,
    },
};

#[derive(Debug, Clone)]
pub struct Func<P, C> {
    pub(crate) input_mode: Mode,
    pub(crate) output_mode: Mode,
    pub(crate) transformer: FuncImpl<Primitive<P>, Composed<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composed(C),
}

#[derive(Clone)]
pub(crate) struct Primitive<Fn> {
    is_extension: bool,
    id: Symbol,
    fn1: Fn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composed<C> {
    pub(crate) body: Val,
    pub(crate) prelude: Ctx,
    pub(crate) input_name: Symbol,
    pub(crate) ctx: C,
}

impl<P, C> Transformer<Val, Val> for Func<P, C>
where
    FuncImpl<Primitive<P>, Composed<C>>: Transformer<Val, Val>,
{
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.transformer.transform(ctx, input)
    }
}

impl<P: Transformer<Val, Val>, C: Transformer<Val, Val>> Transformer<Val, Val> for FuncImpl<P, C> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncImpl::Primitive(p) => p.transform(ctx, input),
            FuncImpl::Composed(c) => c.transform(ctx, input),
        }
    }
}

fn eval_free(mut new_ctx: Ctx, input: Val, input_name: Symbol, body: Val) -> Val {
    let result = (&mut new_ctx).put_value(input_name, CtxValue::new(input));
    if result.is_err() {
        return Val::default();
    }
    Eval.transform(MutCtx::new(&mut new_ctx), body)
}

fn eval_aware(
    mut new_ctx: Ctx,
    caller: &mut Ctx,
    caller_name: Symbol,
    caller_invariant: Invariant,
    input: Val,
    input_name: Symbol,
    body: Val,
) -> Val {
    let result = (&mut new_ctx).put_value(input_name, CtxValue::new(input));
    if result.is_err() {
        return Val::default();
    }
    keep_eval_restore(new_ctx, caller, caller_name, caller_invariant, body)
}

fn keep_eval_restore(
    mut new_ctx: Ctx,
    ctx: &mut Ctx,
    caller_name: Symbol,
    caller_invariant: Invariant,
    body: Val,
) -> Val {
    if !ctx.is_assignable(caller_name.clone()) {
        return Val::default();
    }
    let caller = own_ctx(ctx);
    keep_ctx(&mut new_ctx, caller, caller_name.clone(), caller_invariant);
    let output = Eval.transform(MutCtx::new(&mut new_ctx), body);
    restore_ctx(ctx, new_ctx, caller_name);
    output
}

// here is why we need a `&mut Ctx` for a const func
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn keep_ctx(new_ctx: &mut Ctx, ctx: Ctx, name: Symbol, invariant: Invariant) {
    let val = Val::Ctx(CtxVal::from(ctx));
    new_ctx
        .put_value(name, CtxValue { val, invariant })
        .expect("name should be assignable");
}

fn restore_ctx(ctx: &mut Ctx, new_ctx: Ctx, name: Symbol) {
    let Ok(Val::Ctx(caller)) = new_ctx.into_val(name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    let caller = Ctx::from(caller);
    *ctx = caller;
}

impl<P, C> Func<P, C> {
    pub(crate) fn new_primitive(input_mode: Mode, output_mode: Mode, f: Primitive<P>) -> Self {
        Self {
            input_mode,
            output_mode,
            transformer: FuncImpl::Primitive(f),
        }
    }

    pub(crate) fn new_composed(input_mode: Mode, output_mode: Mode, f: Composed<C>) -> Self {
        Self {
            input_mode,
            output_mode,
            transformer: FuncImpl::Composed(f),
        }
    }

    pub fn input_mode(&self) -> &Mode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &Mode {
        &self.output_mode
    }

    pub(crate) fn transformer(&self) -> &FuncImpl<Primitive<P>, Composed<C>> {
        &self.transformer
    }

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(&self.transformer, FuncImpl::Primitive(_))
    }

    pub(crate) fn primitive_id(&self) -> Option<Symbol> {
        let FuncImpl::Primitive(p) = &self.transformer else {
            return None;
        };
        Some(p.id.clone())
    }

    pub(crate) fn primitive_is_extension(&self) -> Option<bool> {
        let FuncImpl::Primitive(p) = &self.transformer else {
            return None;
        };
        Some(p.is_extension)
    }

    pub(crate) fn composed_body(&self) -> Option<&Val> {
        let FuncImpl::Composed(c) = &self.transformer else {
            return None;
        };
        Some(&c.body)
    }

    pub(crate) fn composed_prelude(&self) -> Option<&Ctx> {
        let FuncImpl::Composed(c) = &self.transformer else {
            return None;
        };
        Some(&c.prelude)
    }

    pub(crate) fn composed_input_name(&self) -> Option<Symbol> {
        let FuncImpl::Composed(c) = &self.transformer else {
            return None;
        };
        Some(c.input_name.clone())
    }
}

impl<P, C> PartialEq for Func<P, C>
where
    Primitive<P>: PartialEq,
    Composed<C>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.input_mode == other.input_mode
            && self.output_mode == other.output_mode
            && self.transformer == other.transformer
    }
}

impl<P, C> Eq for Func<P, C>
where
    Primitive<P>: Eq,
    Composed<C>: Eq,
{
}

impl<P, C> Hash for Func<P, C>
where
    Primitive<P>: Hash,
    Composed<C>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input_mode.hash(state);
        self.output_mode.hash(state);
        self.transformer.hash(state);
    }
}

impl<F> PartialEq for Primitive<F> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.is_extension == other.is_extension
    }
}

impl<F> Eq for Primitive<F> {}

impl<F> Hash for Primitive<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.is_extension.hash(state);
    }
}

impl<T> Debug for Primitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Primitive").field(&self.id).finish()
    }
}

impl<C> Primitive<C> {
    pub(crate) fn get_id(&self) -> &Symbol {
        &self.id
    }

    pub(crate) fn is_extension(&self) -> bool {
        self.is_extension
    }
}

pub(crate) mod free;

pub(crate) mod const1;

pub(crate) mod mut1;
