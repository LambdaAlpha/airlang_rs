use crate::parser::pass::postfix::PostfixNode;
use crate::parser::ParserError;

use super::AtomNode;

pub enum InfixNode {
    Symbol(String),
    Atom(AtomNode),
    List(Vec<Vec<InfixNode>>),
    Map(Vec<(Vec<InfixNode>, Vec<InfixNode>)>),
    Itree(Box<InfixNode>, Box<InfixNode>, Box<InfixNode>),
    Ltree(Vec<InfixNode>, Vec<Vec<InfixNode>>),
    Mtree(Vec<InfixNode>, Vec<(Vec<InfixNode>, Vec<InfixNode>)>),
    Top(Vec<InfixNode>),
}

pub fn parse(postfix_nodes: Vec<PostfixNode>) -> Result<Vec<InfixNode>, ParserError> {
    parse_infix(postfix_nodes, false)
}

fn parse_infix(
    postfix_nodes: Vec<PostfixNode>,
    is_infix_mode: bool,
) -> Result<Vec<InfixNode>, ParserError> {
    let mut infix_nodes = Vec::new();
    let mut iter = postfix_nodes.into_iter();
    let mut op_left = None;
    let mut op_mid = None;
    while let Some(postfix_node) = iter.next() {
        let infix_node = match postfix_node {
            PostfixNode::Atom(a) => InfixNode::Atom(a),
            PostfixNode::Symbol(s) => InfixNode::Symbol(s),
            PostfixNode::Infix(n) => {
                let result = parse_infix(n, true)?;
                let node = result.into_iter().next().unwrap();
                match node {
                    InfixNode::Symbol(s) => InfixNode::Atom(AtomNode::String(s)),
                    _ => node,
                }
            }
            PostfixNode::List(l) => InfixNode::List(parse_list(l)?),
            PostfixNode::Map(m) => InfixNode::Map(parse_map(m)?),
            PostfixNode::Ltree(root, leaves) => {
                let root = parse_infix(root, false)?;
                let list = parse_list(leaves)?;
                InfixNode::Ltree(root, list)
            }
            PostfixNode::Mtree(root, leaves) => {
                let root = parse_infix(root, false)?;
                let map = parse_map(leaves)?;
                InfixNode::Mtree(root, map)
            }
            PostfixNode::Top(v) => InfixNode::Top(parse_infix(v, false)?),
        };
        let is_symbol = matches!(infix_node, InfixNode::Symbol(_));
        match op_left {
            Some(left) => match op_mid {
                Some(mid) => {
                    if !is_infix_mode && is_symbol {
                        return ParserError::err("expect a left value but got a infix".to_owned());
                    }
                    op_left = Some(InfixNode::Itree(
                        Box::new(left),
                        Box::new(mid),
                        Box::new(infix_node),
                    ));
                    op_mid = None;
                }
                None => {
                    if is_infix_mode || is_symbol {
                        op_mid = Some(infix_node);
                        // to pass borrow check
                        op_left = Some(left);
                    } else {
                        infix_nodes.push(left);
                        op_left = Some(infix_node);
                    }
                }
            },
            None => {
                if !is_infix_mode && is_symbol {
                    return ParserError::err("expect a left value but got a infix".to_owned());
                }
                op_left = Some(infix_node);
            }
        }
    }
    if op_mid.is_some() {
        return ParserError::err("expect a right value of an infix value".to_owned());
    }
    if op_left.is_some() {
        infix_nodes.push(op_left.unwrap())
    }
    Ok(infix_nodes)
}

fn parse_list(prefix_nodes: Vec<Vec<PostfixNode>>) -> Result<Vec<Vec<InfixNode>>, ParserError> {
    let mut list = Vec::with_capacity(prefix_nodes.len());
    for node in prefix_nodes {
        list.push(parse(node)?)
    }
    Ok(list)
}

fn parse_map(
    prefix_nodes: Vec<(Vec<PostfixNode>, Vec<PostfixNode>)>,
) -> Result<Vec<(Vec<InfixNode>, Vec<InfixNode>)>, ParserError> {
    let mut map = Vec::with_capacity(prefix_nodes.len());
    for pair in prefix_nodes {
        map.push((parse(pair.0)?, parse(pair.1)?))
    }
    Ok(map)
}
