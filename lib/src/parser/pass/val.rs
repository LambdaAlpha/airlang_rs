use crate::num::{Float, Integer};

use crate::val::{List, Map};
use crate::{parser::ParserError, val::Val};

use super::infix::InfixNode;

pub fn parse(infix_nodes: Vec<InfixNode>) -> Result<Val, ParserError> {
    parse_expect_one(infix_nodes)
}

fn parse_one(node: InfixNode) -> Result<Val, ParserError> {
    match node {
        InfixNode::Atom(a) => match a {
            super::AtomNode::Bool(b) => Ok(bool(b)),
            super::AtomNode::Int(i) => Ok(int(&i)),
            super::AtomNode::Float(f) => Ok(float(&f)),
            super::AtomNode::Bytes(b) => Ok(Val::bytes(b)),
            super::AtomNode::String(s) => Ok(string(s)),
            super::AtomNode::Letter(s) => Ok(string(s)),
        },
        InfixNode::Symbol(s) => Ok(string(s)),
        InfixNode::List(l) => parse_list(l),
        InfixNode::Map(m) => parse_map(m),
        InfixNode::Itree(left, mid, right) => parse_itree(left, mid, right),
        InfixNode::Ltree(root, leaves) => parse_ltree(root, leaves),
        InfixNode::Mtree(root, leaves) => parse_mtree(root, leaves),
        InfixNode::Top(v) => parse_expect_one(v),
    }
}

fn parse_expect_one(nodes: Vec<InfixNode>) -> Result<Val, ParserError> {
    if nodes.len() == 1 {
        Ok(parse_one(nodes.into_iter().next().unwrap())?)
    } else {
        ParserError::err("expect exactly one value".to_owned())
    }
}

fn parse_list(nodes: Vec<Vec<InfixNode>>) -> Result<Val, ParserError> {
    let mut list = List::with_capacity(nodes.len());
    for node in nodes {
        list.push(parse_expect_one(node)?);
    }
    Ok(Val::list(list))
}
fn parse_map(nodes: Vec<(Vec<InfixNode>, Vec<InfixNode>)>) -> Result<Val, ParserError> {
    let mut map = Map::new();
    for node in nodes {
        map.insert(parse_expect_one(node.0)?, parse_expect_one(node.1)?);
    }
    Ok(Val::map(map))
}

fn parse_itree(
    left: Box<InfixNode>,
    mid: Box<InfixNode>,
    right: Box<InfixNode>,
) -> Result<Val, ParserError> {
    let left = parse_one(*left)?;
    let mid = parse_one(*mid)?;
    let right = parse_one(*right)?;
    Ok(Val::ltree1(mid, vec![left, right]))
}

fn parse_ltree(root: Vec<InfixNode>, leaves: Vec<Vec<InfixNode>>) -> Result<Val, ParserError> {
    let root = parse_expect_one(root)?;
    let leaves = parse_list(leaves)?;
    match leaves {
        Val::List(l) => Ok(Val::ltree1(root, *l)),
        _ => unreachable!(),
    }
}

fn parse_mtree(
    root: Vec<InfixNode>,
    leaves: Vec<(Vec<InfixNode>, Vec<InfixNode>)>,
) -> Result<Val, ParserError> {
    let root = parse_expect_one(root)?;
    let leaves = parse_map(leaves)?;
    match leaves {
        Val::Map(m) => Ok(Val::mtree1(root, *m)),
        _ => unreachable!(),
    }
}

pub fn bool(b: bool) -> Val {
    Val::bytes(vec![if b { 0xff } else { 0x00 }])
}

pub fn int(i: &Integer) -> Val {
    Val::bytes(i.to_digits(crate::num::integer::Order::Lsf))
}

pub fn float(f: &Float) -> Val {
    Val::bytes(f.to_string_radix(32, None).into_bytes())
}

pub fn str(s: &str) -> Val {
    Val::bytes(s.as_bytes().to_vec())
}

pub fn string(s: String) -> Val {
    Val::bytes(s.into_bytes())
}
