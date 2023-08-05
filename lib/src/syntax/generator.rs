use {
    crate::{
        syntax::{
            COMMENT_PREFIX,
            ESCAPED_PREFIX,
            LIST_LEFT,
            LIST_RIGHT,
            MAP_LEFT,
            MAP_RIGHT,
            PAIR_SEPARATOR,
            REVERSE_SEPARATOR,
            SEPARATOR,
            STRING_QUOTE,
            WRAP_LEFT,
            WRAP_RIGHT,
        },
        types::{
            Bool,
            Bytes,
            Call,
            Float,
            Int,
            List,
            Map,
            Pair,
            Reverse,
            Str,
            Symbol,
            Unit,
        },
        utils,
    },
    std::hash::Hash,
};

pub(crate) const INDENT: &str = "  ";

#[allow(dead_code)]
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
        pair_separator: format!(" {PAIR_SEPARATOR} "),
        reverse_separator: format!(" {REVERSE_SEPARATOR} "),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0)?;
    Ok(str)
}

#[allow(dead_code)]
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
        pair_separator: format!(" {PAIR_SEPARATOR} "),
        reverse_separator: format!(" {REVERSE_SEPARATOR} "),
        left_padding: "".to_owned(),
        right_padding: "".to_owned(),
    };
    generate(repr, &mut str, &config, 0)?;
    Ok(str)
}

#[allow(dead_code)]
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
        pair_separator: format!(" {PAIR_SEPARATOR} "),
        reverse_separator: format!(" {REVERSE_SEPARATOR} "),
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
    pub(crate) pair_separator: String,
    pub(crate) reverse_separator: String,
    pub(crate) left_padding: String,
    pub(crate) right_padding: String,
}

pub(crate) enum GenerateRepr<'a, T>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    Unit(&'a Unit),
    Bool(&'a Bool),
    Int(&'a Int),
    Float(&'a Float),
    Bytes(&'a Bytes),
    String(&'a Str),
    Symbol(&'a Symbol),
    Pair(&'a Pair<T, T>),
    Call(&'a Call<T, T>),
    Reverse(&'a Reverse<T, T>),
    List(&'a List<T>),
    Map(&'a Map<T, T>),
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
        GenerateRepr::Int(i) => generate_int(i, s),
        GenerateRepr::Float(f) => generate_float(f, s),
        GenerateRepr::Bytes(bytes) => generate_bytes(bytes, s),
        GenerateRepr::String(str) => generate_string(str, s),
        GenerateRepr::Symbol(str) => generate_symbol(str, s),
        GenerateRepr::Pair(p) => generate_pair(&p.first, &p.second, s, format, indent)?,
        GenerateRepr::Call(c) => generate_call(c, s, format, indent)?,
        GenerateRepr::Reverse(i) => generate_reverse(i, s, format, indent)?,
        GenerateRepr::List(list) => generate_list(list, s, format, indent)?,
        GenerateRepr::Map(map) => generate_map(map, s, format, indent)?,
    }
    Ok(())
}

fn generate_unit(s: &mut String) {
    s.push(ESCAPED_PREFIX);
}

fn generate_bool(b: bool, s: &mut String) {
    s.push(ESCAPED_PREFIX);
    s.push_str(if b { "t" } else { "f" });
}

fn generate_int(i: &Int, s: &mut String) {
    s.push_str(&i.to_string());
}

fn generate_float(f: &Float, s: &mut String) {
    s.push_str(&f.to_string());
}

fn generate_bytes(bytes: &Bytes, s: &mut String) {
    s.push_str("1x");
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
    let mut chars = str.chars();
    let first = chars.next().unwrap();
    let escape = match first {
        ESCAPED_PREFIX | STRING_QUOTE | '0'..='9' => true,
        PAIR_SEPARATOR | REVERSE_SEPARATOR | COMMENT_PREFIX => str.len() == 1,
        '+' | '-' => matches!(chars.next(), Some('0'..='9')),
        _ => false,
    };
    if escape {
        s.push(ESCAPED_PREFIX);
    }
    s.push_str(str);
}

fn is_left_open<'a, T>(repr: &'a T) -> Result<bool, <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    let b = matches!(
        repr.try_into()?,
        GenerateRepr::Call(_) | GenerateRepr::Reverse(_) | GenerateRepr::Pair(_)
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
    generate(first, s, format, indent)?;
    s.push_str(&format.pair_separator);
    wrap_if_left_open(second, s, format, indent)
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
        GenerateRepr::Pair(p) => {
            wrap(is_normal_call(&p.first)?, &p.first, s, format, indent)?;

            s.push(' ');

            wrap_if_left_open(&call.func, s, format, indent)?;

            s.push(' ');

            wrap_if_left_open(&p.second, s, format, indent)
        }
        _ => {
            wrap(
                matches!((&call.func).try_into()?, GenerateRepr::Call(_)),
                &call.func,
                s,
                format,
                indent,
            )?;
            s.push(' ');
            wrap_if_left_open(&call.input, s, format, indent)
        }
    }
}

fn generate_reverse<'a, T>(
    reverse: &'a Reverse<T, T>,
    s: &mut String,
    format: &GenerateFormat,
    indent: usize,
) -> Result<(), <&'a T as TryInto<GenerateRepr<'a, T>>>::Error>
where
    &'a T: TryInto<GenerateRepr<'a, T>>,
    T: Eq + Hash,
{
    generate(&reverse.func, s, format, indent)?;
    s.push_str(&format.reverse_separator);
    wrap_if_left_open(&reverse.output, s, format, indent)
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
