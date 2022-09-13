use super::prefix::PrefixNode;
use super::AtomNode;
use crate::parser::ParserError;

pub enum PostfixNode {
    Atom(AtomNode),
    Symbol(String),
    Infix(Vec<PostfixNode>),
    List(Vec<Vec<PostfixNode>>),
    Map(Vec<(Vec<PostfixNode>, Vec<PostfixNode>)>),
    Ltree(Vec<PostfixNode>, Vec<Vec<PostfixNode>>),
    Mtree(Vec<PostfixNode>, Vec<(Vec<PostfixNode>, Vec<PostfixNode>)>),
    Top(Vec<PostfixNode>),
}

pub fn parse(prefix_nodes: Vec<PrefixNode>) -> Result<Vec<PostfixNode>, ParserError> {
    let mut postfix_nodes = Vec::new();
    let mut iter = prefix_nodes.into_iter();
    let mut op_left = None;
    while let Some(prefix_node) = iter.next() {
        let postfix_node = match prefix_node {
            PrefixNode::Atom(a) => PostfixNode::Atom(a),
            PrefixNode::Symbol(s) => PostfixNode::Symbol(s),
            PrefixNode::Infix(i) => PostfixNode::Infix(parse(i)?),
            PrefixNode::List(l) => PostfixNode::List(parse_list(l)?),
            PrefixNode::Map(m) => PostfixNode::Map(parse_map(m)?),
            PrefixNode::Top(v) => PostfixNode::Top(parse(v)?),
        };
        match postfix_node {
            PostfixNode::List(list) => match op_left {
                Some(left_node) => match left_node {
                    PostfixNode::Symbol(_) => {
                        postfix_nodes.push(left_node);
                        op_left = Some(PostfixNode::List(list));
                    }
                    _ => op_left = Some(PostfixNode::Ltree(vec![left_node], list)),
                },
                None => op_left = Some(PostfixNode::List(list)),
            },
            PostfixNode::Map(map) => match op_left {
                Some(left_node) => match left_node {
                    PostfixNode::Symbol(_) => {
                        postfix_nodes.push(left_node);
                        op_left = Some(PostfixNode::Map(map));
                    }
                    _ => op_left = Some(PostfixNode::Mtree(vec![left_node], map)),
                },
                None => op_left = Some(PostfixNode::Map(map)),
            },
            _ => {
                if let Some(left_node) = op_left {
                    postfix_nodes.push(left_node);
                }
                op_left = Some(postfix_node);
            }
        }
    }
    if let Some(left_node) = op_left {
        postfix_nodes.push(left_node);
    }
    Ok(postfix_nodes)
}

fn parse_list(prefix_nodes: Vec<Vec<PrefixNode>>) -> Result<Vec<Vec<PostfixNode>>, ParserError> {
    let mut list = Vec::with_capacity(prefix_nodes.len());
    for node in prefix_nodes {
        list.push(parse(node)?)
    }
    Ok(list)
}

fn parse_map(
    prefix_nodes: Vec<(Vec<PrefixNode>, Vec<PrefixNode>)>,
) -> Result<Vec<(Vec<PostfixNode>, Vec<PostfixNode>)>, ParserError> {
    let mut map = Vec::with_capacity(prefix_nodes.len());
    for pair in prefix_nodes {
        map.push((parse(pair.0)?, parse(pair.1)?))
    }
    Ok(map)
}
