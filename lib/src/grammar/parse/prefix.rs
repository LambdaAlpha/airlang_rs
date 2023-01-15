use crate::grammar::{
    parse::{
        deep::{
            DeepNode,
            DeepNodes,
            MapItem,
        },
        AtomNode,
    },
    ParseError,
    ParseResult,
    COMMENT_PREFIX,
};

#[derive(Debug)]
pub(crate) enum PrefixNode {
    Atom(AtomNode),
    Symbol(String),
    List(Vec<PrefixNodes>),
    Map(Vec<(PrefixNodes, PrefixNodes)>),
    Wrap(PrefixNodes),
}

pub(crate) type PrefixNodes = Vec<PrefixNode>;

pub(crate) fn parse(deep_nodes: DeepNodes) -> ParseResult<PrefixNodes> {
    let prefix_nodes = parse_prefix(deep_nodes)?;
    if prefix_nodes.is_empty() {
        ParseError::err("one value expected".to_owned())
    } else {
        Ok(prefix_nodes)
    }
}

fn parse_prefix(deep_nodes: DeepNodes) -> ParseResult<PrefixNodes> {
    let mut iter = deep_nodes.into_iter();
    let mut prefix_nodes = Vec::new();
    while let Some(deep_node) = iter.next() {
        let prefix_node = match deep_node {
            DeepNode::Atom(a) => PrefixNode::Atom(a),
            DeepNode::Symbol(s) => match s.as_str() {
                COMMENT_PREFIX => {
                    if iter.next().is_none() {
                        return ParseError::err("expect comment body".to_owned());
                    } else {
                        continue;
                    }
                }
                _ => PrefixNode::Symbol(s),
            },
            DeepNode::List(l, last) => parse_list(l, last)?,
            DeepNode::Map(m, last) => parse_map(m, last)?,
            DeepNode::Wrap(i) => {
                let nodes = parse_prefix(i)?;
                // drop comments
                if nodes.is_empty() {
                    continue;
                } else {
                    PrefixNode::Wrap(nodes)
                }
            }
        };
        prefix_nodes.push(prefix_node);
    }

    Ok(prefix_nodes)
}

fn parse_list(nodes: Vec<DeepNodes>, last: DeepNodes) -> ParseResult<PrefixNode> {
    let mut list = Vec::with_capacity(nodes.len());
    for node in nodes {
        let item = parse(node)?;
        list.push(item);
    }

    let prefix_nodes = parse_prefix(last)?;
    if !prefix_nodes.is_empty() {
        list.push(prefix_nodes);
    }

    Ok(PrefixNode::List(list))
}

fn parse_map(nodes: Vec<(DeepNodes, DeepNodes)>, last: MapItem) -> ParseResult<PrefixNode> {
    let mut map = Vec::with_capacity(nodes.len());
    for item in nodes {
        let key = parse(item.0)?;
        let value = parse(item.1)?;
        map.push((key, value));
    }

    match last {
        MapItem::Unit(deep_nodes) => {
            let prefix_nodes = parse_prefix(deep_nodes)?;
            if !prefix_nodes.is_empty() {
                return ParseError::err("key value pair expected".to_owned());
            }
        }
        MapItem::Pair(k, v) => {
            let key = parse(k)?;
            let value = parse(v)?;
            map.push((key, value));
        }
    }

    Ok(PrefixNode::Map(map))
}
