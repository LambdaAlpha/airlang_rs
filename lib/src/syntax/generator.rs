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
        repr::{
            CallRepr,
            ListRepr,
            MapRepr,
            Repr::{
                self,
                Pair,
                Reverse,
            },
            ReverseRepr,
        },
        syntax::{
            PRESERVE_PREFIX,
            REVERSE_SEPARATOR,
        },
        types::{
            Bytes,
            Float,
            Int,
        },
        utils,
    },
    Repr::{
        Call,
        List,
        Map,
    },
};

pub(crate) const INDENT: &str = "  ";

#[allow(dead_code)]
pub(crate) fn generate_compat(repr: &Repr) -> String {
    let mut str = String::new();
    let config = GenerateFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: SEPARATOR.to_string(),
        pair_separator: PAIR_SEPARATOR.to_string(),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0);
    str
}

#[allow(dead_code)]
pub(crate) fn generate_comfort(repr: &Repr) -> String {
    let mut str = String::new();
    let config = GenerateFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: format!("{} ", SEPARATOR),
        pair_separator: format!("{} ", PAIR_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0);
    str
}

#[allow(dead_code)]
pub(crate) fn generate_pretty(repr: &Repr) -> String {
    let mut str = String::new();
    let config = GenerateFormat {
        indent: INDENT.to_owned(),
        before_first: "\n".to_owned(),
        after_last: format!("{}\n", SEPARATOR),
        separator: format!("{}\n", SEPARATOR),
        pair_separator: format!("{} ", PAIR_SEPARATOR),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0);
    str
}

pub(crate) struct GenerateFormat {
    pub(crate) indent: String,
    pub(crate) before_first: String,
    pub(crate) after_last: String,
    pub(crate) separator: String,
    pub(crate) pair_separator: String,
    pub(crate) left_padding: String,
    pub(crate) right_padding: String,
}

pub(crate) fn generate(repr: &Repr, s: &mut String, format: &GenerateFormat, indent: usize) {
    match repr {
        Repr::Unit(_) => generate_unit(s),
        Repr::Bool(b) => generate_bool(b.bool(), s),
        Repr::Int(i) => generate_int(i, s),
        Repr::Float(f) => generate_float(f, s),
        Repr::Bytes(bytes) => generate_bytes(bytes, s),
        Repr::String(str) => generate_string(str, s),
        Repr::Letter(str) => generate_letter(str, s),
        Repr::Symbol(str) => generate_symbol(str, s),
        Pair(p) => generate_pair(&p.first, &p.second, s, format, indent),
        Call(c) => generate_call(c, s, format, indent),
        Reverse(i) => generate_reverse(i, s, format, indent),
        List(list) => generate_list(list, s, format, indent),
        Map(map) => generate_map(map, s, format, indent),
    }
}

fn generate_unit(s: &mut String) {
    s.push(PRESERVE_PREFIX);
    s.push_str("u")
}

fn generate_bool(b: bool, s: &mut String) {
    s.push(PRESERVE_PREFIX);
    s.push_str(if b { "t" } else { "f" })
}

fn generate_int(i: &Int, s: &mut String) {
    s.push_str(&i.to_string())
}

fn generate_float(f: &Float, s: &mut String) {
    s.push_str(&f.to_string())
}

fn generate_bytes(bytes: &Bytes, s: &mut String) {
    s.push_str("1x");
    utils::conversion::u8_array_to_hex_string_mut(bytes.as_ref(), s);
}

fn generate_string(str: &str, s: &mut String) {
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

fn generate_letter(str: &str, s: &mut String) {
    s.push_str(str)
}

fn generate_symbol(str: &str, s: &mut String) {
    s.push_str(str)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Association {
    // a b
    Combine,
    // a b c
    Infix,
    // a(), a{}, a:b, a?b
    Postfix,
    // a, (), {}, [a]
    Delimited,
}

fn association_of(repr: &Repr) -> Association {
    match repr {
        Pair(_) => Association::Postfix,
        Call(a) => match &a.input {
            Pair(_) => Association::Infix,
            List(_) | Map(_) => Association::Postfix,
            _ => Association::Combine,
        },
        Reverse(_) => Association::Postfix,
        _ => Association::Delimited,
    }
}

fn is_prefix(_: &Repr) -> bool {
    false
}

fn is_postfix(repr: &Repr) -> bool {
    matches!(repr, List(_) | Map(_))
}

fn wrap(wrap: bool, repr: &Repr, s: &mut String, format: &GenerateFormat, indent: usize) {
    if wrap {
        generate_wrapped(repr, s, format, indent);
    } else {
        generate(repr, s, format, indent)
    }
}

fn wrap_if_le(
    threshold: Association,
    repr: &Repr,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) {
    wrap(association_of(repr) <= threshold, repr, s, format, indent)
}

fn wrap_if_lt(
    threshold: Association,
    repr: &Repr,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) {
    wrap(association_of(repr) < threshold, repr, s, format, indent)
}

fn generate_pair(
    first: &Repr,
    second: &Repr,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) {
    wrap_if_lt(Association::Postfix, first, s, format, indent);
    s.push_str(&format.pair_separator);
    wrap_if_le(Association::Postfix, second, s, format, indent);
}

fn generate_call(call: &CallRepr, s: &mut String, format: &GenerateFormat, indent: usize) {
    match &call.input {
        Pair(p) => {
            let wrap_left = is_prefix(&p.first) || association_of(&p.first) < Association::Infix;
            wrap(wrap_left, &p.first, s, format, indent);

            s.push(' ');

            let wrap_middle = is_prefix(&call.func)
                || is_postfix(&call.func)
                || association_of(&call.func) <= Association::Infix;
            wrap(wrap_middle, &call.func, s, format, indent);

            s.push(' ');

            let wrap_right =
                is_postfix(&p.second) || association_of(&p.second) <= Association::Infix;
            wrap(wrap_right, &p.second, s, format, indent)
        }
        List(_) | Map(_) => {
            wrap_if_lt(Association::Postfix, &call.func, s, format, indent);
            generate(&call.input, s, format, indent)
        }
        _ => {
            wrap(matches!(&call.func, Call(_)), &call.func, s, format, indent);
            s.push(' ');
            wrap(
                matches!(&call.input, Call(_)),
                &call.input,
                s,
                format,
                indent,
            );
        }
    }
}

fn generate_reverse(reverse: &ReverseRepr, s: &mut String, format: &GenerateFormat, indent: usize) {
    wrap_if_lt(Association::Postfix, &reverse.func, s, format, indent);
    s.push(REVERSE_SEPARATOR);
    wrap_if_le(Association::Postfix, &reverse.output, s, format, indent);
}

fn generate_wrapped(repr: &Repr, s: &mut String, format: &GenerateFormat, indent: usize) {
    s.push(WRAP_LEFT);
    s.push_str(&format.left_padding);
    generate(repr, s, format, indent);
    s.push_str(&format.right_padding);
    s.push(WRAP_RIGHT);
}

fn generate_list(list: &ListRepr, s: &mut String, format: &GenerateFormat, indent: usize) {
    s.push(LIST_LEFT);
    if list.is_empty() {
        s.push(LIST_RIGHT);
        return;
    }

    if list.len() == 1 {
        s.push_str(&format.left_padding);
        generate(list.first().unwrap(), s, format, indent);
        s.push_str(&format.right_padding);
        s.push(LIST_RIGHT);
        return;
    }

    s.push_str(&format.before_first);
    for repr in list.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        generate(repr, s, format, indent + 1);
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(LIST_RIGHT);
}

fn generate_map(map: &MapRepr, s: &mut String, format: &GenerateFormat, indent: usize) {
    s.push(MAP_LEFT);
    if map.is_empty() {
        s.push(MAP_RIGHT);
        return;
    }

    if map.len() == 1 {
        let pair = map.iter().next().unwrap();
        s.push_str(&format.left_padding);
        generate_pair(pair.0, pair.1, s, format, indent);
        s.push_str(&format.right_padding);
        s.push(MAP_RIGHT);
        return;
    }

    s.push_str(&format.before_first);
    for pair in map.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        generate_pair(pair.0, pair.1, s, format, indent + 1);
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(MAP_RIGHT);
}
