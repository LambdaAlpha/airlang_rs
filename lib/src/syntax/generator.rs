use std::fmt::Write;
use std::hash::Hash;

use const_format::concatcp;
use num_traits::Signed;

use crate::bit::Bit;
use crate::byte::Byte;
use crate::call::Call;
use crate::int::Int;
use crate::list::List;
use crate::map::Map;
use crate::number::Number;
use crate::pair::Pair;
use crate::symbol::Symbol;
use crate::syntax::BYTE;
use crate::syntax::CALL_FORWARD;
use crate::syntax::CALL_REVERSE;
use crate::syntax::FALSE;
use crate::syntax::LIST_LEFT;
use crate::syntax::LIST_RIGHT;
use crate::syntax::MAP_LEFT;
use crate::syntax::MAP_RIGHT;
use crate::syntax::PAIR;
use crate::syntax::QUOTE;
use crate::syntax::SCOPE_LEFT;
use crate::syntax::SCOPE_RIGHT;
use crate::syntax::SEPARATOR;
use crate::syntax::SYMBOL_QUOTE;
use crate::syntax::TEXT_QUOTE;
use crate::syntax::TRUE;
use crate::syntax::UNIT;
use crate::syntax::ambiguous;
use crate::syntax::is_delimiter;
use crate::text::Text;
use crate::unit::Unit;
use crate::utils;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenRepr<'a> {
    Unit(&'a Unit),
    Bit(&'a Bit),
    Symbol(&'a Symbol),
    Int(&'a Int),
    Number(&'a Number),
    Text(&'a Text),
    Byte(&'a Byte),
    Pair(Box<Pair<GenRepr<'a>, GenRepr<'a>>>),
    Call(Box<Call<GenRepr<'a>, GenRepr<'a>>>),
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
    pub(crate) symbol_encoding: bool,
}

pub(crate) const COMPACT_FMT: GenFmt = GenFmt {
    indent: "",
    before_first: "",
    after_last: "",
    separator: concatcp!(SEPARATOR),
    left_padding: "",
    right_padding: "",
    symbol_encoding: false,
};

pub(crate) const SYMBOL_FMT: GenFmt = GenFmt {
    indent: "",
    before_first: "",
    after_last: "",
    separator: concatcp!(SEPARATOR),
    left_padding: "",
    right_padding: "",
    symbol_encoding: true,
};

pub(crate) const PRETTY_FMT: GenFmt = GenFmt {
    indent: INDENT,
    before_first: "\n",
    after_last: concatcp!(SEPARATOR, '\n'),
    separator: concatcp!(SEPARATOR, '\n'),
    left_padding: "",
    right_padding: "",
    symbol_encoding: false,
};

const INDENT: &str = "  ";

pub(crate) fn generate(repr: GenRepr, fmt: GenFmt) -> String {
    let ctx = GenCtx { fmt, indent: 0 };
    let mut str = String::new();
    gen1(ctx, &mut str, repr);
    str
}

#[derive(Copy, Clone)]
struct GenCtx {
    fmt: GenFmt,
    indent: u8,
}

fn gen1(ctx: GenCtx, s: &mut String, repr: GenRepr) {
    match repr {
        GenRepr::Unit(_) => gen_unit(ctx, s),
        GenRepr::Bit(bit) => gen_bit(ctx, s, bit.bool()),
        GenRepr::Symbol(symbol) => gen_symbol(ctx, s, symbol),
        GenRepr::Text(text) => gen_text(ctx, s, text),
        GenRepr::Int(int) => gen_int(ctx, s, int),
        GenRepr::Number(number) => gen_number(ctx, s, number),
        GenRepr::Byte(byte) => gen_byte(ctx, s, byte),
        GenRepr::Pair(pair) => gen_pair(ctx, s, *pair),
        GenRepr::Call(call) => gen_call(ctx, s, *call),
        GenRepr::List(list) => gen_list(ctx, s, list),
        GenRepr::Map(map) => gen_map(ctx, s, map),
    }
}

fn gen_unit(_ctx: GenCtx, s: &mut String) {
    s.push_str(UNIT);
}

fn gen_bit(_ctx: GenCtx, s: &mut String, bool: bool) {
    s.push_str(if bool { TRUE } else { FALSE });
}

fn gen_symbol(_ctx: GenCtx, s: &mut String, symbol: &Symbol) {
    if !should_quote(symbol) {
        return s.push_str(symbol);
    }
    s.push(SYMBOL_QUOTE);
    escape_symbol(s, symbol);
    s.push(SYMBOL_QUOTE);
}

pub(crate) fn escape_symbol(s: &mut String, symbol: &str) {
    for c in symbol.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            SYMBOL_QUOTE => concatcp!('\\', QUOTE),
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

fn gen_text(ctx: GenCtx, s: &mut String, text: &Text) {
    s.push(TEXT_QUOTE);
    if ctx.fmt.symbol_encoding {
        escape_text_symbol(s, text);
    } else {
        escape_text(s, text);
    }
    s.push(TEXT_QUOTE);
}

pub(crate) fn escape_text(s: &mut String, str: &str) {
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            TEXT_QUOTE => concatcp!('\\', QUOTE),
            _ => {
                s.push(c);
                continue;
            }
        };
        s.push_str(escaped);
    }
}

pub(crate) fn escape_text_symbol(s: &mut String, str: &str) {
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            TEXT_QUOTE => concatcp!('\\', QUOTE),
            c if Symbol::is_symbol(c) => &format!("{c}"),
            c => &format!("\\u({:x})", c as u32),
        };
        s.push_str(escaped);
    }
}

fn gen_int(_ctx: GenCtx, s: &mut String, int: &Int) {
    if int.is_negative() {
        s.push('0');
    }
    write!(s, "{int:?}").unwrap();
}

fn gen_number(_ctx: GenCtx, s: &mut String, number: &Number) {
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

fn gen_byte(ctx: GenCtx, s: &mut String, byte: &Byte) {
    prefixed(ctx, s, BYTE, |_ctx, s| {
        if !byte.is_empty() {
            utils::conversion::u8_array_to_hex_string_mut(byte, s);
        }
    });
}

fn gen_pair(ctx: GenCtx, s: &mut String, pair: Pair<GenRepr, GenRepr>) {
    gen_scope_if_need(ctx, s, pair.first);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    gen1(ctx, s, pair.second);
}

fn gen_call(ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr>) {
    if !call.reverse {
        if let GenRepr::Pair(pair) = call.input {
            gen_scope_if_need(ctx, s, pair.first);
            s.push(' ');
            gen_scope_if_need(ctx, s, call.func);
            s.push(' ');
            gen1(ctx, s, pair.second);
            return;
        }
    }
    gen_scope_if_need(ctx, s, call.func);
    s.push(' ');
    let infix = if call.reverse { CALL_REVERSE } else { CALL_FORWARD };
    s.push_str(infix);
    s.push(' ');
    gen1(ctx, s, call.input);
}

fn gen_scope_if_need(ctx: GenCtx, s: &mut String, repr: GenRepr) {
    if is_composite(&repr) {
        gen_scope(ctx, s, repr);
    } else {
        gen1(ctx, s, repr);
    }
}

fn gen_scope(ctx: GenCtx, s: &mut String, repr: GenRepr) {
    scoped(ctx, s, |ctx, s| gen1(ctx, s, repr));
}

fn is_composite(repr: &GenRepr) -> bool {
    matches!(repr, GenRepr::Pair(_) | GenRepr::Call(_))
}

fn gen_list(mut ctx: GenCtx, s: &mut String, mut list: List<GenRepr>) {
    if list.is_empty() {
        s.push(LIST_LEFT);
        s.push(LIST_RIGHT);
        return;
    }

    if list.len() == 1 {
        s.push(LIST_LEFT);
        s.push_str(ctx.fmt.left_padding);
        gen1(ctx, s, list.pop().unwrap());
        s.push_str(ctx.fmt.right_padding);
        s.push(LIST_RIGHT);
        return;
    }

    s.push(LIST_LEFT);
    ctx.indent += 1;
    s.push_str(ctx.fmt.before_first);

    for repr in list {
        s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
        gen1(ctx, s, repr);
        s.push_str(ctx.fmt.separator);
    }
    s.truncate(s.len() - ctx.fmt.separator.len());

    s.push_str(ctx.fmt.after_last);
    ctx.indent -= 1;
    s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
    s.push(LIST_RIGHT);
}

fn gen_map(mut ctx: GenCtx, s: &mut String, map: Map<GenRepr, GenRepr>) {
    if map.is_empty() {
        s.push(MAP_LEFT);
        s.push(MAP_RIGHT);
        return;
    }

    if map.len() == 1 {
        s.push(MAP_LEFT);
        let pair = map.into_iter().next().unwrap();
        s.push_str(ctx.fmt.left_padding);
        gen_kv(ctx, s, pair.0, pair.1);
        s.push_str(ctx.fmt.right_padding);
        s.push(MAP_RIGHT);
        return;
    }

    s.push(MAP_LEFT);
    ctx.indent += 1;
    s.push_str(ctx.fmt.before_first);

    for pair in map {
        s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
        gen_kv(ctx, s, pair.0, pair.1);
        s.push_str(ctx.fmt.separator);
    }
    s.truncate(s.len() - ctx.fmt.separator.len());

    s.push_str(ctx.fmt.after_last);
    ctx.indent -= 1;
    s.push_str(&ctx.fmt.indent.repeat(ctx.indent as usize));
    s.push(MAP_RIGHT);
}

fn gen_kv(ctx: GenCtx, s: &mut String, key: GenRepr, value: GenRepr) {
    gen_scope_if_need(ctx, s, key);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    gen1(ctx, s, value);
}

fn prefixed(ctx: GenCtx, s: &mut String, tag: &str, f: impl FnOnce(GenCtx, &mut String)) {
    s.push_str(tag);
    scoped(ctx, s, f);
}

fn scoped(ctx: GenCtx, s: &mut String, f: impl FnOnce(GenCtx, &mut String)) {
    s.push(SCOPE_LEFT);
    s.push_str(ctx.fmt.left_padding);
    f(ctx, s);
    s.push_str(ctx.fmt.right_padding);
    s.push(SCOPE_RIGHT);
}
