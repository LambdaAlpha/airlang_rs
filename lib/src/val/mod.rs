use std::collections::HashMap;

pub enum Val {
    Bytes(Box<Bytes>),
    List(Box<List>),
    Map(Box<Map>),
    Ltree(Box<Ltree>),
    Mtree(Box<Mtree>),
}

pub type Bytes = Vec<u8>;

pub type List = Vec<Val>;

pub type Map = HashMap<Val, Val>;

pub struct Ltree {
    pub root: Val,
    pub leaves: List,
}

pub struct Mtree {
    pub root: Val,
    pub leaves: Map,
}
