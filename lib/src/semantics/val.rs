use {
    crate::{
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
            Bytes,
            Call,
            Float,
            Int,
            Letter,
            List,
            Map,
            Pair,
            Str,
            Symbol,
            Unit,
        },
    },
    std::rc::Rc,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Val {
    Unit(Unit),
    Bool(Bool),
    Int(Rc<Int>),
    Float(Rc<Float>),
    Bytes(Rc<Bytes>),
    Letter(Rc<Letter>),
    Symbol(Rc<Symbol>),
    String(Rc<Str>),
    Pair(Rc<PairVal>),
    Call(Rc<CallVal>),
    List(Rc<ListVal>),
    Map(Rc<MapVal>),
    // todo more val
    Extend(Rc<ExtendVal>),
}

pub(crate) type PairVal = Pair<Rc<Val>, Rc<Val>>;
pub(crate) type CallVal = Call<Rc<Val>, Rc<Val>>;
pub(crate) type ListVal = List<Rc<Val>>;
pub(crate) type MapVal = Map<Rc<Val>, Rc<Val>>;

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
    pub fn int(&self) -> Option<&Rc<Int>> {
        if let Val::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn float(&self) -> Option<&Rc<Float>> {
        if let Val::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn bytes(&self) -> Option<&Rc<Bytes>> {
        if let Val::Bytes(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&Rc<Str>> {
        if let Val::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn letter(&self) -> Option<&Rc<Letter>> {
        if let Val::Letter(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn symbol(&self) -> Option<&Rc<Symbol>> {
        if let Val::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn pair(&self) -> Option<&Rc<PairVal>> {
        if let Val::Pair(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn call(&self) -> Option<&Rc<CallVal>> {
        if let Val::Call(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn list(&self) -> Option<&Rc<ListVal>> {
        if let Val::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map(&self) -> Option<&Rc<MapVal>> {
        if let Val::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn extend(&self) -> Option<&Rc<ExtendVal>> {
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

impl From<Rc<Int>> for Val {
    fn from(value: Rc<Int>) -> Self {
        Val::Int(value)
    }
}

impl From<Rc<Float>> for Val {
    fn from(value: Rc<Float>) -> Self {
        Val::Float(value)
    }
}

impl From<Rc<Bytes>> for Val {
    fn from(value: Rc<Bytes>) -> Self {
        Val::Bytes(value)
    }
}

impl From<Rc<Str>> for Val {
    fn from(value: Rc<Str>) -> Self {
        Val::String(value)
    }
}

impl From<Rc<Letter>> for Val {
    fn from(value: Rc<Letter>) -> Self {
        Val::Letter(value)
    }
}

impl From<Rc<Symbol>> for Val {
    fn from(value: Rc<Symbol>) -> Self {
        Val::Symbol(value)
    }
}

impl From<Rc<PairVal>> for Val {
    fn from(value: Rc<PairVal>) -> Self {
        Val::Pair(value)
    }
}

impl From<Rc<CallVal>> for Val {
    fn from(value: Rc<CallVal>) -> Self {
        Val::Call(value)
    }
}

impl From<Rc<ListVal>> for Val {
    fn from(value: Rc<ListVal>) -> Self {
        Val::List(value)
    }
}

impl From<Rc<MapVal>> for Val {
    fn from(value: Rc<MapVal>) -> Self {
        Val::Map(value)
    }
}

impl From<Rc<ExtendVal>> for Val {
    fn from(value: Rc<ExtendVal>) -> Self {
        Val::Extend(value)
    }
}

impl From<&Repr> for Val {
    fn from(value: &Repr) -> Self {
        match value {
            Repr::Unit(u) => Val::Unit(u.clone()),
            Repr::Bool(b) => Val::Bool(b.clone()),
            Repr::Int(i) => Val::Int((*i).clone()),
            Repr::Float(f) => Val::Float((*f).clone()),
            Repr::Bytes(b) => Val::Bytes((*b).clone()),
            Repr::Letter(l) => Val::Letter((*l).clone()),
            Repr::Symbol(s) => Val::Symbol((*s).clone()),
            Repr::String(s) => Val::String((*s).clone()),
            Repr::Pair(p) => Val::Pair(Rc::new(PairVal::from(&**p))),
            Repr::Call(c) => Val::Call(Rc::new(CallVal::from(&**c))),
            Repr::List(l) => Val::List(Rc::new(ListVal::from(l))),
            Repr::Map(m) => Val::Map(Rc::new(MapVal::from(m))),
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
            Repr::Pair(p) => Val::Pair(Rc::new(PairVal::from(*p))),
            Repr::Call(c) => Val::Call(Rc::new(CallVal::from(*c))),
            Repr::List(l) => Val::List(Rc::new(ListVal::from(l))),
            Repr::Map(m) => Val::Map(Rc::new(MapVal::from(m))),
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
            Val::List(l) => Ok(Repr::List(<_ as TryInto<ListRepr>>::try_into(&**l)?)),
            Val::Map(m) => Ok(Repr::Map(<_ as TryInto<MapRepr>>::try_into(&**m)?)),
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
            Val::Pair(p) => Ok(Repr::Pair(Box::new(<_ as TryInto<PairRepr>>::try_into(p)?))),
            Val::Call(c) => Ok(Repr::Call(Box::new(<_ as TryInto<CallRepr>>::try_into(c)?))),
            Val::List(l) => Ok(Repr::List(<_ as TryInto<ListRepr>>::try_into(l)?)),
            Val::Map(m) => Ok(Repr::Map(<_ as TryInto<MapRepr>>::try_into(m)?)),
            _ => Err(ReprError {}),
        }
    }
}

impl TryInto<Repr> for Rc<Val> {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match Rc::try_unwrap(self) {
            Ok(v) => Ok(v.try_into()?),
            Err(r) => Ok((&*r).try_into()?),
        }
    }
}

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        PairVal::new(
            Rc::new(Val::from(&value.first)),
            Rc::new(Val::from(&value.second)),
        )
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        PairVal::new(
            Rc::new(Val::from(value.first)),
            Rc::new(Val::from(value.second)),
        )
    }
}

impl TryInto<PairRepr> for &PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new(
            (&*self.first).try_into()?,
            (&*self.second).try_into()?,
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

impl TryInto<PairRepr> for Rc<PairVal> {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        match Rc::try_unwrap(self) {
            Ok(v) => Ok(v.try_into()?),
            Err(r) => Ok((&*r).try_into()?),
        }
    }
}

impl From<&CallRepr> for CallVal {
    fn from(value: &CallRepr) -> Self {
        CallVal::new(
            Rc::new(Val::from(&value.func)),
            Rc::new(Val::from(&value.arg)),
        )
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        CallVal::new(
            Rc::new(Val::from(value.func)),
            Rc::new(Val::from(value.arg)),
        )
    }
}

impl TryInto<CallRepr> for &CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(
            (&*self.func).try_into()?,
            (&*self.arg).try_into()?,
        ))
    }
}

impl TryInto<CallRepr> for CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(self.func.try_into()?, self.arg.try_into()?))
    }
}

impl TryInto<CallRepr> for Rc<CallVal> {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        match Rc::try_unwrap(self) {
            Ok(v) => Ok(v.try_into()?),
            Err(r) => Ok((&*r).try_into()?),
        }
    }
}

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        value
            .iter()
            .map(|v| Rc::new(v.into()))
            .collect::<Vec<Rc<Val>>>()
            .into()
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        <_ as Into<Vec<Repr>>>::into(value)
            .into_iter()
            .map(|v| Rc::new(v.into()))
            .collect::<Vec<Rc<Val>>>()
            .into()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        <_ as Into<Vec<Rc<Val>>>>::into(self)
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
            .map(|v| (&**v).try_into())
            .collect::<Result<Vec<Repr>, Self::Error>>()
            .map(|v| v.into())
    }
}

impl TryInto<ListRepr> for Rc<ListVal> {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        match Rc::try_unwrap(self) {
            Ok(v) => Ok(v.try_into()?),
            Err(r) => Ok((&*r).try_into()?),
        }
    }
}

impl From<&MapRepr> for MapVal {
    fn from(value: &MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| {
                (
                    Rc::new(<_ as Into<Val>>::into(k)),
                    Rc::new(<_ as Into<Val>>::into(v)),
                )
            })
            .collect()
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| {
                (
                    Rc::new(<_ as Into<Val>>::into(k)),
                    Rc::new(<_ as Into<Val>>::into(v)),
                )
            })
            .collect()
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map::<Result<(Repr, Repr), Self::Error>, _>(|(k, v)| {
                Ok((
                    <_ as TryInto<Repr>>::try_into(&**k)?,
                    <_ as TryInto<Repr>>::try_into(&**v)?,
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

impl TryInto<MapRepr> for Rc<MapVal> {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        match Rc::try_unwrap(self) {
            Ok(v) => Ok(v.try_into()?),
            Err(r) => Ok((&*r).try_into()?),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtendVal;
