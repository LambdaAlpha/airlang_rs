use crate::grammar::{
    parse::infix::{
        InfixNode,
        InfixNodes,
    },
    repr::{
        List,
        Map,
        Repr,
    },
    ParseError,
    ParseResult,
};

pub(crate) fn parse(infix_nodes: InfixNodes) -> ParseResult<Repr> {
    parse_expect_one(infix_nodes)
}

fn parse_one(node: InfixNode) -> ParseResult<Repr> {
    match node {
        InfixNode::Atom(a) => match a {
            super::AtomNode::Unit => Ok(Repr::Unit),
            super::AtomNode::Bool(b) => Ok(Repr::bool(b)),
            super::AtomNode::Int(i) => Ok(Repr::int(i)),
            super::AtomNode::Float(f) => Ok(Repr::float(f)),
            super::AtomNode::Bytes(b) => Ok(Repr::bytes(b)),
            super::AtomNode::String(s) => Ok(Repr::string(s)),
            super::AtomNode::Letter(s) => Ok(Repr::letter(s)),
        },
        InfixNode::Symbol(s) => Ok(Repr::symbol(s)),
        InfixNode::List(l) => parse_list(l),
        InfixNode::Map(m) => parse_map(m),
        InfixNode::Itree(left, mid, right) => parse_itree(left, mid, right),
        InfixNode::Ltree(root, leaves) => parse_ltree(root, leaves),
        InfixNode::Mtree(root, leaves) => parse_mtree(root, leaves),
    }
}

fn parse_expect_one(nodes: InfixNodes) -> ParseResult<Repr> {
    if nodes.len() == 1 {
        Ok(parse_one(nodes.into_iter().next().unwrap())?)
    } else {
        ParseError::err("expect exactly one repr".to_owned())
    }
}

fn parse_list(nodes: Vec<InfixNodes>) -> ParseResult<Repr> {
    let mut list = List::with_capacity(nodes.len());
    for node in nodes {
        list.push(parse_expect_one(node)?);
    }
    Ok(Repr::list(list))
}

fn parse_map(nodes: Vec<(InfixNodes, InfixNodes)>) -> ParseResult<Repr> {
    let mut map = Map::with_capacity(nodes.len());
    for node in nodes {
        map.push((parse_expect_one(node.0)?, parse_expect_one(node.1)?));
    }
    Ok(Repr::map(map))
}

fn parse_itree(
    left: Box<InfixNode>,
    mid: Box<InfixNode>,
    right: Box<InfixNode>,
) -> ParseResult<Repr> {
    let left = parse_one(*left)?;
    let mid = parse_one(*mid)?;
    let right = parse_one(*right)?;
    Ok(Repr::infix(left, mid, right))
}

fn parse_ltree(root: InfixNodes, leaves: Vec<InfixNodes>) -> ParseResult<Repr> {
    let root = parse_expect_one(root)?;
    let leaves = parse_list(leaves)?;
    match leaves {
        Repr::List(l) => Ok(Repr::ltree(root, *l)),
        _ => unreachable!(),
    }
}

fn parse_mtree(root: InfixNodes, leaves: Vec<(InfixNodes, InfixNodes)>) -> ParseResult<Repr> {
    let root = parse_expect_one(root)?;
    let leaves = parse_map(leaves)?;
    match leaves {
        Repr::Map(m) => Ok(Repr::mtree(root, *m)),
        _ => unreachable!(),
    }
}
