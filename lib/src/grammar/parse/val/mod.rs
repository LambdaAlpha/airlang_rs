use crate::grammar::ParseResult;
use crate::val::{List, Map};
use crate::{grammar::ParseError, val::Val};

use super::infix::{InfixNode, InfixNodes};

pub(crate) fn parse(infix_nodes: InfixNodes) -> ParseResult<Val> {
    parse_expect_one(infix_nodes)
}

fn parse_one(node: InfixNode) -> ParseResult<Val> {
    match node {
        InfixNode::Atom(a) => match a {
            super::AtomNode::Bool(b) => Ok(Val::from(b)),
            super::AtomNode::Int(i) => Ok(Val::from(i)),
            super::AtomNode::Float(f) => Ok(Val::from(f)),
            super::AtomNode::Bytes(b) => Ok(Val::from(b)),
            super::AtomNode::String(s) => Ok(Val::from(s)),
            super::AtomNode::Letter(s) => Ok(Val::letter(s)),
        },
        InfixNode::Symbol(s) => Ok(Val::symbol(s)),
        InfixNode::List(l) => parse_list(l),
        InfixNode::Map(m) => parse_map(m),
        InfixNode::Itree(left, mid, right) => parse_itree(left, mid, right),
        InfixNode::Ltree(root, leaves) => parse_ltree(root, leaves),
        InfixNode::Mtree(root, leaves) => parse_mtree(root, leaves),
    }
}

fn parse_expect_one(nodes: InfixNodes) -> ParseResult<Val> {
    if nodes.len() == 1 {
        Ok(parse_one(nodes.into_iter().next().unwrap())?)
    } else {
        ParseError::err("expect exactly one value".to_owned())
    }
}

fn parse_list(nodes: Vec<InfixNodes>) -> ParseResult<Val> {
    let mut list = List::with_capacity(nodes.len());
    for node in nodes {
        list.push(parse_expect_one(node)?);
    }
    Ok(Val::from(list))
}
fn parse_map(nodes: Vec<(InfixNodes, InfixNodes)>) -> ParseResult<Val> {
    let mut map = Map::default();
    for node in nodes {
        map.insert(parse_expect_one(node.0)?, parse_expect_one(node.1)?);
    }
    Ok(Val::from(map))
}

fn parse_itree(
    left: Box<InfixNode>,
    mid: Box<InfixNode>,
    right: Box<InfixNode>,
) -> ParseResult<Val> {
    let left = parse_one(*left)?;
    let mid = parse_one(*mid)?;
    let right = parse_one(*right)?;
    Ok(Val::infix(left, mid, right))
}

fn parse_ltree(root: InfixNodes, leaves: Vec<InfixNodes>) -> ParseResult<Val> {
    let root = parse_expect_one(root)?;
    let leaves = parse_list(leaves)?;
    match leaves {
        Val::List(l) => Ok(Val::ltree(root, *l)),
        _ => unreachable!(),
    }
}

fn parse_mtree(root: InfixNodes, leaves: Vec<(InfixNodes, InfixNodes)>) -> ParseResult<Val> {
    let root = parse_expect_one(root)?;
    let leaves = parse_map(leaves)?;
    match leaves {
        Val::Map(m) => Ok(Val::mtree(root, *m)),
        _ => unreachable!(),
    }
}
