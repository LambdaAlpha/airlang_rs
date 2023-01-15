use {
    crate::grammar::{
        parse::{
            lexer::{
                FlatNode,
                FlatNodes,
            },
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
    // split last item
    List(Vec<DeepNodes>, DeepNodes),
    // split last item
    Map(Vec<(DeepNodes, DeepNodes)>, MapItem),
    Wrap(DeepNodes),
}

#[derive(Debug)]
pub(crate) enum MapItem {
    Unit(DeepNodes),
    Pair(DeepNodes, DeepNodes),
}

pub(crate) type DeepNodes = Vec<DeepNode>;

enum DeepFlag {
    None,
    List,
    Map,
    Wrap,
}

pub(crate) fn parse(flat_nodes: FlatNodes) -> ParseResult<DeepNodes> {
    let mut iter = flat_nodes.into_iter();
    let mut deep_nodes = Vec::new();
    let mut has_next = true;
    while has_next {
        let parse_info = parse_one(&mut iter, DeepFlag::None)?;
        has_next = parse_info.has_next;
        deep_nodes.push(parse_info.node);
    }
    Ok(deep_nodes)
}

enum Item {
    Node(DeepNode),
    Separator,
    MapKvSeparator,
}

struct ParseInfo {
    node: DeepNode,
    has_next: bool,
}

fn parse_one_node(iter: &mut IntoIter<FlatNode>, flag: DeepFlag) -> ParseResult<DeepNode> {
    Ok(parse_one(iter, flag)?.node)
}

fn parse_one(iter: &mut IntoIter<FlatNode>, flag: DeepFlag) -> ParseResult<ParseInfo> {
    let mut items = Vec::new();
    while let Some(current) = iter.next() {
        let deep_node: Item = match current {
            FlatNode::Atom(a) => Item::Node(DeepNode::Atom(a)),
            FlatNode::Symbol(s) => match s.as_str() {
                LIST_LEFT => Item::Node(parse_one_node(iter, DeepFlag::List)?),
                LIST_RIGHT => {
                    return match flag {
                        DeepFlag::List => Ok(ParseInfo {
                            node: parse_list(items)?,
                            has_next: true,
                        }),
                        _ => ParseError::err(format!("unexpected {}", LIST_RIGHT)),
                    };
                }
                MAP_LEFT => Item::Node(parse_one_node(iter, DeepFlag::Map)?),
                MAP_RIGHT => {
                    return match flag {
                        DeepFlag::Map => Ok(ParseInfo {
                            node: parse_map(items)?,
                            has_next: true,
                        }),
                        _ => ParseError::err(format!("unexpected {}", MAP_RIGHT)),
                    };
                }
                WRAP_LEFT => Item::Node(parse_one_node(iter, DeepFlag::Wrap)?),
                WRAP_RIGHT => {
                    return match flag {
                        DeepFlag::Wrap => Ok(ParseInfo {
                            node: parse_wrap(items)?,
                            has_next: true,
                        }),
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
        DeepFlag::None => Ok(ParseInfo {
            node: parse_wrap(items)?,
            has_next: false,
        }),
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
                list.push(value);
                value = Vec::new();
            }
            Item::MapKvSeparator => {
                return ParseError::err(format!("unexpected {} in list", MAP_KV_SEPARATOR));
            }
        }
    }
    Ok(DeepNode::List(list, value))
}

fn parse_map(items: Vec<Item>) -> ParseResult<DeepNode> {
    let mut map = Vec::new();
    let mut key = Vec::new();
    let mut value = Vec::new();
    let mut is_key = true;
    let mut is_pair = false;
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
                if is_pair {
                    map.push((key, value));
                } else {
                    return ParseError::err(format!("unexpected {}", SEPARATOR));
                }

                key = Vec::new();
                value = Vec::new();
                is_key = true;
                is_pair = false;
            }
            Item::MapKvSeparator => {
                is_key = false;
                is_pair = true;
            }
        }
    }

    let last = if is_pair {
        MapItem::Pair(key, value)
    } else {
        MapItem::Unit(key)
    };
    Ok(DeepNode::Map(map, last))
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
