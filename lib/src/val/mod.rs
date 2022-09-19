use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::parser;

#[derive(PartialEq, Eq)]
pub enum Val {
    Bytes(Box<Bytes>),
    List(Box<List>),
    Map(Box<Map>),
    Ltree(Box<Ltree>),
    Mtree(Box<Mtree>),
}

impl Val {
    pub fn bytes(b: Bytes) -> Val {
        Val::Bytes(Box::new(b))
    }
    pub fn list(l: List) -> Val {
        Val::List(Box::new(l))
    }
    pub fn map(m: Map) -> Val {
        Val::Map(Box::new(m))
    }
    pub fn ltree(ltree: Ltree) -> Val {
        Val::Ltree(Box::new(ltree))
    }
    pub fn ltree1(root: Val, leaves: List) -> Val {
        Val::Ltree(Box::new(Ltree { root, leaves }))
    }
    pub fn mtree(mtree: Mtree) -> Val {
        Val::Mtree(Box::new(mtree))
    }
    pub fn mtree1(root: Val, leaves: Map) -> Val {
        Val::Mtree(Box::new(Mtree { root, leaves }))
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
            Val::Bytes(b) => b.hash(state),
            Val::List(l) => l.hash(state),
            Val::Map(m) => {
                for i in m.iter() {
                    i.hash(state);
                }
            }
            Val::Ltree(lt) => lt.hash(state),
            Val::Mtree(mt) => mt.hash(state),
        }
    }
}

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
