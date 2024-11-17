use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Write,
    },
    hash::Hash,
};

use num_traits::Signed;

use crate::{
    adapt::Adapt,
    ask::Ask,
    bool::Bool,
    byte::Byte,
    call::Call,
    int::Int,
    list::List,
    map::Map,
    number::Number,
    pair::Pair,
    symbol::Symbol,
    syntax::{
        ADAPT,
        ASK,
        BYTE,
        CALL,
        FALSE,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR,
        SCOPE_LEFT,
        SCOPE_RIGHT,
        SEPARATOR,
        SYMBOL_QUOTE,
        TEXT_QUOTE,
        TRUE,
        UNIT,
        is_delimiter,
        maybe_keyword,
    },
    text::Text,
    unit::Unit,
    utils,
};

#[derive(Debug)]
pub struct ReprError {}

pub(crate) const INDENT: &str = "  ";

pub(crate) fn generate_compact<'a, T>(
    repr: &'a T,
) -> Result<String, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
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
    Number(&'a Number),
    Text(&'a Text),
    Pair(&'a Pair<T, T>),
    List(&'a List<T>),
    Map(&'a Map<T, T>),
    Byte(&'a Byte),
    Call(&'a Call<T, T>),
    Ask(&'a Ask<T, T>),
    Adapt(&'a Adapt<T, T>),
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
        GenerateRepr::Number(n) => generate_number(n, s),
        GenerateRepr::Text(t) => generate_text(t, s),
        GenerateRepr::Pair(p) => generate_pair(&p.first, &p.second, s, format, indent)?,
        GenerateRepr::List(list) => generate_list(list, s, format, indent)?,
        GenerateRepr::Map(map) => generate_map(map, s, format, indent)?,
        GenerateRepr::Byte(byte) => generate_byte(byte, s),
        GenerateRepr::Call(c) => generate_call(c, s, format, indent)?,
        GenerateRepr::Ask(i) => generate_ask(i, s, format, indent)?,
        GenerateRepr::Adapt(f) => generate_adapt(f, s, format, indent)?,
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
    if i.is_negative() {
        s.push('0');
    }
    write!(s, "{i:?}").unwrap();
}

fn generate_number(n: &Number, s: &mut String) {
    let int = n.int();
    let radix = n.radix();
    if int.is_negative() || radix != 10 {
        s.push('0');
    }
    if int.is_negative() {
        s.push('-');
    }
    match radix {
        16 => s.push('X'),
        2 => s.push('B'),
        10 => {}
        _ => unreachable!(),
    }
    s.push_str(&int.abs().to_str_radix(radix as u32));
    s.push('E');
    write!(s, "{}", n.exp()).unwrap();
}

fn generate_byte(byte: &Byte, s: &mut String) {
    s.push_str(BYTE);
    s.push(SCOPE_LEFT);
    if !byte.as_ref().is_empty() {
        s.push('X');
        utils::conversion::u8_array_to_hex_string_mut(byte.as_ref(), s);
    }
    s.push(SCOPE_RIGHT);
}

fn generate_text(str: &Text, s: &mut String) {
    s.push(TEXT_QUOTE);
    escape_text(str, s);
    s.push(TEXT_QUOTE);
}

pub(crate) fn escape_text(str: &str, s: &mut String) {
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\".to_owned(),
            '\n' => "\\n".to_owned(),
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            TEXT_QUOTE => format!("\\{}", TEXT_QUOTE),
            _ => c.to_string(),
        };
        s.push_str(&escaped);
    }
}

fn generate_symbol(symbol: &Symbol, s: &mut String) {
    if !is_need_quote(symbol) {
        return s.push_str(symbol);
    }

    s.push(SYMBOL_QUOTE);
    for c in symbol.chars() {
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
    if first.is_ascii_digit() {
        return true;
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
        GenerateRepr::Call(_)
            | GenerateRepr::Ask(_)
            | GenerateRepr::Pair(_)
            | GenerateRepr::Adapt(_)
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
        GenerateRepr::Call(_)
            | GenerateRepr::Ask(_)
            | GenerateRepr::Pair(_)
            | GenerateRepr::Adapt(_)
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

fn scope<'a, T>(
    scope: bool,
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    if scope {
        generate_scope(repr, s, format, indent)
    } else {
        generate(repr, s, format, indent)
    }
}

#[allow(unused)]
fn scope_if_left_open<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    scope(is_left_open(repr)?, repr, s, format, indent)
}

fn scope_if_right_open<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    scope(is_right_open(repr)?, repr, s, format, indent)
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
            s.push_str(PAIR);
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
            |s, format, indent| scope_if_right_open(&call.func, s, format, indent),
            &p.second,
            s,
            format,
            indent,
        ),
        _ => generate_infix(
            &call.func,
            |s, _format, _indent| {
                s.push_str(CALL);
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
            s.push_str(ASK);
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
    scope_if_right_open(left, s, format, indent)?;

    s.push(' ');

    generate_middle(s, format, indent)?;

    s.push(' ');

    scope(is_normal_call(right)?, right, s, format, indent)
}

fn generate_scope<'a, T>(
    repr: &'a T,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    s.push(SCOPE_LEFT);
    s.push_str(&format.left_padding);
    generate(repr, s, format, indent)?;
    s.push_str(&format.right_padding);
    s.push(SCOPE_RIGHT);
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

fn generate_adapt<'a, T>(
    adapt: &'a Adapt<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    generate_infix(
        &adapt.spec,
        |s, _format, _indent| {
            s.push_str(ADAPT);
            Ok(())
        },
        &adapt.value,
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
