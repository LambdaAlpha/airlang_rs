use {
    super::{
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR_SEPARATOR,
        SEPARATOR,
        WRAP_LEFT,
        WRAP_RIGHT,
    },
    crate::{
        grammar::PRESERVE_PREFIX,
        repr::{
            CallRepr,
            ListRepr,
            MapRepr,
            Repr,
        },
        types::{
            Bytes,
            Float,
            Int,
        },
        utils,
        Repr::Pair,
    },
    Repr::{
        Call,
        List,
        Map,
    },
};

pub(crate) const INDENT: &str = "  ";

#[allow(dead_code)]
pub(crate) fn stringify_compat(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: SEPARATOR.to_string(),
        pair_separator: PAIR_SEPARATOR.to_string(),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

#[allow(dead_code)]
pub(crate) fn stringify_comfort(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: format!("{} ", SEPARATOR),
        pair_separator: format!("{} ", PAIR_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

#[allow(dead_code)]
pub(crate) fn stringify_pretty(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyFormat {
        indent: INDENT.to_owned(),
        before_first: "\n".to_owned(),
        after_last: format!("{}\n", SEPARATOR),
        separator: format!("{}\n", SEPARATOR),
        pair_separator: format!("{} ", PAIR_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

pub(crate) struct StringifyFormat {
    pub(crate) indent: String,
    pub(crate) before_first: String,
    pub(crate) after_last: String,
    pub(crate) separator: String,
    pub(crate) pair_separator: String,
    pub(crate) left_padding: String,
    pub(crate) right_padding: String,
}

pub(crate) fn stringify(repr: &Repr, s: &mut String, format: &StringifyFormat, indent: usize) {
    match repr {
        Repr::Unit(_) => stringify_unit(s),
        Repr::Bool(b) => stringify_bool(b.bool(), s),
        Repr::Int(i) => stringify_int(i, s),
        Repr::Float(f) => stringify_float(f, s),
        Repr::Bytes(bytes) => stringify_bytes(bytes, s),
        Repr::String(str) => stringify_string(str, s),
        Repr::Letter(str) => stringify_letter(str, s),
        Repr::Symbol(str) => stringify_symbol(str, s),
        Repr::Pair(p) => stringify_pair(&p.first, &p.second, s, format, indent),
        Call(c) => stringify_call(c, s, format, indent),
        List(list) => stringify_list(list, s, format, indent),
        Map(map) => stringify_map(map, s, format, indent),
    }
}

fn stringify_unit(s: &mut String) {
    s.push(PRESERVE_PREFIX);
    s.push_str("u")
}

fn stringify_bool(b: bool, s: &mut String) {
    s.push(PRESERVE_PREFIX);
    s.push_str(if b { "t" } else { "f" })
}

fn stringify_int(i: &Int, s: &mut String) {
    s.push_str(&i.to_string())
}

fn stringify_float(f: &Float, s: &mut String) {
    s.push_str(&f.to_string())
}

fn stringify_bytes(bytes: &Bytes, s: &mut String) {
    s.push_str("1x");
    utils::conversion::u8_array_to_hex_string_mut(bytes.as_ref(), s);
}

fn stringify_string(str: &String, s: &mut String) {
    s.push('"');
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\".to_owned(),
            '\n' => "\\n".to_owned(),
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            '"' => "\\\"".to_owned(),
            _ => c.to_string(),
        };
        s.push_str(&escaped);
    }
    s.push('"');
}

fn stringify_letter(str: &String, s: &mut String) {
    s.push_str(str)
}

fn stringify_symbol(str: &String, s: &mut String) {
    s.push_str(str)
}

fn stringify_pair(
    first: &Repr,
    second: &Repr,
    s: &mut String,
    format: &StringifyFormat,
    indent: usize,
) {
    match first {
        Call(c) => {
            match &c.arg {
                List(_) | Map(_) => {
                    // a():b
                    // a{}:b
                    stringify(first, s, format, indent);
                }
                _ => {
                    // [a b]:c
                    // [a b c]:d
                    stringify_wrapped(first, s, format, indent);
                }
            }
        }
        _ => {
            // a:b
            // a:b:c
            // ():a
            // {}:a
            stringify(first, s, format, indent);
        }
    }
    s.push_str(&format.pair_separator);
    match second {
        Call(_) | Pair(_) => {
            // a:[b()]
            // a:[b{}]
            // a:[b c d]
            // a:[b c]
            // a:[b:c]
            stringify_wrapped(second, s, format, indent);
        }
        _ => {
            // a:b
            // a:()
            // a:{}
            stringify(second, s, format, indent);
        }
    }
}

fn stringify_call(call: &CallRepr, s: &mut String, format: &StringifyFormat, indent: usize) {
    match &call.arg {
        Pair(p) => {
            match &p.first {
                Call(c) => {
                    match &c.arg {
                        Pair(_) | List(_) | Map(_) => {
                            // a() b c
                            // a{} b c
                            // a b c d e
                            stringify(&p.first, s, format, indent)
                        }
                        _ => {
                            // [a b] c d
                            stringify_wrapped(&p.first, s, format, indent)
                        }
                    }
                }
                _ => {
                    // a:b c d
                    // () a b
                    // {} a b
                    // a b c
                    stringify(&p.first, s, format, indent)
                }
            }
            s.push(' ');
            match &call.func {
                List(_) | Map(_) => {
                    // a [()] b
                    // a [{}] b
                    stringify_wrapped(&call.func, s, format, indent)
                }
                Call(c) => {
                    match &c.arg {
                        List(_) | Map(_) => {
                            // a b() c
                            // a b{} c
                            stringify(&call.func, s, format, indent)
                        }
                        _ => {
                            // a [b c d] e
                            // a [b c] d
                            stringify_wrapped(&call.func, s, format, indent)
                        }
                    }
                }
                _ => {
                    // a b c
                    // a b:c d
                    stringify(&call.func, s, format, indent)
                }
            }
            s.push(' ');
            match &p.second {
                Call(c) => {
                    match &c.arg {
                        List(_) | Map(_) => {
                            // a b c()
                            // a b c{}
                            stringify(&p.second, s, format, indent)
                        }
                        _ => {
                            // a b [c d e]
                            // a b [c d]
                            stringify_wrapped(&p.second, s, format, indent)
                        }
                    }
                }
                List(_) | Map(_) => {
                    // a b [()]
                    // a b [{}]
                    stringify_wrapped(&p.second, s, format, indent)
                }
                _ => {
                    // a b c
                    // a b c:d
                    stringify(&p.second, s, format, indent)
                }
            }
        }
        _ => {
            match &call.func {
                Call(c) => {
                    match &c.arg {
                        List(_) | Map(_) => {
                            // a() b
                            // a{} b
                            stringify(&call.func, s, format, indent)
                        }
                        _ => {
                            // [a b c] d
                            // [a b] c
                            stringify_wrapped(&call.func, s, format, indent)
                        }
                    }
                }
                _ => {
                    // a b
                    // () a
                    // {} a
                    // a:b c
                    stringify(&call.func, s, format, indent)
                }
            }
            match &call.arg {
                List(_) | Map(_) => {
                    // a()
                    // a{}
                    stringify(&call.arg, s, format, indent)
                }
                Call(c) => {
                    match &c.arg {
                        List(_) | Map(_) => {
                            // a b()
                            // a b{}
                            s.push(' ');
                            stringify(&call.arg, s, format, indent)
                        }
                        _ => {
                            // a [b c]
                            // a [b c d]
                            s.push(' ');
                            stringify_wrapped(&call.arg, s, format, indent)
                        }
                    }
                }
                _ => {
                    // a b
                    // a b:c
                    s.push(' ');
                    stringify(&call.arg, s, format, indent)
                }
            }
        }
    }
}

fn stringify_wrapped(repr: &Repr, s: &mut String, format: &StringifyFormat, indent: usize) {
    s.push(WRAP_LEFT);
    s.push_str(&format.left_padding);
    stringify(repr, s, format, indent);
    s.push_str(&format.right_padding);
    s.push(WRAP_RIGHT);
}

fn stringify_list(list: &ListRepr, s: &mut String, format: &StringifyFormat, indent: usize) {
    s.push(LIST_LEFT);
    if list.is_empty() {
        s.push(LIST_RIGHT);
        return;
    }

    if list.len() == 1 {
        s.push_str(&format.left_padding);
        stringify(list.first().unwrap(), s, format, indent);
        s.push_str(&format.right_padding);
        s.push(LIST_RIGHT);
        return;
    }

    s.push_str(&format.before_first);
    for repr in list.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        stringify(repr, s, format, indent + 1);
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(LIST_RIGHT);
}

fn stringify_map(map: &MapRepr, s: &mut String, format: &StringifyFormat, indent: usize) {
    s.push(MAP_LEFT);
    if map.is_empty() {
        s.push(MAP_RIGHT);
        return;
    }

    if map.len() == 1 {
        let pair = map.iter().next().unwrap();
        s.push_str(&format.left_padding);
        stringify_pair(pair.0, pair.1, s, format, indent);
        s.push_str(&format.right_padding);
        s.push(MAP_RIGHT);
        return;
    }

    s.push_str(&format.before_first);
    for pair in map.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        stringify_pair(pair.0, pair.1, s, format, indent + 1);
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(MAP_RIGHT);
}
