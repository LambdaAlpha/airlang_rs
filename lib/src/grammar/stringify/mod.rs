use std::fmt::Write;

use crate::{
    grammar::repr::{Bytes, Float, Infix, List, Ltree, Map, Mtree, Repr},
    num::Integer,
    utils,
};

use super::{
    LIST_LEFT, LIST_RIGHT, MAP_KV_SEPARATOR, MAP_LEFT, MAP_RIGHT, SEPARATOR, SYMBOL_PREFIX,
    WRAP_LEFT, WRAP_RIGHT,
};

const INDENT: &str = "  ";

pub(crate) fn stringify_compat(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyConfig {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: SEPARATOR.to_owned(),
        kv_separator: MAP_KV_SEPARATOR.to_owned(),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

pub(crate) fn stringify_comfort(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyConfig {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: format!("{} ", SEPARATOR),
        kv_separator: format!("{} ", MAP_KV_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

pub(crate) fn stringify_pretty(repr: &Repr) -> String {
    let mut str = String::new();
    let config = StringifyConfig {
        indent: INDENT.to_owned(),
        before_first: "\n".to_owned(),
        after_last: format!("{}\n", SEPARATOR),
        separator: format!("{}\n", SEPARATOR),
        kv_separator: format!("{} ", MAP_KV_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    stringify(repr, &mut str, &config, 0);
    str
}

struct StringifyConfig {
    indent: String,
    before_first: String,
    after_last: String,
    separator: String,
    kv_separator: String,
    left_padding: String,
    right_padding: String,
}

fn stringify(repr: &Repr, s: &mut String, config: &StringifyConfig, indent: usize) {
    match repr {
        Repr::Unit => stringify_unit(s),
        Repr::Bool(b) => stringify_bool(*b, s),
        Repr::Int(i) => stringify_int(i, s),
        Repr::Float(f) => stringify_float(f, s),
        Repr::String(str) => stringify_string(str, s),
        Repr::Letter(str) => stringify_letter(str, s),
        Repr::Symbol(str) => stringify_symbol(str, s),
        Repr::Bytes(bytes) => stringify_bytes(bytes, s),
        Repr::List(list) => stringify_list(list, s, config, indent),
        Repr::Map(map) => stringify_map(map, s, config, indent),
        Repr::Ltree(ltree) => stringify_ltree(ltree, s, config, indent),
        Repr::Mtree(mtree) => stringify_mtree(mtree, s, config, indent),
        Repr::Infix(infix) => stringify_infix(infix, s, config, indent),
    }
}

fn stringify_unit(s: &mut String) {
    s.push_str("'u")
}

fn stringify_bool(b: bool, s: &mut String) {
    s.push_str(if b { "'t" } else { "'f" })
}

fn stringify_int(i: &Integer, s: &mut String) {
    s.push_str(&i.to_string())
}

fn stringify_float(f: &Float, s: &mut String) {
    let (b, digits, exp) = f.to_sign_string_exp(10, None);
    let sign = if b { "-" } else { "" };
    let exp = if exp.is_some() { exp.unwrap() } else { 0 };
    let exp = if exp == 0 {
        "".to_owned()
    } else {
        format!("e{exp}")
    };
    write!(s, "{sign}0.{digits}{exp}").unwrap();
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
    if str.len() > 1 {
        s.push_str(SYMBOL_PREFIX)
    }
    s.push_str(str)
}

fn stringify_bytes(bytes: &Bytes, s: &mut String) {
    s.push('\'');
    utils::conversion::u8_array_to_hex_string_mut(bytes, s);
}

fn stringify_wrapped(repr: &Repr, s: &mut String, config: &StringifyConfig, indent: usize) {
    s.push_str(WRAP_LEFT);
    s.push_str(&config.left_padding);
    stringify(repr, s, config, indent);
    s.push_str(&config.right_padding);
    s.push_str(WRAP_RIGHT);
}

fn stringify_list(list: &List, s: &mut String, config: &StringifyConfig, indent: usize) {
    s.push_str(LIST_LEFT);
    if list.is_empty() {
        s.push_str(LIST_RIGHT);
        return;
    }

    if list.len() == 1 {
        s.push_str(&config.left_padding);
        stringify(list.first().unwrap(), s, config, indent);
        s.push_str(&config.right_padding);
        s.push_str(LIST_RIGHT);
        return;
    }

    s.push_str(&config.before_first);
    for repr in list.iter() {
        s.push_str(&config.indent.repeat(indent + 1));
        stringify(repr, s, config, indent + 1);
        s.push_str(&config.separator);
    }
    s.truncate(s.len() - config.separator.len());
    s.push_str(&config.after_last);

    s.push_str(&config.indent.repeat(indent));
    s.push_str(LIST_RIGHT);
}

fn stringify_map(map: &Map, s: &mut String, config: &StringifyConfig, indent: usize) {
    s.push_str(MAP_LEFT);
    if map.is_empty() {
        s.push_str(MAP_RIGHT);
        return;
    }

    if map.len() == 1 {
        let pair = map.iter().next().unwrap();
        s.push_str(&config.left_padding);
        stringify(pair.0, s, config, indent);
        s.push_str(&config.kv_separator);
        stringify(pair.1, s, config, indent);
        s.push_str(&config.right_padding);
        s.push_str(MAP_RIGHT);
        return;
    }

    s.push_str(&config.before_first);
    for pair in map.iter() {
        s.push_str(&config.indent.repeat(indent + 1));
        stringify(pair.0, s, config, indent + 1);
        s.push_str(&config.kv_separator);
        stringify(pair.1, s, config, indent + 1);
        s.push_str(&config.separator);
    }
    s.truncate(s.len() - config.separator.len());
    s.push_str(&config.after_last);

    s.push_str(&config.indent.repeat(indent));
    s.push_str(MAP_RIGHT);
}

fn stringify_ltree(ltree: &Ltree, s: &mut String, config: &StringifyConfig, indent: usize) {
    if matches!(ltree.root, Repr::Infix(_)) {
        stringify_wrapped(&ltree.root, s, config, indent);
    } else {
        stringify(&ltree.root, s, config, indent);
    }
    stringify_list(&ltree.leaves, s, config, indent);
}

fn stringify_mtree(mtree: &Mtree, s: &mut String, config: &StringifyConfig, indent: usize) {
    if matches!(mtree.root, Repr::Infix(_)) {
        stringify_wrapped(&mtree.root, s, config, indent);
    } else {
        stringify(&mtree.root, s, config, indent);
    }
    stringify_map(&mtree.leaves, s, config, indent);
}

fn stringify_infix(infix: &Infix, s: &mut String, config: &StringifyConfig, indent: usize) {
    stringify(&infix.left, s, config, indent);
    s.push(' ');
    if matches!(infix.infix, Repr::List(_) | Repr::Map(_) | Repr::Infix(_)) {
        stringify_wrapped(&infix.infix, s, config, indent);
    } else {
        stringify(&infix.infix, s, config, indent);
    }
    s.push(' ');
    if matches!(infix.right, Repr::List(_) | Repr::Map(_) | Repr::Infix(_)) {
        stringify_wrapped(&infix.right, s, config, indent);
    } else {
        stringify(&infix.right, s, config, indent);
    }
}
