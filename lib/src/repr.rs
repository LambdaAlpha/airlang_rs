use crate::types::{
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
};

#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(not(feature = "syntax"), derive(Debug))]
pub enum Repr {
    Unit(Unit),
    Bool(Bool),
    Int(Int),
    Float(Float),
    Bytes(Bytes),
    Letter(Letter),
    Symbol(Symbol),
    String(Str),
    Pair(Box<PairRepr>),
    Call(Box<CallRepr>),
    List(ListRepr),
    Map(MapRepr),
}

pub type PairRepr = Pair<Repr, Repr>;
pub type CallRepr = Call<Repr, Repr>;
pub type ListRepr = List<Repr>;
pub type MapRepr = Map<Repr, Repr>;

impl Repr {
    pub fn unit(&self) -> Option<&Unit> {
        if let Repr::Unit(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn bool(&self) -> Option<&Bool> {
        if let Repr::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn int(&self) -> Option<&Int> {
        if let Repr::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn float(&self) -> Option<&Float> {
        if let Repr::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn bytes(&self) -> Option<&Bytes> {
        if let Repr::Bytes(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn letter(&self) -> Option<&Letter> {
        if let Repr::Letter(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn symbol(&self) -> Option<&Symbol> {
        if let Repr::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn string(&self) -> Option<&Str> {
        if let Repr::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn pair(&self) -> Option<&Box<PairRepr>> {
        if let Repr::Pair(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn call(&self) -> Option<&Box<CallRepr>> {
        if let Repr::Call(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn list(&self) -> Option<&ListRepr> {
        if let Repr::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn map(&self) -> Option<&MapRepr> {
        if let Repr::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Unit> for Repr {
    fn from(u: Unit) -> Self {
        Repr::Unit(u)
    }
}

impl From<Bool> for Repr {
    fn from(b: Bool) -> Self {
        Repr::Bool(b)
    }
}

impl From<Int> for Repr {
    fn from(i: Int) -> Self {
        Repr::Int(i)
    }
}

impl From<Float> for Repr {
    fn from(f: Float) -> Self {
        Repr::Float(f)
    }
}

impl From<Bytes> for Repr {
    fn from(b: Bytes) -> Self {
        Repr::Bytes(b)
    }
}

impl From<Letter> for Repr {
    fn from(l: Letter) -> Self {
        Repr::Letter(l)
    }
}

impl From<Symbol> for Repr {
    fn from(s: Symbol) -> Self {
        Repr::Symbol(s)
    }
}

impl From<Str> for Repr {
    fn from(s: Str) -> Self {
        Repr::String(s)
    }
}

impl From<Box<PairRepr>> for Repr {
    fn from(p: Box<PairRepr>) -> Self {
        Repr::Pair(p)
    }
}

impl From<Box<CallRepr>> for Repr {
    fn from(c: Box<CallRepr>) -> Self {
        Repr::Call(c)
    }
}

impl From<ListRepr> for Repr {
    fn from(l: ListRepr) -> Self {
        Repr::List(l)
    }
}

impl From<MapRepr> for Repr {
    fn from(m: MapRepr) -> Self {
        Repr::Map(m)
    }
}
