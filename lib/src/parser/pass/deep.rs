use std::vec::IntoIter;

use super::flat::FlatNode;
use super::AtomNode;
use crate::parser::{ParseError, ParseResult};

pub const SEPERATOR: &str = ",";
pub const MAP_KV_SEPERATOR: &str = ":";
pub const LIST_LEFT: &str = "(";
pub const LIST_RIGHT: &str = ")";
pub const MAP_LEFT: &str = "{";
pub const MAP_RIGHT: &str = "}";
pub const INFIX_LEFT: &str = "[";
pub const INFIX_RIGHT: &str = "]";

pub enum DeepNode {
    Symbol(String),
    Atom(AtomNode),
    List(Vec<DeepNodes>),
    Map(Vec<(DeepNodes, DeepNodes)>),
    Infix(DeepNodes),
    Top(DeepNodes),
}

pub type DeepNodes = Vec<DeepNode>;

enum DeepFlag {
    None,
    List,
    Map,
    Infix,
}

pub fn parse(flat_nodes: Vec<FlatNode>) -> ParseResult<DeepNodes> {
    let mut iter = flat_nodes.into_iter();
    let deep_node = parse_one(&mut iter, DeepFlag::None)?;
    match deep_node {
        DeepNode::Top(nodes) => Ok(nodes),
        _ => Ok(vec![deep_node]),
    }
}

enum Item {
    Node(DeepNode),
    Seperator,
    MapKvSeperator,
}

fn parse_one(iter: &mut IntoIter<FlatNode>, flag: DeepFlag) -> ParseResult<DeepNode> {
    let mut items = Vec::new();
    while let Some(current) = iter.next() {
        let deep_node: Item = match current {
            FlatNode::Atom(a) => Item::Node(DeepNode::Atom(a)),
            FlatNode::Symbol(s) => match s.as_str() {
                LIST_LEFT => Item::Node(parse_one(iter, DeepFlag::List)?),
                LIST_RIGHT => {
                    return match flag {
                        DeepFlag::List => parse_list(items),
                        _ => ParseError::err(format!("unexpected {}", LIST_RIGHT)),
                    }
                }
                MAP_LEFT => Item::Node(parse_one(iter, DeepFlag::Map)?),
                MAP_RIGHT => {
                    return match flag {
                        DeepFlag::Map => parse_map(items),
                        _ => ParseError::err(format!("unexpected {}", MAP_RIGHT)),
                    }
                }
                INFIX_LEFT => Item::Node(parse_one(iter, DeepFlag::Infix)?),
                INFIX_RIGHT => {
                    return match flag {
                        DeepFlag::Infix => parse_infix(items),
                        _ => ParseError::err(format!("unexpected {}", INFIX_RIGHT)),
                    }
                }
                SEPERATOR => Item::Seperator,
                MAP_KV_SEPERATOR => Item::MapKvSeperator,
                _ => Item::Node(DeepNode::Symbol(s)),
            },
        };
        items.push(deep_node);
    }

    match flag {
        DeepFlag::None => parse_top(items),
        DeepFlag::List => ParseError::err("unexpected end of list".to_owned()),
        DeepFlag::Map => ParseError::err("unexpected end of map".to_owned()),
        DeepFlag::Infix => ParseError::err("unexpected end of infix".to_owned()),
    }
}

fn parse_list(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut list = Vec::new();
    let mut value = Vec::new();
    for item in items {
        match item {
            Item::Node(node) => {
                value.push(node);
            }
            Item::Seperator => {
                if value.is_empty() {
                    return ParseError::err(format!("unexpected {}", SEPERATOR));
                } else {
                    list.push(value);
                    value = Vec::new();
                }
            }
            Item::MapKvSeperator => {
                return ParseError::err(format!("unexpected {} in list", MAP_KV_SEPERATOR))
            }
        }
    }
    if !value.is_empty() {
        list.push(value);
    }
    Ok(DeepNode::List(list))
}

fn parse_map(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut map = Vec::new();
    let mut key = Vec::new();
    let mut value = Vec::new();
    let mut is_key = true;
    for item in items {
        if is_key {
            match item {
                Item::Node(node) => {
                    key.push(node);
                }
                Item::Seperator => return ParseError::err(format!("unexpected {}", SEPERATOR)),
                Item::MapKvSeperator => {
                    if key.is_empty() {
                        return ParseError::err(format!("unexpected {}", SEPERATOR));
                    } else {
                        is_key = false;
                    }
                }
            }
        } else {
            match item {
                Item::Node(node) => {
                    value.push(node);
                }
                Item::Seperator => {
                    if value.is_empty() {
                        return ParseError::err(format!("unexpected {}", SEPERATOR));
                    } else {
                        map.push((key, value));
                        key = Vec::new();
                        value = Vec::new();
                        is_key = true;
                    }
                }
                Item::MapKvSeperator => {
                    return ParseError::err(format!("unexpected {}", SEPERATOR));
                }
            }
        }
    }

    if key.is_empty() {
        Ok(DeepNode::Map(map))
    } else {
        map.push((key, value));
        Ok(DeepNode::Map(map))
    }
}

fn parse_infix(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut list = Vec::with_capacity(items.len());
    for item in items {
        match item {
            Item::Node(node) => {
                list.push(node);
            }
            Item::Seperator => {
                return ParseError::err(format!("unexpected {} in infix", SEPERATOR));
            }
            Item::MapKvSeperator => {
                return ParseError::err(format!("unexpected {} in infix", MAP_KV_SEPERATOR))
            }
        }
    }
    Ok(DeepNode::Infix(list))
}

fn parse_top(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut list = Vec::with_capacity(items.len());
    for item in items {
        match item {
            Item::Node(node) => {
                list.push(node);
            }
            Item::Seperator => {
                return ParseError::err(format!("unexpected {}", SEPERATOR));
            }
            Item::MapKvSeperator => {
                return ParseError::err(format!("unexpected {}", MAP_KV_SEPERATOR))
            }
        }
    }
    Ok(DeepNode::Top(list))
}
