use crate::{
    repr::{
        CallRepr,
        ListRepr,
        MapRepr,
        PairRepr,
        Repr,
    },
    semantics::ReprError,
    types::{
        Bool,
        BoxRef,
        Bytes,
        Call,
        Float,
        ImRef,
        Int,
        Letter,
        List,
        Map,
        MutRef,
        Pair,
        Str,
        Symbol,
        Unit,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Val {
    Unit(Unit),
    Bool(Bool),
    Int(Int),
    Float(Float),
    Bytes(Bytes),
    Letter(Letter),
    Symbol(Symbol),
    String(Str),
    Pair(Box<PairVal>),
    Call(Box<CallVal>),
    List(ListVal),
    Map(MapVal),

    BoxRef(BoxRefVal),
    ImRef(ImRefVal),
    MutRef(MutRefVal),

    // todo more val
    Extend(ExtendVal),
}

pub(crate) type PairVal = Pair<Val, Val>;
pub(crate) type CallVal = Call<Val, Val>;
pub(crate) type ListVal = List<Val>;
pub(crate) type MapVal = Map<Repr, Val>;
pub(crate) type BoxRefVal = BoxRef<Val>;
pub(crate) type ImRefVal = ImRef<Val>;
pub(crate) type MutRefVal = MutRef<Val>;

#[allow(dead_code)]
impl Val {
    pub fn unit(&self) -> Option<&Unit> {
        if let Val::Unit(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn bool(&self) -> Option<&Bool> {
        if let Val::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn int(&self) -> Option<&Int> {
        if let Val::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn float(&self) -> Option<&Float> {
        if let Val::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn bytes(&self) -> Option<&Bytes> {
        if let Val::Bytes(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&Str> {
        if let Val::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn letter(&self) -> Option<&Letter> {
        if let Val::Letter(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn symbol(&self) -> Option<&Symbol> {
        if let Val::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn pair(&self) -> Option<&Box<PairVal>> {
        if let Val::Pair(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn call(&self) -> Option<&Box<CallVal>> {
        if let Val::Call(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn list(&self) -> Option<&ListVal> {
        if let Val::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map(&self) -> Option<&MapVal> {
        if let Val::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn box_ref(&self) -> Option<&BoxRefVal> {
        if let Val::BoxRef(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn im_ref(&self) -> Option<&ImRefVal> {
        if let Val::ImRef(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn mut_ref(&self) -> Option<&MutRefVal> {
        if let Val::MutRef(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn extended(&self) -> Option<&ExtendVal> {
        if let Val::Extend(v) = self {
            Some(v)
        } else {
            None
        }
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

impl From<Letter> for Val {
    fn from(value: Letter) -> Self {
        Val::Letter(value)
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

impl From<BoxRefVal> for Val {
    fn from(value: BoxRefVal) -> Self {
        Val::BoxRef(value)
    }
}

impl From<ImRefVal> for Val {
    fn from(value: ImRefVal) -> Self {
        Val::ImRef(value)
    }
}

impl From<MutRefVal> for Val {
    fn from(value: MutRefVal) -> Self {
        Val::MutRef(value)
    }
}

impl From<ExtendVal> for Val {
    fn from(value: ExtendVal) -> Self {
        Val::Extend(value)
    }
}

impl From<&Repr> for Val {
    fn from(value: &Repr) -> Self {
        match value {
            Repr::Unit(u) => Val::Unit(u.clone()),
            Repr::Bool(b) => Val::Bool(b.clone()),
            Repr::Int(i) => Val::Int(i.clone()),
            Repr::Float(f) => Val::Float(f.clone()),
            Repr::Bytes(b) => Val::Bytes(b.clone()),
            Repr::Letter(l) => Val::Letter(l.clone()),
            Repr::Symbol(s) => Val::Symbol(s.clone()),
            Repr::String(s) => Val::String(s.clone()),
            Repr::Pair(p) => Val::Pair(Box::new(PairVal::from(&**p))),
            Repr::Call(c) => Val::Call(Box::new(CallVal::from(&**c))),
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
            Repr::Letter(l) => Val::Letter(l),
            Repr::Symbol(s) => Val::Symbol(s),
            Repr::String(s) => Val::String(s),
            Repr::Pair(p) => Val::Pair(Box::new(PairVal::from(*p))),
            Repr::Call(c) => Val::Call(Box::new(CallVal::from(*c))),
            Repr::List(l) => Val::List(ListVal::from(l)),
            Repr::Map(m) => Val::Map(MapVal::from(m)),
        }
    }
}

impl TryInto<Repr> for &Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(u) => Ok(Repr::Unit(u.clone())),
            Val::Bool(b) => Ok(Repr::Bool(b.clone())),
            Val::Int(i) => Ok(Repr::Int((*i).clone())),
            Val::Float(f) => Ok(Repr::Float((*f).clone())),
            Val::Bytes(b) => Ok(Repr::Bytes((*b).clone())),
            Val::Letter(l) => Ok(Repr::Letter((*l).clone())),
            Val::Symbol(s) => Ok(Repr::Symbol((*s).clone())),
            Val::String(s) => Ok(Repr::String((*s).clone())),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(<_ as TryInto<PairRepr>>::try_into(
                &**p,
            )?))),
            Val::Call(c) => Ok(Repr::Call(Box::new(<_ as TryInto<CallRepr>>::try_into(
                &**c,
            )?))),
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
            Val::Letter(l) => Ok(Repr::Letter(l)),
            Val::Symbol(s) => Ok(Repr::Symbol(s)),
            Val::String(s) => Ok(Repr::String(s)),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(<_ as TryInto<PairRepr>>::try_into(
                *p,
            )?))),
            Val::Call(c) => Ok(Repr::Call(Box::new(<_ as TryInto<CallRepr>>::try_into(
                *c,
            )?))),
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
        CallVal::new(Val::from(&value.func), Val::from(&value.arg))
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        CallVal::new(Val::from(value.func), Val::from(value.arg))
    }
}

impl TryInto<CallRepr> for &CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(
            (&self.func).try_into()?,
            (&self.arg).try_into()?,
        ))
    }
}

impl TryInto<CallRepr> for CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(self.func.try_into()?, self.arg.try_into()?))
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
            .map(|(k, v)| (k.clone(), <_ as Into<Val>>::into(v)))
            .collect()
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (k, <_ as Into<Val>>::into(v)))
            .collect()
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map::<Result<(Repr, Repr), Self::Error>, _>(|(k, v)| {
                Ok((k.clone(), <_ as TryInto<Repr>>::try_into(v)?))
            })
            .collect::<Result<MapRepr, Self::Error>>()
    }
}

impl TryInto<MapRepr> for MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map::<Result<(Repr, Repr), Self::Error>, _>(|(k, v)| {
                Ok((k, <_ as TryInto<Repr>>::try_into(v)?))
            })
            .collect::<Result<MapRepr, Self::Error>>()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendVal;
