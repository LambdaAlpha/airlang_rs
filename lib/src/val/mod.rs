use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::num;
use crate::parser;

#[derive(PartialEq)]
pub enum Val {
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

impl Val {
    pub fn letter(s: String) -> Val {
        Val::Letter(Box::new(s))
    }
    pub fn symbol(s: String) -> Val {
        Val::Symbol(Box::new(s))
    }
    pub fn ltree(root: Val, leaves: List) -> Val {
        Val::Ltree(Box::new(Ltree { root, leaves }))
    }
    pub fn mtree(root: Val, leaves: Map) -> Val {
        Val::Mtree(Box::new(Mtree { root, leaves }))
    }
    pub fn infix(left: Val, infix: Val, right: Val) -> Val {
        Val::Infix(Box::new(Infix { infix, left, right }))
    }
}

impl From<Bool> for Val {
    fn from(b: Bool) -> Self {
        Val::Bool(b)
    }
}

impl From<Int> for Val {
    fn from(i: Int) -> Self {
        Val::Int(Box::new(i))
    }
}

impl From<Float> for Val {
    fn from(f: Float) -> Self {
        Val::Float(Box::new(f))
    }
}

impl From<String> for Val {
    fn from(s: String) -> Self {
        Val::String(Box::new(s))
    }
}

impl From<Bytes> for Val {
    fn from(b: Bytes) -> Self {
        Val::Bytes(Box::new(b))
    }
}

impl From<List> for Val {
    fn from(l: List) -> Self {
        Val::List(Box::new(l))
    }
}

impl From<Map> for Val {
    fn from(m: Map) -> Self {
        Val::Map(Box::new(m))
    }
}

impl From<Ltree> for Val {
    fn from(ltree: Ltree) -> Self {
        Val::Ltree(Box::new(ltree))
    }
}

impl From<Mtree> for Val {
    fn from(mtree: Mtree) -> Self {
        Val::Mtree(Box::new(mtree))
    }
}

impl From<Infix> for Val {
    fn from(infix: Infix) -> Self {
        Val::Infix(Box::new(infix))
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", parser::stringify_pretty(self))
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", parser::stringify_pretty(self))
    }
}

impl Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Val::Bool(b) => b.hash(state),
            Val::Int(i) => i.hash(state),
            Val::Float(f) => f.as_ord().hash(state),
            Val::String(s) => s.hash(state),
            Val::Letter(s) => s.hash(state),
            Val::Symbol(s) => s.hash(state),
            Val::Bytes(b) => b.hash(state),
            Val::List(l) => l.hash(state),
            Val::Map(m) => {
                for i in m.iter() {
                    i.hash(state);
                }
            }
            Val::Ltree(lt) => lt.hash(state),
            Val::Mtree(mt) => mt.hash(state),
            Val::Infix(i) => i.hash(state),
        }
    }
}

impl Eq for Val {
    fn assert_receiver_is_total_eq(&self) {}
}

pub type Bool = bool;

pub type Int = num::Integer;

pub type Float = num::Float;

pub type String = std::string::String;

pub type Bytes = Vec<u8>;

pub type List = Vec<Val>;

pub type Map = HashMap<Val, Val>;

#[derive(PartialEq, Eq, Hash)]
pub struct Ltree {
    pub root: Val,
    pub leaves: List,
}

impl Ltree {
    pub fn new(root: Val, leaves: List) -> Ltree {
        Ltree { root, leaves }
    }
}

#[derive(PartialEq, Eq)]
pub struct Mtree {
    pub root: Val,
    pub leaves: Map,
}

impl Mtree {
    pub fn new(root: Val, leaves: Map) -> Mtree {
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

#[derive(PartialEq, Eq, Hash)]
pub struct Infix {
    pub infix: Val,
    pub left: Val,
    pub right: Val,
}
