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
    Ctx,
    Mode,
    PrimitiveMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncImpl,
        cell::CellFunc,
        const1::ConstFunc,
        free::FreeFunc,
        mode::ModeFunc,
        mut1::MutFunc,
    },
    transformer::Transformer,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    Mode(ModeFuncVal),
    Cell(CellFuncVal),
    Free(FreeFuncVal),
    Const(ConstFuncVal),
    Mut(MutFuncVal),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModeFuncVal(Rc<ModeFunc>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CellFuncVal(Box<CellFunc>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FreeFuncVal(Rc<FreeFunc>);

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
            FuncVal::Mode(f) => f.transform(ctx, input),
            FuncVal::Cell(f) => f.transform(input),
            FuncVal::Free(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            FuncVal::Mode(f) => f.transform(ctx, input),
            FuncVal::Cell(f) => f.transform_mut(input),
            FuncVal::Free(f) => f.transform(ctx, input),
            FuncVal::Const(f) => f.transform(ctx, input),
            FuncVal::Mut(f) => f.transform(ctx, input),
        }
    }

    pub(crate) fn call_mode(&self) -> &Mode {
        match self {
            FuncVal::Mode(_) => &Mode::Primitive(PrimitiveMode::Id),
            FuncVal::Cell(f) => &f.call_mode,
            FuncVal::Free(f) => &f.call_mode,
            FuncVal::Const(f) => &f.call_mode,
            FuncVal::Mut(f) => &f.call_mode,
        }
    }

    pub(crate) fn ask_mode(&self) -> &Mode {
        match self {
            FuncVal::Mode(_) => &Mode::Primitive(PrimitiveMode::Eval),
            FuncVal::Cell(f) => &f.ask_mode,
            FuncVal::Free(f) => &f.ask_mode,
            FuncVal::Const(f) => &f.ask_mode,
            FuncVal::Mut(f) => &f.ask_mode,
        }
    }

    pub(crate) fn cacheable(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.cacheable(),
            FuncVal::Cell(f) => f.cacheable,
            FuncVal::Free(f) => f.cacheable,
            FuncVal::Const(f) => f.cacheable,
            FuncVal::Mut(f) => f.cacheable,
        }
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.is_primitive(),
            FuncVal::Cell(f) => f.is_primitive(),
            FuncVal::Free(f) => f.is_primitive(),
            FuncVal::Const(f) => f.is_primitive(),
            FuncVal::Mut(f) => f.is_primitive(),
        }
    }

    pub(crate) fn id(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.id(),
            FuncVal::Free(f) => f.id(),
            FuncVal::Const(f) => f.id(),
            FuncVal::Mut(f) => f.id(),
        }
    }

    pub(crate) fn is_extension(&self) -> Option<bool> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.is_extension(),
            FuncVal::Free(f) => f.is_extension(),
            FuncVal::Const(f) => f.is_extension(),
            FuncVal::Mut(f) => f.is_extension(),
        }
    }

    pub(crate) fn body_mode(&self) -> Option<&Mode> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.body_mode(),
            FuncVal::Free(f) => f.body_mode(),
            FuncVal::Const(f) => f.body_mode(),
            FuncVal::Mut(f) => f.body_mode(),
        }
    }

    pub(crate) fn body(&self) -> Option<&Val> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.body(),
            FuncVal::Free(f) => f.body(),
            FuncVal::Const(f) => f.body(),
            FuncVal::Mut(f) => f.body(),
        }
    }

    pub(crate) fn prelude(&self) -> Option<&Ctx> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.prelude(),
            FuncVal::Free(f) => f.prelude(),
            FuncVal::Const(f) => f.prelude(),
            FuncVal::Mut(f) => f.prelude(),
        }
    }

    pub(crate) fn input_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(f) => f.input_name(),
            FuncVal::Free(f) => f.input_name(),
            FuncVal::Const(f) => f.input_name(),
            FuncVal::Mut(f) => f.input_name(),
        }
    }

    pub(crate) fn ctx_name(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::Cell(_) => None,
            FuncVal::Free(_) => None,
            FuncVal::Const(f) => f.ctx_name(),
            FuncVal::Mut(f) => f.ctx_name(),
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncVal::Mode(func) => ModeFuncVal::fmt(func, f),
            FuncVal::Cell(func) => CellFuncVal::fmt(func, f),
            FuncVal::Free(func) => FreeFuncVal::fmt(func, f),
            FuncVal::Const(func) => ConstFuncVal::fmt(func, f),
            FuncVal::Mut(func) => MutFuncVal::fmt(func, f),
        }
    }
}

impl ModeFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Rc<ModeFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<ModeFunc> {
        self.0
    }
}

impl From<ModeFunc> for ModeFuncVal {
    fn from(func: ModeFunc) -> Self {
        Self(Rc::new(func))
    }
}

impl Debug for ModeFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ModeFunc");
        s.field("mode", self.mode());
        s.finish()
    }
}

impl Deref for ModeFuncVal {
    type Target = ModeFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModeFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl CellFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Box<CellFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<CellFunc> {
        self.0
    }
}

impl From<CellFunc> for CellFuncVal {
    fn from(value: CellFunc) -> Self {
        Self(Box::new(value))
    }
}

impl Debug for CellFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("CellFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                p.dbg_field(&mut s);
                p.dbg_field_ext(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
            }
        }
        s.finish()
    }
}

impl Deref for CellFuncVal {
    type Target = CellFunc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CellFuncVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FreeFuncVal {
    #[allow(unused)]
    pub(crate) fn new(func: Rc<FreeFunc>) -> Self {
        Self(func)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<FreeFunc> {
        self.0
    }
}

impl From<FreeFunc> for FreeFuncVal {
    fn from(value: FreeFunc) -> Self {
        Self(Rc::new(value))
    }
}

impl Debug for FreeFuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FreeFunc");
        match &self.transformer {
            FuncImpl::Primitive(p) => {
                p.dbg_field(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
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
                p.dbg_field(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
                c.dbg_field_ext(&mut s);
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
                p.dbg_field(&mut s);
            }
            FuncImpl::Composite(c) => {
                self.dbg_field(&mut s);
                c.dbg_field(&mut s);
                c.dbg_field_ext(&mut s);
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
