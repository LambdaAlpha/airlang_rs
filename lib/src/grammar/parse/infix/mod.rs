use crate::grammar::parse::postfix::PostfixNode;
use crate::grammar::{ParseError, ParseResult};

use super::postfix::PostfixNodes;
use super::AtomNode;

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) enum InfixNode {
    Symbol(String),
    Atom(AtomNode),
    List(Vec<InfixNodes>),
    Map(Vec<(InfixNodes, InfixNodes)>),
    Itree(Box<InfixNode>, Box<InfixNode>, Box<InfixNode>),
    Ltree(InfixNodes, Vec<InfixNodes>),
    Mtree(InfixNodes, Vec<(InfixNodes, InfixNodes)>),
}

pub(crate) type InfixNodes = Vec<InfixNode>;

pub(crate) fn parse(postfix_nodes: PostfixNodes) -> ParseResult<InfixNodes> {
    parse_infix(postfix_nodes)
}

fn parse_infix(postfix_nodes: PostfixNodes) -> ParseResult<InfixNodes> {
    let mut infix_nodes = Vec::new();
    let mut iter = postfix_nodes.into_iter();
    let mut op_left = None;
    let mut op_mid = None;
    while let Some(postfix_node) = iter.next() {
        let infix_node = match postfix_node {
            PostfixNode::Atom(a) => InfixNode::Atom(a),
            PostfixNode::Symbol(s) => InfixNode::Symbol(s),
            PostfixNode::Wrap(n) => {
                let result = parse_infix(n)?;
                result.into_iter().next().unwrap()
            }
            PostfixNode::List(l) => InfixNode::List(parse_list(l)?),
            PostfixNode::Map(m) => InfixNode::Map(parse_map(m)?),
            PostfixNode::Ltree(root, leaves) => {
                let root = parse_infix(root)?;
                let list = parse_list(leaves)?;
                InfixNode::Ltree(root, list)
            }
            PostfixNode::Mtree(root, leaves) => {
                let root = parse_infix(root)?;
                let map = parse_map(leaves)?;
                InfixNode::Mtree(root, map)
            }
        };
        match op_left {
            Some(left) => match op_mid {
                Some(mid) => {
                    op_left = Some(InfixNode::Itree(
                        Box::new(left),
                        Box::new(mid),
                        Box::new(infix_node),
                    ));
                    op_mid = None;
                }
                None => {
                    op_mid = Some(infix_node);
                    // to pass borrow check
                    op_left = Some(left);
                }
            },
            None => {
                op_left = Some(infix_node);
            }
        }
    }
    if op_mid.is_some() {
        return ParseError::err("expect a right value of an infix value".to_owned());
    }
    if op_left.is_some() {
        infix_nodes.push(op_left.unwrap())
    }
    Ok(infix_nodes)
}

fn parse_list(prefix_nodes: Vec<PostfixNodes>) -> ParseResult<Vec<InfixNodes>> {
    let mut list = Vec::with_capacity(prefix_nodes.len());
    for node in prefix_nodes {
        list.push(parse(node)?)
    }
    Ok(list)
}

fn parse_map(
    prefix_nodes: Vec<(PostfixNodes, PostfixNodes)>,
) -> ParseResult<Vec<(InfixNodes, InfixNodes)>> {
    let mut map = Vec::with_capacity(prefix_nodes.len());
    for pair in prefix_nodes {
        map.push((parse(pair.0)?, parse(pair.1)?))
    }
    Ok(map)
}
