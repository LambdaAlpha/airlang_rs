use crate::grammar::{
    parse::{
        AtomNode,
        prefix::{
            PrefixNode,
            PrefixNodes,
        },
    },
    ParseResult,
};

#[derive(Debug)]
pub(crate) enum PostfixNode {
    Atom(AtomNode),
    Symbol(String),
    Wrap(PostfixNodes),
    List(Vec<PostfixNodes>),
    Map(Vec<(PostfixNodes, PostfixNodes)>),
    Ltree(PostfixNodes, Vec<PostfixNodes>),
    Mtree(PostfixNodes, Vec<(PostfixNodes, PostfixNodes)>),
}

pub(crate) type PostfixNodes = Vec<PostfixNode>;

pub(crate) fn parse(prefix_nodes: PrefixNodes) -> ParseResult<PostfixNodes> {
    let mut postfix_nodes = Vec::new();
    let mut iter = prefix_nodes.into_iter();
    let mut op_left = None;
    while let Some(prefix_node) = iter.next() {
        let postfix_node = match prefix_node {
            PrefixNode::Atom(a) => PostfixNode::Atom(a),
            PrefixNode::Symbol(s) => PostfixNode::Symbol(s),
            PrefixNode::Wrap(i) => PostfixNode::Wrap(parse(i)?),
            PrefixNode::List(l) => PostfixNode::List(parse_list(l)?),
            PrefixNode::Map(m) => PostfixNode::Map(parse_map(m)?),
        };
        match postfix_node {
            PostfixNode::List(list) => match op_left {
                Some(left_node) => {
                    op_left = Some(PostfixNode::Ltree(vec![left_node], list));
                }
                None => op_left = Some(PostfixNode::List(list)),
            },
            PostfixNode::Map(map) => match op_left {
                Some(left_node) => {
                    op_left = Some(PostfixNode::Mtree(vec![left_node], map));
                }
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

fn parse_list(prefix_nodes: Vec<PrefixNodes>) -> ParseResult<Vec<PostfixNodes>> {
    let mut list = Vec::with_capacity(prefix_nodes.len());
    for node in prefix_nodes {
        list.push(parse(node)?)
    }
    Ok(list)
}

fn parse_map(
    prefix_nodes: Vec<(PrefixNodes, PrefixNodes)>,
) -> ParseResult<Vec<(PostfixNodes, PostfixNodes)>> {
    let mut map = Vec::with_capacity(prefix_nodes.len());
    for pair in prefix_nodes {
        map.push((parse(pair.0)?, parse(pair.1)?))
    }
    Ok(map)
}
