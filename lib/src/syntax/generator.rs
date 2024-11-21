use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Write,
    },
    hash::Hash,
};

use const_format::concatcp;
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
        ambiguous,
        is_delimiter,
    },
    text::Text,
    unit::Unit,
    utils,
};

#[derive(Debug)]
pub struct ReprError {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenRepr<'a> {
    #[allow(unused)]
    Unit(&'a Unit),
    Bool(&'a Bool),
    Symbol(&'a Symbol),
    Int(&'a Int),
    Number(&'a Number),
    Text(&'a Text),
    Byte(&'a Byte),
    Pair(Box<Pair<GenRepr<'a>, GenRepr<'a>>>),
    Adapt(Box<Adapt<GenRepr<'a>, GenRepr<'a>>>),
    Call(Box<Call<GenRepr<'a>, GenRepr<'a>>>),
    Ask(Box<Ask<GenRepr<'a>, GenRepr<'a>>>),
    List(List<GenRepr<'a>>),
    Map(Map<GenRepr<'a>, GenRepr<'a>>),
}

#[derive(Copy, Clone)]
pub(crate) struct GenFmt {
    pub(crate) indent: &'static str,
    pub(crate) before_first: &'static str,
    pub(crate) after_last: &'static str,
    pub(crate) separator: &'static str,
    pub(crate) left_padding: &'static str,
    pub(crate) right_padding: &'static str,
}

pub(crate) const COMPACT_FMT: GenFmt = GenFmt {
    indent: "",
    before_first: "",
    after_last: "",
    separator: concatcp!(SEPARATOR),
    left_padding: "",
    right_padding: "",
};

pub(crate) const PRETTY_FMT: GenFmt = GenFmt {
    indent: INDENT,
    before_first: "\n",
    after_last: concatcp!(SEPARATOR, '\n'),
    separator: concatcp!(SEPARATOR, '\n'),
    left_padding: "",
    right_padding: "",
};

const INDENT: &str = "  ";

pub(crate) fn generate(repr: GenRepr, fmt: GenFmt) -> String {
    let ctx = GenCtx { fmt, indent: 0 };
    let mut str = String::new();
    gen(ctx, repr, &mut str);
    str
}

#[derive(Copy, Clone)]
struct GenCtx {
    fmt: GenFmt,
    indent: u8,
}

fn gen(ctx: GenCtx, repr: GenRepr, s: &mut String) {
    match repr {
        GenRepr::Unit(_) => gen_unit(s),
        GenRepr::Bool(bool) => gen_bool(bool.bool(), s),
        GenRepr::Symbol(symbol) => gen_symbol(symbol, s),
        GenRepr::Text(text) => gen_text(text, s),
        GenRepr::Int(int) => gen_int(int, s),
        GenRepr::Number(number) => gen_number(number, s),
        GenRepr::Byte(byte) => gen_byte(byte, s),
        GenRepr::Pair(pair) => gen_pair(ctx, *pair, s),
        GenRepr::Adapt(adapt) => gen_adapt(ctx, *adapt, s),
        GenRepr::Call(call) => gen_call(ctx, *call, s),
        GenRepr::Ask(ask) => gen_ask(ctx, *ask, s),
        GenRepr::List(list) => gen_list(ctx, list, s),
        GenRepr::Map(map) => gen_map(ctx, map, s),
    }
}

fn gen_unit(s: &mut String) {
    s.push_str(UNIT);
}

fn gen_bool(bool: bool, s: &mut String) {
    s.push_str(if bool { TRUE } else { FALSE });
}

fn gen_symbol(symbol: &Symbol, s: &mut String) {
    if !should_quote(symbol) {
        return s.push_str(symbol);
    }
    s.push(SYMBOL_QUOTE);
    escape_symbol(symbol, s);
    s.push(SYMBOL_QUOTE);
}

pub(crate) fn escape_symbol(symbol: &str, s: &mut String) {
    for c in symbol.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            SYMBOL_QUOTE => concatcp!('\\', SYMBOL_QUOTE),
            _ => {
                s.push(c);
                continue;
            }
        };
        s.push_str(escaped);
    }
}

fn should_quote(str: &str) -> bool {
    if str.is_empty() {
        return true;
    }
    if ambiguous(str) {
        return true;
    }
    let first = str.chars().next().unwrap();
    if first.is_ascii_digit() {
        return true;
    }
    str.chars().any(is_delimiter)
}

fn gen_text(text: &Text, s: &mut String) {
    s.push(TEXT_QUOTE);
    escape_text(text, s);
    s.push(TEXT_QUOTE);
}

pub(crate) fn escape_text(str: &str, s: &mut String) {
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            TEXT_QUOTE => concatcp!('\\', TEXT_QUOTE),
            _ => {
                s.push(c);
                continue;
            }
        };
        s.push_str(escaped);
    }
}

fn gen_int(int: &Int, s: &mut String) {
    if int.is_negative() {
        s.push('0');
    }
    write!(s, "{int:?}").unwrap();
}

fn gen_number(number: &Number, s: &mut String) {
    let int = number.int();
    let radix = number.radix();
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
    write!(s, "{}", number.exp()).unwrap();
}

fn gen_byte(byte: &Byte, s: &mut String) {
    s.push_str(BYTE);
    s.push(SCOPE_LEFT);
    if !byte.as_ref().is_empty() {
        utils::conversion::u8_array_to_hex_string_mut(byte.as_ref(), s);
    }
    s.push(SCOPE_RIGHT);
}

fn gen_pair(ctx: GenCtx, pair: Pair<GenRepr, GenRepr>, s: &mut String) {
    gen_scope_if_need(ctx, pair.first, s);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    gen(ctx, pair.second, s);
}

fn gen_adapt(ctx: GenCtx, adapt: Adapt<GenRepr, GenRepr>, s: &mut String) {
    gen_scope_if_need(ctx, adapt.spec, s);
    s.push(' ');
    s.push_str(ADAPT);
    s.push(' ');
    gen(ctx, adapt.value, s);
}

fn gen_call(ctx: GenCtx, call: Call<GenRepr, GenRepr>, s: &mut String) {
    if let GenRepr::Pair(pair) = call.input {
        gen_scope_if_need(ctx, pair.first, s);
        s.push(' ');
        gen_scope_if_need(ctx, call.func, s);
        s.push(' ');
        gen(ctx, pair.second, s);
    } else {
        gen_scope_if_need(ctx, call.func, s);
        s.push(' ');
        s.push_str(CALL);
        s.push(' ');
        gen(ctx, call.input, s);
    }
}

fn gen_ask(ctx: GenCtx, ask: Ask<GenRepr, GenRepr>, s: &mut String) {
    gen_scope_if_need(ctx, ask.func, s);
    s.push(' ');
    s.push_str(ASK);
    s.push(' ');
    gen(ctx, ask.output, s);
}

fn gen_scope_if_need(ctx: GenCtx, repr: GenRepr, s: &mut String) {
    if is_composite(&repr) {
        gen_scope(ctx, repr, s);
    } else {
        gen(ctx, repr, s);
    }
}

fn gen_scope(ctx: GenCtx, repr: GenRepr, s: &mut String) {
    s.push(SCOPE_LEFT);
    s.push_str(ctx.fmt.left_padding);
    gen(ctx, repr, s);
    s.push_str(ctx.fmt.right_padding);
    s.push(SCOPE_RIGHT);
}

fn is_composite(repr: &GenRepr) -> bool {
    matches!(
        repr,
        GenRepr::Call(_) | GenRepr::Ask(_) | GenRepr::Pair(_) | GenRepr::Adapt(_)
    )
}

fn gen_list(mut ctx: GenCtx, mut list: List<GenRepr>, s: &mut String) {
    if list.is_empty() {
        s.push(LIST_LEFT);
        s.push(LIST_RIGHT);
        return;
    }

    if list.len() == 1 {
        s.push(LIST_LEFT);
        s.push_str(ctx.fmt.left_padding);
        gen(ctx, list.pop().unwrap(), s);
        s.push_str(ctx.fmt.right_padding);
        s.push(LIST_RIGHT);
        return;
    }

    s.push(LIST_LEFT);
    ctx.indent += 1;
    s.push_str(ctx.fmt.before_first);

    for repr in list.into_iter() {
        s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
        gen(ctx, repr, s);
        s.push_str(ctx.fmt.separator);
    }
    s.truncate(s.len() - ctx.fmt.separator.len());

    s.push_str(ctx.fmt.after_last);
    ctx.indent -= 1;
    s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
    s.push(LIST_RIGHT);
}

fn gen_map(mut ctx: GenCtx, map: Map<GenRepr, GenRepr>, s: &mut String) {
    if map.is_empty() {
        s.push(MAP_LEFT);
        s.push(MAP_RIGHT);
        return;
    }

    if map.len() == 1 {
        s.push(MAP_LEFT);
        let pair = map.into_iter().next().unwrap();
        s.push_str(ctx.fmt.left_padding);
        gen_kv(ctx, pair.0, pair.1, s);
        s.push_str(ctx.fmt.right_padding);
        s.push(MAP_RIGHT);
        return;
    }

    s.push(MAP_LEFT);
    ctx.indent += 1;
    s.push_str(ctx.fmt.before_first);

    for pair in map.into_iter() {
        s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
        gen_kv(ctx, pair.0, pair.1, s);
        s.push_str(ctx.fmt.separator);
    }
    s.truncate(s.len() - ctx.fmt.separator.len());

    s.push_str(ctx.fmt.after_last);
    ctx.indent -= 1;
    s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
    s.push(MAP_RIGHT);
}

fn gen_kv(ctx: GenCtx, key: GenRepr, value: GenRepr, s: &mut String) {
    gen_scope_if_need(ctx, key, s);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    gen(ctx, value, s);
}

impl Display for ReprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReprError")
    }
}

impl Error for ReprError {}
