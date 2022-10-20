use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use rustc_hash::FxHashMap;

use crate::grammar;
use crate::num;

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
    pub fn letter(s: String) -> Repr {
        Repr::Letter(Box::new(s))
    }
    pub fn symbol(s: String) -> Repr {
        Repr::Symbol(Box::new(s))
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

impl From<()> for Repr {
    fn from(_: ()) -> Self {
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

impl Hash for Repr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Repr::Unit => ().hash(state),
            Repr::Bool(b) => b.hash(state),
            Repr::Int(i) => i.hash(state),
            Repr::Float(f) => f.as_ord().hash(state),
            Repr::String(s) => s.hash(state),
            Repr::Letter(s) => s.hash(state),
            Repr::Symbol(s) => s.hash(state),
            Repr::Bytes(b) => b.hash(state),
            Repr::List(l) => l.hash(state),
            Repr::Map(m) => {
                for i in m.iter() {
                    i.hash(state);
                }
            }
            Repr::Ltree(lt) => lt.hash(state),
            Repr::Mtree(mt) => mt.hash(state),
            Repr::Infix(i) => i.hash(state),
        }
    }
}

impl Eq for Repr {
    fn assert_receiver_is_total_eq(&self) {}
}

pub type Bool = bool;

pub type Int = num::Integer;

pub type Float = num::Float;

pub type String = std::string::String;

pub type Bytes = Vec<u8>;

pub type List = Vec<Repr>;

pub type Map = FxHashMap<Repr, Repr>;

pub mod map {
    use super::{Map, Repr};

    pub fn from<const N: usize>(pairs: [(Repr, Repr); N]) -> Map {
        let mut map = Map::default();
        for pair in pairs {
            map.insert(pair.0, pair.1);
        }
        map
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Ltree {
    pub root: Repr,
    pub leaves: List,
}

impl Ltree {
    pub fn new(root: Repr, leaves: List) -> Ltree {
        Ltree { root, leaves }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Mtree {
    pub root: Repr,
    pub leaves: Map,
}

impl Mtree {
    pub fn new(root: Repr, leaves: Map) -> Mtree {
        Mtree { root, leaves }
    }
}

impl Hash for Mtree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root.hash(state);
        for i in self.leaves.iter() {
            i.hash(state);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Infix {
    pub infix: Repr,
    pub left: Repr,
    pub right: Repr,
}
