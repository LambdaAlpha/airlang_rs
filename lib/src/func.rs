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
        map::CtxMapRef,
        mut1::MutCtx,
        ref1::CtxMeta,
        Ctx,
        CtxValue,
        Invariant,
    },
    mode::{
        eval::Eval,
        Mode,
    },
    symbol::Symbol,
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
    pub(crate) cacheable: bool,
    pub(crate) transformer: FuncImpl<Primitive<P>, Composite<C>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FuncImpl<P, C> {
    Primitive(P),
    Composite(C),
}

#[derive(Clone)]
pub(crate) struct Primitive<Ext> {
    is_extension: bool,
    id: Symbol,
    ext: Ext,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite<Ext> {
    pub(crate) body: Val,
    pub(crate) prelude: Ctx,
    pub(crate) input_name: Symbol,
    pub(crate) ext: Ext,
}

impl<P, C> Transformer<Val, Val> for Func<P, C>
where
    FuncImpl<Primitive<P>, Composite<C>>: Transformer<Val, Val>,
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
            FuncImpl::Composite(c) => c.transform(ctx, input),
        }
    }
}

fn eval_free(prelude: &mut Ctx, input: Val, input_name: Symbol, body: Val) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    Eval.transform(MutCtx::new(prelude), body)
}

fn eval_aware(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    input: Val,
    input_name: Symbol,
    body: Val,
) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    keep_eval_restore(prelude, ctx, ctx_name, ctx_invariant, body)
}

fn keep_eval_restore(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    body: Val,
) -> Val {
    if !prelude.variables().fallback() && prelude.variables().is_assignable(ctx_name.clone()) {
        let caller = own_ctx(ctx);
        keep_ctx(&mut prelude, caller, ctx_name.clone(), ctx_invariant);
        let output = Eval.transform(MutCtx::new(&mut prelude), body);
        restore_ctx(prelude, ctx, ctx_name);
        output
    } else {
        Eval.transform(MutCtx::new(&mut prelude), body)
    }
}

// here is why we need a `&mut Ctx` for a const func
fn own_ctx(ctx: &mut Ctx) -> Ctx {
    let mut owned = Ctx::default();
    swap(ctx, &mut owned);
    owned
}

fn keep_ctx(prelude: &mut Ctx, ctx: Ctx, name: Symbol, invariant: Invariant) {
    let val = Val::Ctx(CtxVal::from(ctx));
    prelude
        .variables_mut()
        .put_value(name, CtxValue { val, invariant })
        .expect("name should be assignable");
}

fn restore_ctx(prelude: Ctx, ctx: &mut Ctx, name: Symbol) {
    let Ok(Val::Ctx(caller)) = prelude.into_val(name) else {
        unreachable!("restore_ctx ctx invariant is broken!!!");
    };
    let caller = Ctx::from(caller);
    *ctx = caller;
}

impl<P, C> Func<P, C> {
    pub(crate) fn new_primitive(
        input_mode: Mode,
        output_mode: Mode,
        cacheable: bool,
        f: Primitive<P>,
    ) -> Self {
        Self {
            input_mode,
            output_mode,
            cacheable,
            transformer: FuncImpl::Primitive(f),
        }
    }

    pub(crate) fn new_composite(
        input_mode: Mode,
        output_mode: Mode,
        cacheable: bool,
        f: Composite<C>,
    ) -> Self {
        Self {
            input_mode,
            output_mode,
            cacheable,
            transformer: FuncImpl::Composite(f),
        }
    }

    pub fn input_mode(&self) -> &Mode {
        &self.input_mode
    }

    pub fn output_mode(&self) -> &Mode {
        &self.output_mode
    }

    pub fn cacheable(&self) -> bool {
        self.cacheable
    }

    pub(crate) fn transformer(&self) -> &FuncImpl<Primitive<P>, Composite<C>> {
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

    pub(crate) fn composite_body(&self) -> Option<&Val> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(&c.body)
    }

    pub(crate) fn composite_prelude(&self) -> Option<&Ctx> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(&c.prelude)
    }

    pub(crate) fn composite_input_name(&self) -> Option<Symbol> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(c.input_name.clone())
    }
}

impl<P, C> PartialEq for Func<P, C>
where
    Primitive<P>: PartialEq,
    Composite<C>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.input_mode == other.input_mode
            && self.output_mode == other.output_mode
            && self.cacheable == other.cacheable
            && self.transformer == other.transformer
    }
}

impl<P, C> Eq for Func<P, C>
where
    Primitive<P>: Eq,
    Composite<C>: Eq,
{
}

impl<P, C> Hash for Func<P, C>
where
    Primitive<P>: Hash,
    Composite<C>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input_mode.hash(state);
        self.output_mode.hash(state);
        self.cacheable.hash(state);
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

pub(crate) mod static1;

pub(crate) mod const1;

pub(crate) mod mut1;
