use {
    crate::{
        syntax::{
            generate,
            parse,
            ParseError,
        },
        traits::TryClone,
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
            Reverse,
            Str,
            Symbol,
            Unit,
        },
    },
    std::{
        fmt::{
            Debug,
            Display,
        },
        ops::{
            ControlFlow,
            FromResidual,
            Try,
        },
        str::FromStr,
    },
};

#[derive(PartialEq, Eq, Clone, Hash)]
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
    Reverse(Box<ReverseRepr>),
    List(ListRepr),
    Map(MapRepr),
}

pub type PairRepr = Pair<Repr, Repr>;
pub type CallRepr = Call<Repr, Repr>;
pub type ReverseRepr = Reverse<Repr, Repr>;
pub type ListRepr = List<Repr>;
pub type MapRepr = Map<Repr, Repr>;

impl Repr {
    pub fn is_unit(&self) -> bool {
        matches!(self, Repr::Unit(_))
    }

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
    pub fn reverse(&self) -> Option<&Box<ReverseRepr>> {
        if let Repr::Reverse(v) = self {
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
    fn from(a: Box<CallRepr>) -> Self {
        Repr::Call(a)
    }
}

impl From<Box<ReverseRepr>> for Repr {
    fn from(i: Box<ReverseRepr>) -> Self {
        Repr::Reverse(i)
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

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl TryFrom<&str> for Repr {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse(value)
    }
}

impl FromStr for Repr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Into<String> for &Repr {
    fn into(self) -> String {
        generate(self)
    }
}

impl FromResidual for Repr {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for Repr {
    type Output = Repr;
    type Residual = Repr;

    fn from_output(output: Self::Output) -> Self {
        output
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Repr::Unit(_) => ControlFlow::Break(self),
            _ => ControlFlow::Continue(self),
        }
    }
}

impl TryClone for Repr {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
    }
}
