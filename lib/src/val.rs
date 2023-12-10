use {
    crate::{
        ctx::Ctx,
        func::Func,
        logic::Prop,
        syntax::{
            generator::GenerateRepr,
            parser::ParseRepr,
            repr::{
                CallRepr,
                ListRepr,
                MapRepr,
                PairRepr,
                Repr,
                ReverseRepr,
            },
        },
        types::{
            Bool,
            Bytes,
            Call,
            Float,
            Int,
            List,
            Map,
            Pair,
            Reader,
            Reverse,
            Str,
            Symbol,
            Unit,
        },
        ReprError,
    },
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::{
            Hash,
            Hasher,
        },
        ops::{
            ControlFlow,
            Deref,
            FromResidual,
            Try,
        },
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Unit(Unit),
    Bool(Bool),
    Int(Int),
    Float(Float),
    Bytes(Bytes),
    Symbol(Symbol),
    String(Str),
    Pair(Box<PairVal>),
    Call(Box<CallVal>),
    Reverse(Box<ReverseVal>),
    List(ListVal),
    Map(MapVal),

    Func(FuncVal),
    Ctx(CtxVal),

    Prop(PropVal),
}

pub type PairVal = Pair<Val, Val>;
pub type CallVal = Call<Val, Val>;
pub type ReverseVal = Reverse<Val, Val>;
pub type ListVal = List<Val>;
pub type MapVal = Map<Val, Val>;

#[derive(Clone, Eq)]
pub struct FuncVal(pub(crate) Reader<Func>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtxVal(pub(crate) Box<Ctx>);

#[derive(Clone, Eq)]
pub struct PropVal(pub(crate) Reader<Prop>);

#[allow(dead_code)]
impl Val {
    pub(crate) fn is_unit(&self) -> bool {
        matches!(self, Val::Unit(_))
    }
}

impl Default for Val {
    fn default() -> Self {
        Val::Unit(Unit)
    }
}

impl From<Unit> for Val {
    fn from(value: Unit) -> Self {
        Val::Unit(value)
    }
}

impl From<Bool> for Val {
    fn from(value: Bool) -> Self {
        Val::Bool(value)
    }
}

impl From<Int> for Val {
    fn from(value: Int) -> Self {
        Val::Int(value)
    }
}

impl From<Float> for Val {
    fn from(value: Float) -> Self {
        Val::Float(value)
    }
}

impl From<Bytes> for Val {
    fn from(value: Bytes) -> Self {
        Val::Bytes(value)
    }
}

impl From<Str> for Val {
    fn from(value: Str) -> Self {
        Val::String(value)
    }
}

impl From<Symbol> for Val {
    fn from(value: Symbol) -> Self {
        Val::Symbol(value)
    }
}

impl From<Box<PairVal>> for Val {
    fn from(value: Box<PairVal>) -> Self {
        Val::Pair(value)
    }
}

impl From<Box<CallVal>> for Val {
    fn from(value: Box<CallVal>) -> Self {
        Val::Call(value)
    }
}

impl From<Box<ReverseVal>> for Val {
    fn from(value: Box<ReverseVal>) -> Self {
        Val::Reverse(value)
    }
}

impl From<ListVal> for Val {
    fn from(value: ListVal) -> Self {
        Val::List(value)
    }
}

impl From<MapVal> for Val {
    fn from(value: MapVal) -> Self {
        Val::Map(value)
    }
}

impl From<FuncVal> for Val {
    fn from(value: FuncVal) -> Self {
        Val::Func(value)
    }
}

impl From<CtxVal> for Val {
    fn from(value: CtxVal) -> Self {
        Val::Ctx(value)
    }
}

impl From<PropVal> for Val {
    fn from(value: PropVal) -> Self {
        Val::Prop(value)
    }
}

impl From<&Repr> for Val {
    fn from(value: &Repr) -> Self {
        match value {
            Repr::Unit(u) => Val::Unit(*u),
            Repr::Bool(b) => Val::Bool(*b),
            Repr::Int(i) => Val::Int(i.clone()),
            Repr::Float(f) => Val::Float(f.clone()),
            Repr::Bytes(b) => Val::Bytes(b.clone()),
            Repr::Symbol(s) => Val::Symbol(s.clone()),
            Repr::String(s) => Val::String(s.clone()),
            Repr::Pair(p) => Val::Pair(Box::new(PairVal::from(&**p))),
            Repr::Call(c) => Val::Call(Box::new(CallVal::from(&**c))),
            Repr::Reverse(i) => Val::Reverse(Box::new(ReverseVal::from(&**i))),
            Repr::List(l) => Val::List(ListVal::from(l)),
            Repr::Map(m) => Val::Map(MapVal::from(m)),
        }
    }
}

impl From<Repr> for Val {
    fn from(value: Repr) -> Self {
        match value {
            Repr::Unit(u) => Val::Unit(u),
            Repr::Bool(b) => Val::Bool(b),
            Repr::Int(i) => Val::Int(i),
            Repr::Float(f) => Val::Float(f),
            Repr::Bytes(b) => Val::Bytes(b),
            Repr::Symbol(s) => Val::Symbol(s),
            Repr::String(s) => Val::String(s),
            Repr::Pair(p) => Val::Pair(Box::new(PairVal::from(*p))),
            Repr::Call(c) => Val::Call(Box::new(CallVal::from(*c))),
            Repr::Reverse(i) => Val::Reverse(Box::new(ReverseVal::from(*i))),
            Repr::List(l) => Val::List(ListVal::from(l)),
            Repr::Map(m) => Val::Map(MapVal::from(m)),
        }
    }
}

impl TryInto<Repr> for &Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(u) => Ok(Repr::Unit(*u)),
            Val::Bool(b) => Ok(Repr::Bool(*b)),
            Val::Int(i) => Ok(Repr::Int((*i).clone())),
            Val::Float(f) => Ok(Repr::Float((*f).clone())),
            Val::Bytes(b) => Ok(Repr::Bytes((*b).clone())),
            Val::Symbol(s) => Ok(Repr::Symbol((*s).clone())),
            Val::String(s) => Ok(Repr::String((*s).clone())),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(<_ as TryInto<PairRepr>>::try_into(
                &**p,
            )?))),
            Val::Call(c) => Ok(Repr::Call(Box::new(<_ as TryInto<CallRepr>>::try_into(
                &**c,
            )?))),
            Val::Reverse(i) => Ok(Repr::Reverse(Box::new(
                <_ as TryInto<ReverseRepr>>::try_into(&**i)?,
            ))),
            Val::List(l) => Ok(Repr::List(<_ as TryInto<ListRepr>>::try_into(l)?)),
            Val::Map(m) => Ok(Repr::Map(<_ as TryInto<MapRepr>>::try_into(m)?)),
            _ => Err(ReprError {}),
        }
    }
}

impl TryInto<Repr> for Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(u) => Ok(Repr::Unit(u)),
            Val::Bool(b) => Ok(Repr::Bool(b)),
            Val::Int(i) => Ok(Repr::Int(i)),
            Val::Float(f) => Ok(Repr::Float(f)),
            Val::Bytes(b) => Ok(Repr::Bytes(b)),
            Val::Symbol(s) => Ok(Repr::Symbol(s)),
            Val::String(s) => Ok(Repr::String(s)),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(<_ as TryInto<PairRepr>>::try_into(
                *p,
            )?))),
            Val::Call(c) => Ok(Repr::Call(Box::new(<_ as TryInto<CallRepr>>::try_into(
                *c,
            )?))),
            Val::Reverse(i) => Ok(Repr::Reverse(Box::new(
                <_ as TryInto<ReverseRepr>>::try_into(*i)?,
            ))),
            Val::List(l) => Ok(Repr::List(<_ as TryInto<ListRepr>>::try_into(l)?)),
            Val::Map(m) => Ok(Repr::Map(<_ as TryInto<MapRepr>>::try_into(m)?)),
            _ => Err(ReprError {}),
        }
    }
}

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        PairVal::new(Val::from(&value.first), Val::from(&value.second))
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        PairVal::new(Val::from(value.first), Val::from(value.second))
    }
}

impl TryInto<PairRepr> for &PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new(
            (&self.first).try_into()?,
            (&self.second).try_into()?,
        ))
    }
}

impl TryInto<PairRepr> for PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new(
            self.first.try_into()?,
            self.second.try_into()?,
        ))
    }
}

impl From<&CallRepr> for CallVal {
    fn from(value: &CallRepr) -> Self {
        CallVal::new(Val::from(&value.func), Val::from(&value.input))
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        CallVal::new(Val::from(value.func), Val::from(value.input))
    }
}

impl TryInto<CallRepr> for &CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(
            (&self.func).try_into()?,
            (&self.input).try_into()?,
        ))
    }
}

impl TryInto<CallRepr> for CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(self.func.try_into()?, self.input.try_into()?))
    }
}

impl From<&ReverseRepr> for ReverseVal {
    fn from(value: &ReverseRepr) -> Self {
        ReverseVal::new(Val::from(&value.func), Val::from(&value.output))
    }
}

impl From<ReverseRepr> for ReverseVal {
    fn from(value: ReverseRepr) -> Self {
        ReverseVal::new(Val::from(value.func), Val::from(value.output))
    }
}

impl TryInto<ReverseRepr> for &ReverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReverseRepr, Self::Error> {
        Ok(ReverseRepr::new(
            (&self.func).try_into()?,
            (&self.output).try_into()?,
        ))
    }
}

impl TryInto<ReverseRepr> for ReverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReverseRepr, Self::Error> {
        Ok(ReverseRepr::new(
            self.func.try_into()?,
            self.output.try_into()?,
        ))
    }
}

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        value.iter().map(|v| v.into()).collect::<Vec<Val>>().into()
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        <_ as Into<Vec<Repr>>>::into(value)
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<Val>>()
            .into()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        <_ as Into<Vec<Val>>>::into(self)
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<Repr>, Self::Error>>()
            .map(|v| v.into())
    }
}

impl TryInto<ListRepr> for &ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<Repr>, Self::Error>>()
            .map(|v| v.into())
    }
}

impl From<&MapRepr> for MapVal {
    fn from(value: &MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (<_ as Into<Val>>::into(k), <_ as Into<Val>>::into(v)))
            .collect()
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (<_ as Into<Val>>::into(k), <_ as Into<Val>>::into(v)))
            .collect()
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map::<Result<(Repr, Repr), Self::Error>, _>(|(k, v)| {
                Ok((
                    <_ as TryInto<Repr>>::try_into(k)?,
                    <_ as TryInto<Repr>>::try_into(v)?,
                ))
            })
            .collect::<Result<MapRepr, Self::Error>>()
    }
}

impl TryInto<MapRepr> for MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map::<Result<(Repr, Repr), Self::Error>, _>(|(k, v)| {
                Ok((
                    <_ as TryInto<Repr>>::try_into(k)?,
                    <_ as TryInto<Repr>>::try_into(v)?,
                ))
            })
            .collect::<Result<MapRepr, Self::Error>>()
    }
}

impl FromResidual for Val {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for Val {
    type Output = Val;
    type Residual = Val;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Val::Unit(_) => ControlFlow::Break(self),
            _ => ControlFlow::Continue(self),
        }
    }
}

impl ParseRepr for Val {
    fn try_into_pair(self) -> Result<(Self, Self), Self> {
        match self {
            Val::Pair(pair) => Ok((pair.first, pair.second)),
            other => Err(other),
        }
    }
}

impl<'a> TryInto<GenerateRepr<'a, Val>> for &'a Val {
    type Error = ReprError;

    fn try_into(self) -> Result<GenerateRepr<'a, Val>, Self::Error> {
        let r = match self {
            Val::Unit(u) => GenerateRepr::Unit(u),
            Val::Bool(b) => GenerateRepr::Bool(b),
            Val::Int(i) => GenerateRepr::Int(i),
            Val::Float(f) => GenerateRepr::Float(f),
            Val::Bytes(b) => GenerateRepr::Bytes(b),
            Val::Symbol(s) => GenerateRepr::Symbol(s),
            Val::String(s) => GenerateRepr::String(s),
            Val::Pair(p) => GenerateRepr::Pair(p),
            Val::Call(c) => GenerateRepr::Call(c),
            Val::Reverse(r) => GenerateRepr::Reverse(r),
            Val::List(l) => GenerateRepr::List(l),
            Val::Map(m) => GenerateRepr::Map(m),
            _ => return Err(ReprError {}),
        };
        Ok(r)
    }
}

impl From<Reader<Func>> for FuncVal {
    fn from(value: Reader<Func>) -> Self {
        FuncVal(value)
    }
}

impl PartialEq for FuncVal {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        *self.0 == *other.0
    }
}

impl Hash for FuncVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl From<Box<Ctx>> for CtxVal {
    fn from(value: Box<Ctx>) -> Self {
        CtxVal(value)
    }
}

impl From<Reader<Prop>> for PropVal {
    fn from(value: Reader<Prop>) -> Self {
        PropVal(value)
    }
}

impl PartialEq for PropVal {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        *self.0 == *other.0
    }
}

impl Hash for PropVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Unit(u) => <_ as Debug>::fmt(u, f),
            Val::Bool(b) => <_ as Debug>::fmt(b, f),
            Val::Int(i) => <_ as Debug>::fmt(i, f),
            Val::Float(float) => <_ as Debug>::fmt(float, f),
            Val::Bytes(b) => <_ as Debug>::fmt(b, f),
            Val::Symbol(s) => <_ as Debug>::fmt(s, f),
            Val::String(s) => <_ as Debug>::fmt(s, f),
            Val::Pair(p) => <_ as Debug>::fmt(p, f),
            Val::Call(c) => <_ as Debug>::fmt(c, f),
            Val::Reverse(r) => <_ as Debug>::fmt(r, f),
            Val::List(l) => <_ as Debug>::fmt(l, f),
            Val::Map(m) => <_ as Debug>::fmt(m, f),
            Val::Func(func) => <_ as Debug>::fmt(func, f),
            Val::Ctx(c) => <_ as Debug>::fmt(c, f),
            Val::Prop(p) => <_ as Debug>::fmt(p, f),
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Debug for CtxVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Debug for PropVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}