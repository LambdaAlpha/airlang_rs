use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    annotation::Annotation,
    ask::Ask,
    bool::Bool,
    bytes::Bytes,
    call::Call,
    float::Float,
    int::Int,
    list::List,
    map::Map,
    pair::Pair,
    string::Str,
    symbol::Symbol,
    syntax::{
        is_delimiter,
        maybe_keyword,
        ANNOTATION_INFIX,
        ASK_INFIX,
        BYTES_PREFIX,
        CALL_INFIX,
        FALSE,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR_INFIX,
        SEPARATOR,
        STRING_QUOTE,
        SYMBOL_QUOTE,
        TRUE,
        UNIT,
        WRAP_LEFT,
        WRAP_RIGHT,
    },
    unit::Unit,
    utils,
};

#[derive(Debug)]
pub struct ReprError {}

pub(crate) const INDENT: &str = "  ";

#[allow(unused)]
pub(crate) fn generate_compat<'a, T>(
    repr: &'a T,
) -> Result<String, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: Into<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let mut str = String::new();
    let config = GenerateFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: SEPARATOR.to_string(),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0)?;
    Ok(str)
}

#[allow(unused)]
pub(crate) fn generate_comfort<'a, T>(
    repr: &'a T,
) -> Result<String, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: Into<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let mut str = String::new();
    let config = GenerateFormat {
        indent: "".to_owned(),
        before_first: "".to_owned(),
        after_last: "".to_owned(),
        separator: format!("{SEPARATOR} "),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0)?;
    Ok(str)
}

pub(crate) fn generate_pretty<'a, T>(
    repr: &'a T,
) -> Result<String, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let mut str = String::new();
    let config = GenerateFormat {
        indent: INDENT.to_owned(),
        before_first: "\n".to_owned(),
        after_last: format!("{SEPARATOR}\n"),
        separator: format!("{SEPARATOR}\n"),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0)?;
    Ok(str)
}

pub(crate) struct GenerateFormat {
    pub(crate) indent: String,
    pub(crate) before_first: String,
    pub(crate) after_last: String,
    pub(crate) separator: String,
    pub(crate) left_padding: String,
    pub(crate) right_padding: String,
}

pub(crate) enum GenerateRepr<'a, T>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    #[allow(unused)]
    Unit(&'a Unit),
    Bool(&'a Bool),
    Symbol(&'a Symbol),
    Int(&'a Int),
    Float(&'a Float),
    String(&'a Str),
    Pair(&'a Pair<T, T>),
    List(&'a List<T>),
    Map(&'a Map<T, T>),
    Bytes(&'a Bytes),
    Call(&'a Call<T, T>),
    Ask(&'a Ask<T, T>),
    Annotation(&'a Annotation<T, T>),
}

pub(crate) fn generate<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    match repr.try_into()? {
        GenerateRepr::Unit(_) => generate_unit(s),
        GenerateRepr::Bool(b) => generate_bool(b.bool(), s),
        GenerateRepr::Symbol(str) => generate_symbol(str, s),
        GenerateRepr::Int(i) => generate_int(i, s),
        GenerateRepr::Float(f) => generate_float(f, s),
        GenerateRepr::String(str) => generate_string(str, s),
        GenerateRepr::Pair(p) => generate_pair(&p.first, &p.second, s, format, indent)?,
        GenerateRepr::List(list) => generate_list(list, s, format, indent)?,
        GenerateRepr::Map(map) => generate_map(map, s, format, indent)?,
        GenerateRepr::Bytes(bytes) => generate_bytes(bytes, s),
        GenerateRepr::Call(c) => generate_call(c, s, format, indent)?,
        GenerateRepr::Ask(i) => generate_ask(i, s, format, indent)?,
        GenerateRepr::Annotation(a) => generate_annotation(a, s, format, indent)?,
    }
    Ok(())
}

fn generate_unit(s: &mut String) {
    s.push_str(UNIT);
}

fn generate_bool(b: bool, s: &mut String) {
    s.push_str(if b { TRUE } else { FALSE });
}

fn generate_int(i: &Int, s: &mut String) {
    s.push_str(&i.to_string());
}

fn generate_float(f: &Float, s: &mut String) {
    s.push_str(&f.to_string());
}

fn generate_bytes(bytes: &Bytes, s: &mut String) {
    s.push(BYTES_PREFIX);
    if bytes.as_ref().is_empty() {
        return;
    }
    s.push('x');
    utils::conversion::u8_array_to_hex_string_mut(bytes.as_ref(), s);
}

fn generate_string(str: &str, s: &mut String) {
    s.push(STRING_QUOTE);
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\".to_owned(),
            '\n' => "\\n".to_owned(),
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            STRING_QUOTE => format!("\\{}", STRING_QUOTE),
            _ => c.to_string(),
        };
        s.push_str(&escaped);
    }
    s.push(STRING_QUOTE);
}

fn generate_symbol(str: &str, s: &mut String) {
    if !is_need_quote(str) {
        return s.push_str(str);
    }

    s.push(SYMBOL_QUOTE);
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\".to_owned(),
            SYMBOL_QUOTE => format!("\\{}", SYMBOL_QUOTE),
            _ => c.to_string(),
        };
        s.push_str(&escaped);
    }
    s.push(SYMBOL_QUOTE);
}

fn is_need_quote(str: &str) -> bool {
    if str.is_empty() {
        return true;
    }
    if maybe_keyword(str) {
        return true;
    }
    let mut chars = str.chars();
    let first = chars.next().unwrap();
    match first {
        BYTES_PREFIX | SYMBOL_QUOTE | STRING_QUOTE | '0'..='9' => return true,
        '+' | '-' if matches!(chars.next(), Some('0'..='9')) => return true,
        _ => {}
    }
    str.chars().any(is_delimiter)
}

fn is_left_open<'a, T>(repr: &'a T) -> Result<bool, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let b = matches!(
        repr.try_into()?,
        GenerateRepr::Call(_) | GenerateRepr::Ask(_) | GenerateRepr::Pair(_)
    );
    Ok(b)
}

fn is_right_open<'a, T>(repr: &'a T) -> Result<bool, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let b = matches!(
        repr.try_into()?,
        GenerateRepr::Call(_) | GenerateRepr::Ask(_) | GenerateRepr::Pair(_)
    );
    Ok(b)
}

fn is_normal_call<'a, T>(
    repr: &'a T,
) -> Result<bool, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let b = match repr.try_into()? {
        GenerateRepr::Call(call) => !matches!((&call.input).try_into()?, GenerateRepr::Pair(_)),
        _ => false,
    };
    Ok(b)
}

fn wrap<'a, T>(
    wrap: bool,
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    if wrap {
        generate_wrapped(repr, s, format, indent)
    } else {
        generate(repr, s, format, indent)
    }
}

#[allow(unused)]
fn wrap_if_left_open<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    wrap(is_left_open(repr)?, repr, s, format, indent)
}

fn wrap_if_right_open<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    wrap(is_right_open(repr)?, repr, s, format, indent)
}

fn generate_pair<'a, T>(
    first: &'a T,
    second: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    generate_infix(
        first,
        |s, _format, _indent| {
            s.push_str(PAIR_INFIX);
            Ok(())
        },
        second,
        s,
        format,
        indent,
    )
}

fn generate_call<'a, T>(
    call: &'a Call<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    match (&call.input).try_into()? {
        GenerateRepr::Pair(p) => generate_infix(
            &p.first,
            |s, format, indent| wrap_if_right_open(&call.func, s, format, indent),
            &p.second,
            s,
            format,
            indent,
        ),
        _ => generate_infix(
            &call.func,
            |s, _format, _indent| {
                s.push_str(CALL_INFIX);
                Ok(())
            },
            &call.input,
            s,
            format,
            indent,
        ),
    }
}

fn generate_ask<'a, T>(
    ask: &'a Ask<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    generate_infix(
        &ask.func,
        |s, _format, _indent| {
            s.push_str(ASK_INFIX);
            Ok(())
        },
        &ask.output,
        s,
        format,
        indent,
    )
}

fn generate_infix<'a, T>(
    left: &'a T,
    generate_middle: impl FnOnce(
        &mut String,
        &GenerateFormat,
        usize,
    ) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>,
    right: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    wrap_if_right_open(left, s, format, indent)?;

    s.push(' ');

    generate_middle(s, format, indent)?;

    s.push(' ');

    wrap(is_normal_call(right)?, right, s, format, indent)
}

fn generate_wrapped<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    s.push(WRAP_LEFT);
    s.push_str(&format.left_padding);
    generate(repr, s, format, indent)?;
    s.push_str(&format.right_padding);
    s.push(WRAP_RIGHT);
    Ok(())
}

fn generate_list<'a, T>(
    list: &'a List<T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    s.push(LIST_LEFT);
    if list.is_empty() {
        s.push(LIST_RIGHT);
        return Ok(());
    }

    if list.len() == 1 {
        s.push_str(&format.left_padding);
        generate(list.first().unwrap(), s, format, indent)?;
        s.push_str(&format.right_padding);
        s.push(LIST_RIGHT);
        return Ok(());
    }

    s.push_str(&format.before_first);
    for repr in list.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        generate(repr, s, format, indent + 1)?;
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(LIST_RIGHT);
    Ok(())
}

fn generate_map<'a, T>(
    map: &'a Map<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    s.push(MAP_LEFT);
    if map.is_empty() {
        s.push(MAP_RIGHT);
        return Ok(());
    }

    if map.len() == 1 {
        let pair = map.iter().next().unwrap();
        s.push_str(&format.left_padding);
        generate_pair(pair.0, pair.1, s, format, indent)?;
        s.push_str(&format.right_padding);
        s.push(MAP_RIGHT);
        return Ok(());
    }

    s.push_str(&format.before_first);
    for pair in map.iter() {
        s.push_str(&format.indent.repeat(indent + 1));
        generate_pair(pair.0, pair.1, s, format, indent + 1)?;
        s.push_str(&format.separator);
    }
    s.truncate(s.len() - format.separator.len());
    s.push_str(&format.after_last);

    s.push_str(&format.indent.repeat(indent));
    s.push(MAP_RIGHT);
    Ok(())
}

fn generate_annotation<'a, T>(
    annotation: &'a Annotation<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    generate_infix(
        &annotation.note,
        |s, _format, _indent| {
            s.push_str(ANNOTATION_INFIX);
            Ok(())
        },
        &annotation.value,
        s,
        format,
        indent,
    )
}

impl Display for ReprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReprError")
    }
}

impl Error for ReprError {}
