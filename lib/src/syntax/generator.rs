use std::{
    fmt::Write,
    hash::Hash,
};

use const_format::concatcp;
use num_traits::Signed;

use crate::{
    Either,
    Generate,
    abstract1::Abstract,
    bit::Bit,
    byte::Byte,
    call::Call,
    change::Change,
    equiv::Equiv,
    int::Int,
    inverse::Inverse,
    list::List,
    map::Map,
    number::Number,
    pair::Pair,
    reify::Reify,
    symbol::Symbol,
    syntax::{
        ABSTRACT,
        BYTE,
        CALL,
        CHANGE,
        EITHER_THAT,
        EITHER_THIS,
        EQUIV,
        FALSE,
        GENERATE,
        INVERSE,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR,
        REIFY,
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
    Either(Box<Either<GenRepr<'a>, GenRepr<'a>>>),
    Change(Box<Change<GenRepr<'a>, GenRepr<'a>>>),
    Call(Box<Call<GenRepr<'a>, GenRepr<'a>>>),
    Reify(Box<Reify<GenRepr<'a>>>),
    Equiv(Box<Equiv<GenRepr<'a>>>),
    Inverse(Box<Inverse<GenRepr<'a>>>),
    Generate(Box<Generate<GenRepr<'a>>>),
    Abstract(Box<Abstract<GenRepr<'a>>>),
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
        GenRepr::Either(either) => gen_either(ctx, s, *either),
        GenRepr::Change(change) => gen_change(ctx, s, *change),
        GenRepr::Call(call) => gen_call(ctx, s, *call),
        GenRepr::Reify(reify) => gen_reify(ctx, s, *reify),
        GenRepr::Equiv(equiv) => gen_equiv(ctx, s, *equiv),
        GenRepr::Inverse(inverse) => gen_inverse(ctx, s, *inverse),
        GenRepr::Generate(generate) => gen_generate(ctx, s, *generate),
        GenRepr::Abstract(abstract1) => gen_abstract(ctx, s, *abstract1),
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
            TEXT_QUOTE => concatcp!('\\', TEXT_QUOTE),
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
            TEXT_QUOTE => concatcp!('\\', TEXT_QUOTE),
            c if Symbol::is_symbol(c) => &format!("{}", c),
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

fn gen_either(ctx: GenCtx, s: &mut String, either: Either<GenRepr, GenRepr>) {
    match either {
        Either::This(this) => prefixed(ctx, s, EITHER_THIS, |ctx, s| gen1(ctx, s, this)),
        Either::That(that) => prefixed(ctx, s, EITHER_THAT, |ctx, s| gen1(ctx, s, that)),
    }
}

fn gen_change(ctx: GenCtx, s: &mut String, change: Change<GenRepr, GenRepr>) {
    gen_scope_if_need(ctx, s, change.from);
    s.push(' ');
    s.push_str(CHANGE);
    s.push(' ');
    gen1(ctx, s, change.to);
}

fn gen_call(ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr>) {
    if let GenRepr::Pair(pair) = call.input {
        gen_scope_if_need(ctx, s, pair.first);
        s.push(' ');
        gen_scope_if_need(ctx, s, call.func);
        s.push(' ');
        gen1(ctx, s, pair.second);
    } else {
        gen_scope_if_need(ctx, s, call.func);
        s.push(' ');
        s.push_str(CALL);
        s.push(' ');
        gen1(ctx, s, call.input);
    }
}

fn gen_reify(ctx: GenCtx, s: &mut String, reify: Reify<GenRepr>) {
    prefixed(ctx, s, REIFY, |ctx, s| gen1(ctx, s, reify.func));
}

fn gen_equiv(ctx: GenCtx, s: &mut String, equiv: Equiv<GenRepr>) {
    prefixed(ctx, s, EQUIV, |ctx, s| gen1(ctx, s, equiv.func));
}

fn gen_inverse(ctx: GenCtx, s: &mut String, inverse: Inverse<GenRepr>) {
    prefixed(ctx, s, INVERSE, |ctx, s| gen1(ctx, s, inverse.func));
}

fn gen_generate(ctx: GenCtx, s: &mut String, generate: Generate<GenRepr>) {
    prefixed(ctx, s, GENERATE, |ctx, s| gen1(ctx, s, generate.func));
}

fn gen_abstract(ctx: GenCtx, s: &mut String, abstract1: Abstract<GenRepr>) {
    prefixed(ctx, s, ABSTRACT, |ctx, s| gen1(ctx, s, abstract1.func));
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
    matches!(repr, GenRepr::Pair(_) | GenRepr::Change(_) | GenRepr::Call(_))
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
