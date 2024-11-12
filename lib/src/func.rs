use std::{
    fmt::{
        Debug,
        DebugStruct,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    mem::take,
};

use crate::{
    ctx::{
        Ctx,
        CtxValue,
        Invariant,
        map::CtxMapRef,
        mut1::MutCtx,
        ref1::CtxMeta,
    },
    mode::Mode,
    symbol::Symbol,
    transformer::Transformer,
    val::{
        Val,
        ctx::CtxVal,
    },
};

#[derive(Debug, Clone)]
pub struct Func<P, C> {
    pub(crate) call_mode: Mode,
    pub(crate) ask_mode: Mode,
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
    pub(crate) is_extension: bool,
    pub(crate) id: Symbol,
    pub(crate) ext: Ext,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite<Ext> {
    pub(crate) body_mode: Mode,
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

fn eval_free(prelude: &mut Ctx, input: Val, input_name: Symbol, mode: &Mode, body: Val) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    mode.transform(MutCtx::new(prelude), body)
}

#[allow(clippy::too_many_arguments)]
fn eval_aware(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    input: Val,
    input_name: Symbol,
    mode: &Mode,
    body: Val,
) -> Val {
    let _ = prelude
        .variables_mut()
        .put_value(input_name, CtxValue::new(input));
    keep_eval_restore(prelude, ctx, ctx_name, ctx_invariant, mode, body)
}

fn keep_eval_restore(
    mut prelude: Ctx,
    ctx: &mut Ctx,
    ctx_name: Symbol,
    ctx_invariant: Invariant,
    mode: &Mode,
    body: Val,
) -> Val {
    if !prelude.variables().fallback() && prelude.variables().is_assignable(ctx_name.clone()) {
        // here is why we need a `&mut Ctx` for a const func
        let caller = take(ctx);
        keep_ctx(&mut prelude, caller, ctx_name.clone(), ctx_invariant);
        let output = mode.transform(MutCtx::new(&mut prelude), body);
        restore_ctx(prelude, ctx, ctx_name);
        output
    } else {
        mode.transform(MutCtx::new(&mut prelude), body)
    }
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
        call_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        f: Primitive<P>,
    ) -> Self {
        Self {
            call_mode,
            ask_mode,
            cacheable,
            transformer: FuncImpl::Primitive(f),
        }
    }

    pub(crate) fn new_composite(
        call_mode: Mode,
        ask_mode: Mode,
        cacheable: bool,
        f: Composite<C>,
    ) -> Self {
        Self {
            call_mode,
            ask_mode,
            cacheable,
            transformer: FuncImpl::Composite(f),
        }
    }

    pub fn call_mode(&self) -> &Mode {
        &self.call_mode
    }

    pub fn ask_mode(&self) -> &Mode {
        &self.ask_mode
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

    pub(crate) fn id(&self) -> Option<Symbol> {
        let FuncImpl::Primitive(p) = &self.transformer else {
            return None;
        };
        Some(p.id.clone())
    }

    pub(crate) fn is_extension(&self) -> Option<bool> {
        let FuncImpl::Primitive(p) = &self.transformer else {
            return None;
        };
        Some(p.is_extension)
    }

    pub(crate) fn body_mode(&self) -> Option<&Mode> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(&c.body_mode)
    }

    pub(crate) fn body(&self) -> Option<&Val> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(&c.body)
    }

    pub(crate) fn prelude(&self) -> Option<&Ctx> {
        let FuncImpl::Composite(c) = &self.transformer else {
            return None;
        };
        Some(&c.prelude)
    }

    pub(crate) fn input_name(&self) -> Option<Symbol> {
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
        self.call_mode == other.call_mode
            && self.ask_mode == other.ask_mode
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
        self.call_mode.hash(state);
        self.ask_mode.hash(state);
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
        let mut s = f.debug_struct("Primitive");
        self.dbg_field(&mut s);
        s.finish()
    }
}

impl<T> Primitive<T> {
    pub(crate) fn dbg_field(&self, s: &mut DebugStruct) {
        s.field("id", &self.id);
        s.field("is_extension", &self.is_extension);
    }
}

impl<T> Composite<T> {
    pub(crate) fn dbg_field(&self, s: &mut DebugStruct) {
        s.field("body_mode", &self.body_mode);
        s.field("body", &self.body);
        s.field("prelude", &self.prelude);
        s.field("input_name", &self.input_name);
    }
}

impl<P, C> Func<P, C> {
    pub(crate) fn dbg_field(&self, s: &mut DebugStruct) {
        s.field("call_mode", &self.call_mode);
        s.field("ask_mode", &self.ask_mode);
        s.field("cacheable", &self.cacheable);
    }
}

pub(crate) mod mode;

pub(crate) mod cell;

pub(crate) mod free;

pub(crate) mod const1;

pub(crate) mod mut1;

pub(crate) mod repr;
