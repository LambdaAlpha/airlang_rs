use std::fmt::Write;
use std::hash::Hash;

use const_format::concatcp;
use num_traits::Signed;

use super::BYTE;
use super::CALL_CTX;
use super::CALL_FORWARD;
use super::CALL_REVERSE;
use super::COMMENT;
use super::Direction;
use super::FALSE;
use super::LEFT;
use super::LIST_LEFT;
use super::LIST_RIGHT;
use super::MAP_LEFT;
use super::MAP_RIGHT;
use super::PAIR;
use super::QUOTE;
use super::RIGHT;
use super::SCOPE_LEFT;
use super::SCOPE_RIGHT;
use super::SEPARATOR;
use super::SYMBOL_QUOTE;
use super::TEXT_QUOTE;
use super::TRUE;
use super::UNIT;
use super::ambiguous;
use super::is_delimiter;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Text;
use crate::type_::Unit;
use crate::utils;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GenRepr<'a> {
    Unit(&'a Unit),
    Bit(&'a Bit),
    Symbol(&'a Symbol),
    Int(&'a Int),
    Number(&'a Number),
    Text(&'a Text),
    Byte(&'a Byte),
    Pair(Box<Pair<GenRepr<'a>, GenRepr<'a>>>),
    Call(Box<Call<GenRepr<'a>, GenRepr<'a>, GenRepr<'a>>>),
    List(List<GenRepr<'a>>),
    Map(Map<GenRepr<'a>, GenRepr<'a>>),
}

#[derive(Copy, Clone)]
pub(crate) struct GenFmt {
    pub(crate) before_first: &'static str,
    pub(crate) after_last: &'static str,
    pub(crate) left_padding: &'static str,
    pub(crate) right_padding: &'static str,
    pub(crate) before_item: &'static str,
    pub(crate) after_item: &'static str,
    pub(crate) symbol_encoding: bool,
}

pub(crate) const COMPACT_FMT: GenFmt = GenFmt {
    before_first: "",
    after_last: "",
    left_padding: "",
    right_padding: "",
    before_item: "",
    after_item: concatcp!(SEPARATOR),
    symbol_encoding: false,
};

pub(crate) const SYMBOL_FMT: GenFmt = GenFmt {
    before_first: "",
    after_last: "",
    left_padding: "",
    right_padding: "",
    before_item: "",
    after_item: concatcp!(SEPARATOR),
    symbol_encoding: true,
};

pub(crate) const PRETTY_FMT: GenFmt = GenFmt {
    before_first: "\n",
    after_last: concatcp!(SEPARATOR, '\n'),
    left_padding: "",
    right_padding: "",
    after_item: concatcp!(SEPARATOR, '\n'),
    before_item: INDENT,
    symbol_encoding: false,
};

const INDENT: &str = "  ";

pub(crate) fn generate(repr: GenRepr, fmt: GenFmt) -> String {
    let ctx = GenCtx { fmt, indent: 0, direction: Direction::default(), reverse: false };
    let mut str = String::new();
    repr.gen_(ctx, &mut str);
    str
}

#[derive(Copy, Clone)]
struct GenCtx {
    fmt: GenFmt,
    indent: u8,
    direction: Direction,
    reverse: bool,
}

trait Gen {
    fn gen_(self, ctx: GenCtx, s: &mut String);
}

// todo impl shortest repr
impl Gen for GenRepr<'_> {
    fn gen_(self, ctx: GenCtx, s: &mut String) {
        match self {
            GenRepr::Unit(unit) => unit.gen_(ctx, s),
            GenRepr::Bit(bit) => bit.gen_(ctx, s),
            GenRepr::Symbol(symbol) => symbol.gen_(ctx, s),
            GenRepr::Text(text) => text.gen_(ctx, s),
            GenRepr::Int(int) => int.gen_(ctx, s),
            GenRepr::Number(number) => number.gen_(ctx, s),
            GenRepr::Byte(byte) => byte.gen_(ctx, s),
            GenRepr::Pair(pair) => pair.gen_(ctx, s),
            GenRepr::Call(call) => call.gen_(ctx, s),
            GenRepr::List(list) => list.gen_(ctx, s),
            GenRepr::Map(map) => map.gen_(ctx, s),
        }
    }
}

impl Gen for &Unit {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        s.push_str(UNIT);
    }
}

impl Gen for &Bit {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        s.push_str(if self.bool() { TRUE } else { FALSE });
    }
}

impl Gen for &Symbol {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        if !should_quote(self) {
            return s.push_str(self);
        }
        s.push(SYMBOL_QUOTE);
        escape_symbol(s, self);
        s.push(SYMBOL_QUOTE);
    }
}

pub fn escape_symbol(s: &mut String, symbol: &str) {
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

impl Gen for &Text {
    fn gen_(self, ctx: GenCtx, s: &mut String) {
        s.push(TEXT_QUOTE);
        if ctx.fmt.symbol_encoding {
            escape_text_symbol(s, self);
        } else {
            escape_text(s, self);
        }
        s.push(TEXT_QUOTE);
    }
}

pub fn escape_text(s: &mut String, str: &str) {
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

pub fn escape_text_symbol(s: &mut String, str: &str) {
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

impl Gen for &Int {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        if self.is_negative() {
            s.push('0');
        }
        write!(s, "{self:?}").unwrap();
    }
}

impl Gen for &Number {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        let int = self.int();
        let radix = self.radix();
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
        write!(s, "{}", self.exp()).unwrap();
    }
}

impl Gen for &Byte {
    fn gen_(self, ctx: GenCtx, s: &mut String) {
        prefixed(ctx, s, BYTE, |_ctx, s| {
            if !self.is_empty() {
                utils::conversion::u8_array_to_hex_string_mut(self, s);
            }
        });
    }
}

impl<'a> Gen for Pair<GenRepr<'a>, GenRepr<'a>> {
    fn gen_(self, mut ctx: GenCtx, s: &mut String) {
        match ctx.direction {
            Direction::Left => {
                if is_open(&self.first) || !is_open(&self.second) {
                    return gen_pair_left(ctx, s, self);
                }
                ctx.direction = Direction::Right;
                prefixed(ctx, s, RIGHT, |ctx, s| {
                    gen_pair_right(ctx, s, self);
                });
            }
            Direction::Right => {
                if !is_open(&self.first) || is_open(&self.second) {
                    return gen_pair_right(ctx, s, self);
                }
                ctx.direction = Direction::Left;
                prefixed(ctx, s, LEFT, |ctx, s| {
                    gen_pair_left(ctx, s, self);
                });
            }
        }
    }
}

fn gen_pair_left(ctx: GenCtx, s: &mut String, pair: Pair<GenRepr, GenRepr>) {
    pair.first.gen_(ctx, s);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    gen_scope_if_need(ctx, s, pair.second);
}

fn gen_pair_right(ctx: GenCtx, s: &mut String, pair: Pair<GenRepr, GenRepr>) {
    gen_scope_if_need(ctx, s, pair.first);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    pair.second.gen_(ctx, s);
}

impl<'a> Gen for Call<GenRepr<'a>, GenRepr<'a>, GenRepr<'a>> {
    fn gen_(self, mut ctx: GenCtx, s: &mut String) {
        if ctx.reverse == self.reverse {
            return gen_call_elements(ctx, s, self);
        }
        ctx.reverse = self.reverse;
        let tag = if self.reverse { CALL_REVERSE } else { CALL_FORWARD };
        prefixed(ctx, s, tag, |ctx, s| gen_call_elements(ctx, s, self));
    }
}

fn gen_call_elements(ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr, GenRepr>) {
    if !call.ctx.is_unit() {
        return gen_call_ctx(ctx, s, call);
    }
    if let GenRepr::Pair(pair) = call.input {
        gen_call_infix(ctx, s, call.func, *pair);
    } else {
        gen_call(ctx, s, call.func, call.input);
    }
}

fn gen_call_infix(mut ctx: GenCtx, s: &mut String, func: GenRepr, pair: Pair<GenRepr, GenRepr>) {
    match ctx.direction {
        Direction::Left => {
            if is_open(&pair.first) || !is_open(&pair.second) {
                return gen_call_infix_left(ctx, s, func, pair);
            }
            ctx.direction = Direction::Right;
            prefixed(ctx, s, RIGHT, |ctx, s| {
                gen_call_infix_right(ctx, s, func, pair);
            });
        }
        Direction::Right => {
            if !is_open(&pair.first) || is_open(&pair.second) {
                return gen_call_infix_right(ctx, s, func, pair);
            }
            ctx.direction = Direction::Left;
            prefixed(ctx, s, LEFT, |ctx, s| {
                gen_call_infix_left(ctx, s, func, pair);
            });
        }
    }
}

fn gen_call_infix_left(ctx: GenCtx, s: &mut String, func: GenRepr, pair: Pair<GenRepr, GenRepr>) {
    pair.first.gen_(ctx, s);
    s.push(' ');
    gen_scope_if_need(ctx, s, func);
    s.push(' ');
    gen_scope_if_need(ctx, s, pair.second);
}

fn gen_call_infix_right(ctx: GenCtx, s: &mut String, func: GenRepr, pair: Pair<GenRepr, GenRepr>) {
    gen_scope_if_need(ctx, s, pair.first);
    s.push(' ');
    gen_scope_if_need(ctx, s, func);
    s.push(' ');
    pair.second.gen_(ctx, s);
}

fn gen_call(ctx: GenCtx, s: &mut String, func: GenRepr, input: GenRepr) {
    match ctx.direction {
        Direction::Left => gen_call_left(ctx, s, func, input),
        Direction::Right => gen_call_right(ctx, s, func, input),
    }
}

fn gen_call_left(ctx: GenCtx, s: &mut String, func: GenRepr, input: GenRepr) {
    input.gen_(ctx, s);
    s.push(' ');
    gen_scope_if_need(ctx, s, func);
    s.push(' ');
    s.push_str(COMMENT);
}

fn gen_call_right(ctx: GenCtx, s: &mut String, func: GenRepr, input: GenRepr) {
    s.push_str(COMMENT);
    s.push(' ');
    gen_scope_if_need(ctx, s, func);
    s.push(' ');
    input.gen_(ctx, s);
}

fn gen_call_ctx(mut ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr, GenRepr>) {
    match ctx.direction {
        Direction::Left => {
            if is_open(&call.ctx) || !is_open(&call.input) {
                return gen_call_ctx_left(ctx, s, call);
            }
            ctx.direction = Direction::Right;
            prefixed(ctx, s, RIGHT, |ctx, s| gen_call_ctx_right(ctx, s, call));
        }
        Direction::Right => {
            if !is_open(&call.ctx) || is_open(&call.input) {
                return gen_call_ctx_right(ctx, s, call);
            }
            ctx.direction = Direction::Left;
            prefixed(ctx, s, LEFT, |ctx, s| gen_call_ctx_left(ctx, s, call));
        }
    }
}

fn gen_call_ctx_left(ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr, GenRepr>) {
    call.ctx.gen_(ctx, s);
    s.push(' ');
    s.push_str(CALL_CTX);
    s.push(' ');
    gen_scope_if_need(ctx, s, call.func);
    s.push(' ');
    gen_scope_if_need(ctx, s, call.input);
}

fn gen_call_ctx_right(ctx: GenCtx, s: &mut String, call: Call<GenRepr, GenRepr, GenRepr>) {
    gen_scope_if_need(ctx, s, call.ctx);
    s.push(' ');
    s.push_str(CALL_CTX);
    s.push(' ');
    gen_scope_if_need(ctx, s, call.func);
    s.push(' ');
    call.input.gen_(ctx, s);
}

fn gen_scope_if_need(ctx: GenCtx, s: &mut String, repr: GenRepr) {
    if is_open(&repr) {
        scoped(ctx, s, |ctx, s| repr.gen_(ctx, s));
    } else {
        repr.gen_(ctx, s);
    }
}

fn is_open(repr: &GenRepr) -> bool {
    matches!(repr, GenRepr::Pair(_) | GenRepr::Call(_))
}

impl<'a> Gen for List<GenRepr<'a>> {
    fn gen_(self, mut ctx: GenCtx, s: &mut String) {
        if self.is_empty() {
            s.push(LIST_LEFT);
            s.push(LIST_RIGHT);
            return;
        }

        if self.len() == 1 {
            s.push(LIST_LEFT);
            s.push_str(ctx.fmt.left_padding);
            self.into_iter().next().unwrap().gen_(ctx, s);
            s.push_str(ctx.fmt.right_padding);
            s.push(LIST_RIGHT);
            return;
        }

        s.push(LIST_LEFT);
        ctx.indent += 1;
        s.push_str(ctx.fmt.before_first);

        for repr in self {
            indent(ctx, s);
            repr.gen_(ctx, s);
            s.push_str(ctx.fmt.after_item);
        }
        s.truncate(s.len() - ctx.fmt.after_item.len());

        s.push_str(ctx.fmt.after_last);
        ctx.indent -= 1;
        indent(ctx, s);
        s.push(LIST_RIGHT);
    }
}

impl<'a> Gen for Map<GenRepr<'a>, GenRepr<'a>> {
    fn gen_(self, mut ctx: GenCtx, s: &mut String) {
        if self.is_empty() {
            s.push(MAP_LEFT);
            s.push(MAP_RIGHT);
            return;
        }

        if self.len() == 1 {
            s.push(MAP_LEFT);
            let pair = self.into_iter().next().unwrap();
            s.push_str(ctx.fmt.left_padding);
            gen_kv(ctx, s, pair.0, pair.1);
            s.push_str(ctx.fmt.right_padding);
            s.push(MAP_RIGHT);
            return;
        }

        s.push(MAP_LEFT);
        ctx.indent += 1;
        s.push_str(ctx.fmt.before_first);

        for pair in self {
            indent(ctx, s);
            gen_kv(ctx, s, pair.0, pair.1);
            s.push_str(ctx.fmt.after_item);
        }
        s.truncate(s.len() - ctx.fmt.after_item.len());

        s.push_str(ctx.fmt.after_last);
        ctx.indent -= 1;
        indent(ctx, s);
        s.push(MAP_RIGHT);
    }
}

fn gen_kv(ctx: GenCtx, s: &mut String, key: GenRepr, value: GenRepr) {
    gen_scope_if_need(ctx, s, key);
    s.push(' ');
    s.push_str(PAIR);
    s.push(' ');
    value.gen_(ctx, s);
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

fn indent(ctx: GenCtx, s: &mut String) {
    for _ in 0 .. ctx.indent {
        s.push_str(ctx.fmt.before_item);
    }
}

impl<'a> GenRepr<'a> {
    fn is_unit(&self) -> bool {
        matches!(self, GenRepr::Unit(_))
    }
}
