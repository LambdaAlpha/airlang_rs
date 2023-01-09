use {
    crate::grammar::{
        parse::{
            lexer::FlatNode,
            AtomNode,
        },
        ParseError,
        ParseResult,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_KV_SEPARATOR,
        MAP_LEFT,
        MAP_RIGHT,
        SEPARATOR,
        WRAP_LEFT,
        WRAP_RIGHT,
    },
    std::vec::IntoIter,
};

#[derive(Debug)]
pub(crate) enum DeepNode {
    Symbol(String),
    Atom(AtomNode),
    List(Vec<DeepNodes>),
    Map(Vec<(DeepNodes, DeepNodes)>),
    Wrap(DeepNodes),
}

pub(crate) type DeepNodes = Vec<DeepNode>;

enum DeepFlag {
    None,
    List,
    Map,
    Wrap,
}

pub(crate) fn parse(flat_nodes: Vec<FlatNode>) -> ParseResult<DeepNodes> {
    let mut iter = flat_nodes.into_iter();
    let deep_node = parse_one(&mut iter, DeepFlag::None)?;
    match deep_node {
        DeepNode::Wrap(nodes) => Ok(nodes),
        _ => Ok(vec![deep_node]),
    }
}

enum Item {
    Node(DeepNode),
    Separator,
    MapKvSeparator,
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
                    };
                }
                MAP_LEFT => Item::Node(parse_one(iter, DeepFlag::Map)?),
                MAP_RIGHT => {
                    return match flag {
                        DeepFlag::Map => parse_map(items),
                        _ => ParseError::err(format!("unexpected {}", MAP_RIGHT)),
                    };
                }
                WRAP_LEFT => Item::Node(parse_one(iter, DeepFlag::Wrap)?),
                WRAP_RIGHT => {
                    return match flag {
                        DeepFlag::Wrap => parse_wrap(items),
                        _ => ParseError::err(format!("unexpected {}", WRAP_RIGHT)),
                    };
                }
                SEPARATOR => Item::Separator,
                MAP_KV_SEPARATOR => Item::MapKvSeparator,
                _ => Item::Node(DeepNode::Symbol(s)),
            },
        };
        items.push(deep_node);
    }

    match flag {
        DeepFlag::None => parse_wrap(items),
        DeepFlag::List => ParseError::err("unexpected end of list".to_owned()),
        DeepFlag::Map => ParseError::err("unexpected end of map".to_owned()),
        DeepFlag::Wrap => ParseError::err("unexpected end of wrap".to_owned()),
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
            Item::Separator => {
                if value.is_empty() {
                    return ParseError::err(format!("unexpected {}", SEPARATOR));
                } else {
                    list.push(value);
                    value = Vec::new();
                }
            }
            Item::MapKvSeparator => {
                return ParseError::err(format!("unexpected {} in list", MAP_KV_SEPARATOR));
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
        match item {
            Item::Node(node) => {
                if is_key {
                    key.push(node);
                } else {
                    value.push(node);
                }
            }
            Item::Separator => {
                if key.is_empty() {
                    return ParseError::err(format!("unexpected {}", SEPARATOR));
                } else {
                    map.push((key, value));
                    key = Vec::new();
                    value = Vec::new();
                    is_key = true;
                }
            }
            Item::MapKvSeparator => {
                if !is_key || key.is_empty() {
                    return ParseError::err(format!("unexpected {}", SEPARATOR));
                } else {
                    is_key = false;
                }
            }
        }
    }

    if !key.is_empty() {
        map.push((key, value));
    }
    Ok(DeepNode::Map(map))
}

fn parse_wrap(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut list = Vec::with_capacity(items.len());
    for item in items {
        match item {
            Item::Node(node) => {
                list.push(node);
            }
            Item::Separator => {
                return ParseError::err(format!("unexpected {} in wrap", SEPARATOR));
            }
            Item::MapKvSeparator => {
                return ParseError::err(format!("unexpected {} in wrap", MAP_KV_SEPARATOR));
            }
        }
    }
    Ok(DeepNode::Wrap(list))
}
