use {
    crate::grammar,
    std::fmt::{
        Debug,
        Display,
    },
};

#[derive(PartialEq, Clone)]
pub enum Repr {
    Unit,
    Bool(Bool),
    Int(Box<Int>),
    Float(Box<Float>),
    Letter(Box<String>),
    Symbol(Box<String>),
    String(Box<String>),
    Bytes(Box<Bytes>),
    List(Box<List>),
    Map(Box<Map>),
    Ltree(Box<Ltree>),
    Mtree(Box<Mtree>),
    Infix(Box<Infix>),
}

impl Repr {
    pub fn unit() -> Repr {
        Repr::Unit
    }
    pub fn bool(b: Bool) -> Repr {
        Repr::Bool(b)
    }
    pub fn int(i: Int) -> Repr {
        Repr::Int(Box::new(i))
    }
    pub fn float(f: Float) -> Repr {
        Repr::Float(Box::new(f))
    }
    pub fn letter(s: String) -> Repr {
        Repr::Letter(Box::new(s))
    }
    pub fn symbol(s: String) -> Repr {
        Repr::Symbol(Box::new(s))
    }
    pub fn string(s: String) -> Repr {
        Repr::String(Box::new(s))
    }
    pub fn bytes(b: Bytes) -> Repr {
        Repr::Bytes(Box::new(b))
    }
    pub fn list(l: List) -> Repr {
        Repr::List(Box::new(l))
    }
    pub fn map(m: Map) -> Repr {
        Repr::Map(Box::new(m))
    }
    pub fn ltree(root: Repr, leaves: List) -> Repr {
        Repr::Ltree(Box::new(Ltree { root, leaves }))
    }
    pub fn mtree(root: Repr, leaves: Map) -> Repr {
        Repr::Mtree(Box::new(Mtree { root, leaves }))
    }
    pub fn infix(left: Repr, infix: Repr, right: Repr) -> Repr {
        Repr::Infix(Box::new(Infix { infix, left, right }))
    }
}

impl From<Unit> for Repr {
    fn from(_: Unit) -> Self {
        Repr::Unit
    }
}

impl From<Bool> for Repr {
    fn from(b: Bool) -> Self {
        Repr::Bool(b)
    }
}

impl From<Int> for Repr {
    fn from(i: Int) -> Self {
        Repr::Int(Box::new(i))
    }
}

impl From<Float> for Repr {
    fn from(f: Float) -> Self {
        Repr::Float(Box::new(f))
    }
}

impl From<String> for Repr {
    fn from(s: String) -> Self {
        Repr::String(Box::new(s))
    }
}

impl From<Bytes> for Repr {
    fn from(b: Bytes) -> Self {
        Repr::Bytes(Box::new(b))
    }
}

impl From<List> for Repr {
    fn from(l: List) -> Self {
        Repr::List(Box::new(l))
    }
}

impl From<Map> for Repr {
    fn from(m: Map) -> Self {
        Repr::Map(Box::new(m))
    }
}

impl From<Ltree> for Repr {
    fn from(ltree: Ltree) -> Self {
        Repr::Ltree(Box::new(ltree))
    }
}

impl From<Mtree> for Repr {
    fn from(mtree: Mtree) -> Self {
        Repr::Mtree(Box::new(mtree))
    }
}

impl From<Infix> for Repr {
    fn from(infix: Infix) -> Self {
        Repr::Infix(Box::new(infix))
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", grammar::stringify_pretty(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", grammar::stringify_pretty(self))
    }
}

pub type Unit = ();

pub type Bool = bool;

#[derive(Debug, PartialEq, Clone)]
pub struct Int {
    pub sign: bool,
    pub radix: u8,
    pub digits: std::string::String,
}

impl Int {
    pub fn new(sign: bool, radix: u8, digits: String) -> Self {
        Self {
            sign,
            radix,
            digits,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub sign: bool,
    pub integral: std::string::String,
    pub fractional: std::string::String,
    pub exp_sign: bool,
    pub exp_digits: std::string::String,
}

impl Float {
    pub fn new(
        sign: bool,
        integral: String,
        fractional: String,
        exp_sign: bool,
        exp_digits: String,
    ) -> Self {
        Self {
            sign,
            integral,
            fractional,
            exp_sign,
            exp_digits,
        }
    }
}

pub type String = std::string::String;

pub type Bytes = Vec<u8>;

pub type List = Vec<Repr>;

pub type Map = Vec<(Repr, Repr)>;

#[derive(Debug, PartialEq, Clone)]
pub struct Ltree {
    pub root: Repr,
    pub leaves: List,
}

impl Ltree {
    pub fn new(root: Repr, leaves: List) -> Self {
        Self { root, leaves }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mtree {
    pub root: Repr,
    pub leaves: Map,
}

impl Mtree {
    pub fn new(root: Repr, leaves: Map) -> Self {
        Self { root, leaves }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Infix {
    pub infix: Repr,
    pub left: Repr,
    pub right: Repr,
}

impl Infix {
    pub fn new(infix: Repr, left: Repr, right: Repr) -> Self {
        Self { infix, left, right }
    }
}
