use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
    rc::Rc,
};

use crate::{
    ctx::ref1::CtxMeta,
    func::{
        const1::ConstFunc,
        free::FreeFunc,
        mut1::MutFunc,
        static1::StaticFunc,
        FuncImpl,
    },
    transformer::Transformer,
    Ctx,
    Mode,
    Symbol,
    Val,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    Free(FreeFuncVal),
    Static(StaticFuncVal),
    Const(ConstFuncVal),
    Mut(MutFuncVal),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FreeFuncVal(Box<FreeFunc>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StaticFuncVal(Rc<StaticFunc>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ConstFuncVal(Rc<ConstFunc>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MutFuncVal(Rc<MutFunc>);

impl FuncVal {
    pub(crate) fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Free(f) => f.transform(input),
            FuncVal::Static(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Free(f) => f.transform_mut(input),
            FuncVal::Static(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn input_mode(&self) -> &Mode {
        match self {
            FuncVal::Free(f) => &f.input_mode,
            FuncVal::Static(f) => &f.input_mode,
            FuncVal::Const(f) => &f.input_mode,
            FuncVal::Mut(f) => &f.input_mode,
        }
    }

    pub(crate) fn output_mode(&self) -> &Mode {
        match self {
            FuncVal::Free(f) => &f.output_mode,
            FuncVal::Static(f) => &f.output_mode,
            FuncVal::Const(f) => &f.output_mode,
            FuncVal::Mut(f) => &f.output_mode,
        }
    }

    pub(crate) fn cacheable(&self) -> bool {
        match self {
            FuncVal::Free(f) => f.cacheable,
            FuncVal::Static(f) => f.cacheable,
            FuncVal::Const(f) => f.cacheable,
            FuncVal::Mut(f) => f.cacheable,
        }
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Free(f) => f.is_primitive(),
            FuncVal::Static(f) => f.is_primitive(),
            FuncVal::Const(f) => f.is_primitive(),
            FuncVal::Mut(f) => f.is_primitive(),
        }
    }

    pub(crate) fn primitive_id(&self) -> Option<Symbol> {
        match self {
            FuncVal::Free(f) => f.primitive_id(),
            FuncVal::Static(f) => f.primitive_id(),
            FuncVal::Const(f) => f.primitive_id(),
            FuncVal::Mut(f) => f.primitive_id(),
        }
    }

    pub(crate) fn primitive_is_extension(&self) -> Option<bool> {
        match self {
            FuncVal::Free(f) => f.primitive_is_extension(),
            FuncVal::Static(f) => f.primitive_is_extension(),
            FuncVal::Const(f) => f.primitive_is_extension(),
            FuncVal::Mut(f) => f.primitive_is_extension(),
        }
    }

    pub(crate) fn composite_body(&self) -> Option<&Val> {
        match self {
            FuncVal::Free(f) => f.composite_body(),
            FuncVal::Static(f) => f.composite_body(),
            FuncVal::Const(f) => f.composite_body(),
            FuncVal::Mut(f) => f.composite_body(),
        }
    }

    pub(crate) fn composite_prelude(&self) -> Option<&Ctx> {
        match self {
            FuncVal::Free(f) => f.composite_prelude(),
            FuncVal::Static(f) => f.composite_prelude(),
            FuncVal::Const(f) => f.composite_prelude(),
            FuncVal::Mut(f) => f.composite_prelude(),
        }
    }

    pub(crate) fn composite_input_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Free(f) => f.composite_input_name(),
            FuncVal::Static(f) => f.composite_input_name(),
            FuncVal::Const(f) => f.composite_input_name(),
            FuncVal::Mut(f) => f.composite_input_name(),
        }
    }

    pub(crate) fn composite_ctx_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Free(_) => None,
            FuncVal::Static(_) => None,
            FuncVal::Const(f) => {
                let FuncImpl::Composite(c) = f.transformer() else {
                    return None;
                };
                Some(c.ext.ctx_name.clone())
            }
            FuncVal::Mut(f) => {
                let FuncImpl::Composite(c) = f.transformer() else {
                    return None;
                };
                Some(c.ext.ctx_name.clone())
            }
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncVal::Free(func) => FreeFuncVal::fmt(func, f),
            FuncVal::Static(func) => StaticFuncVal::fmt(func, f),
            FuncVal::Const(func) => ConstFuncVal::fmt(func, f),
            FuncVal::Mut(func) => MutFuncVal::fmt(func, f),
        }
    }
}

impl FreeFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Box<FreeFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<FreeFunc> {
        self.0
    }
}

impl From<FreeFunc> for FreeFuncVal {
    fn from(value: FreeFunc) -> Self {
        Self(Box::new(value))
    }
}

impl Debug for FreeFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FreeFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                s.field("id", p.get_id());
                s.field("is_extension", &p.is_extension());
            }
            FuncImpl::Composite(c) => {
                s.field("input_mode", &self.input_mode);
                s.field("output_mode", &self.output_mode);
                s.field("body", &c.body);
                s.field("prelude", &c.prelude);
                s.field("input_name", &c.input_name);
            }
        }
        s.finish()
    }
}

impl Deref for FreeFuncVal {
    type Target = FreeFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FreeFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StaticFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Rc<StaticFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<StaticFunc> {
        self.0
    }
}

impl From<StaticFunc> for StaticFuncVal {
    fn from(value: StaticFunc) -> Self {
        Self(Rc::new(value))
    }
}

impl Debug for StaticFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("StaticFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                s.field("id", p.get_id());
                s.field("is_extension", &p.is_extension());
            }
            FuncImpl::Composite(c) => {
                s.field("input_mode", &self.input_mode);
                s.field("output_mode", &self.output_mode);
                s.field("body", &c.body);
                s.field("prelude", &c.prelude);
                s.field("input_name", &c.input_name);
            }
        }
        s.finish()
    }
}

impl Deref for StaticFuncVal {
    type Target = StaticFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StaticFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl ConstFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Rc<ConstFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<ConstFunc> {
        self.0
    }
}

impl From<ConstFunc> for ConstFuncVal {
    fn from(value: ConstFunc) -> Self {
        Self(Rc::new(value))
    }
}

impl Debug for ConstFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ConstFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                s.field("id", p.get_id());
                s.field("is_extension", &p.is_extension());
            }
            FuncImpl::Composite(c) => {
                s.field("input_mode", &self.input_mode);
                s.field("output_mode", &self.output_mode);
                s.field("body", &c.body);
                s.field("prelude", &c.prelude);
                s.field("context_name", &c.ext.ctx_name);
                s.field("input_name", &c.input_name);
            }
        }
        s.finish()
    }
}

impl Deref for ConstFuncVal {
    type Target = ConstFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConstFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl MutFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Rc<MutFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<MutFunc> {
        self.0
    }
}

impl From<MutFunc> for MutFuncVal {
    fn from(value: MutFunc) -> Self {
        Self(Rc::new(value))
    }
}

impl Debug for MutFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("MutFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                s.field("id", p.get_id());
                s.field("is_extension", &p.is_extension());
            }
            FuncImpl::Composite(c) => {
                s.field("input_mode", &self.input_mode);
                s.field("output_mode", &self.output_mode);
                s.field("body", &c.body);
                s.field("prelude", &c.prelude);
                s.field("context_name", &c.ext.ctx_name);
                s.field("input_name", &c.input_name);
            }
        }
        s.finish()
    }
}

impl Deref for MutFuncVal {
    type Target = MutFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MutFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}
