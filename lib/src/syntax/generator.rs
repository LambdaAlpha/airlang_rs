use std::fmt::Write;
use std::hash::Hash;

use const_format::concatcp;
use derive_more::IsVariant;
use num_traits::Signed;

use super::BYTE;
use super::Direction;
use super::EMPTY;
use super::FALSE;
use super::KEY_QUOTE;
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
use super::TEXT_QUOTE;
use super::TRUE;
use super::UNIT;
use super::ambiguous;
use super::is_delimiter;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Text;
use crate::type_::Unit;
use crate::utils;

#[derive(Clone, PartialEq, Eq, Hash, IsVariant)]
pub enum GenRepr<'a> {
    Unit(&'a Unit),
    Bit(&'a Bit),
    Key(&'a Key),
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
    pub(crate) before_first: &'static str,
    pub(crate) after_last: &'static str,
    pub(crate) left_padding: &'static str,
    pub(crate) right_padding: &'static str,
    pub(crate) before_item: &'static str,
    pub(crate) after_item: &'static str,
    pub(crate) key_encoding: bool,
    pub(crate) compact: bool,
}

pub(crate) const COMPACT_FMT: GenFmt = GenFmt {
    before_first: "",
    after_last: "",
    left_padding: "",
    right_padding: "",
    before_item: "",
    after_item: concatcp!(SEPARATOR),
    key_encoding: false,
    compact: true,
};

pub(crate) const KEY_FMT: GenFmt = GenFmt {
    before_first: "",
    after_last: "",
    left_padding: "",
    right_padding: "",
    before_item: "",
    after_item: concatcp!(SEPARATOR),
    key_encoding: true,
    compact: true,
};

pub(crate) const PRETTY_FMT: GenFmt = GenFmt {
    before_first: "\n",
    after_last: concatcp!(SEPARATOR, '\n'),
    left_padding: "",
    right_padding: "",
    after_item: concatcp!(SEPARATOR, '\n'),
    before_item: INDENT,
    key_encoding: false,
    compact: false,
};

const INDENT: &str = "  ";

pub(crate) fn generate(repr: GenRepr, fmt: GenFmt) -> String {
    let ctx = GenCtx { fmt, indent: 0, direction: Direction::default() };
    let mut str = String::new();
    repr.gen_(ctx, &mut str);
    str
}

#[derive(Copy, Clone)]
struct GenCtx {
    fmt: GenFmt,
    indent: u8,
    direction: Direction,
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
            GenRepr::Key(key) => key.gen_(ctx, s),
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
        s.push_str(if **self { TRUE } else { FALSE });
    }
}

impl Gen for &Key {
    fn gen_(self, _ctx: GenCtx, s: &mut String) {
        if !should_quote(self) {
            return s.push_str(self);
        }
        s.push(KEY_QUOTE);
        escape_key(s, self);
        s.push(KEY_QUOTE);
    }
}

pub fn escape_key(s: &mut String, key: &str) {
    for c in key.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            KEY_QUOTE => concatcp!('\\', QUOTE),
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
        if ctx.fmt.key_encoding {
            escape_text_key(s, self);
        } else if ctx.fmt.compact {
            escape_text(s, self);
        } else if self.contains('\n') {
            escape_text_raw(ctx, s, self);
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

pub fn escape_text_key(s: &mut String, str: &str) {
    for c in str.chars() {
        let escaped = match c {
            '\\' => "\\\\",
            '\n' => "\\n",
            '\r' => "\\r",
            '\t' => "\\t",
            TEXT_QUOTE => concatcp!('\\', QUOTE),
            c if Key::is_key(c) => &format!("{c}"),
            c => &format!("\\u({:x})", c as u32),
        };
        s.push_str(escaped);
    }
}

fn escape_text_raw(ctx: GenCtx, s: &mut String, str: &str) {
    s.push('\n');
    indent(ctx, s);
    s.push_str("|(");
    for line in str.split_inclusive('\n') {
        s.push_str(line);
        if line.ends_with('\n') {
            indent(ctx, s);
            s.push_str("+ ");
        }
    }
    s.push('\n');
    indent(ctx, s);
    s.push_str("|)");
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

impl<'a> Gen for Call<GenRepr<'a>, GenRepr<'a>> {
    fn gen_(self, ctx: GenCtx, s: &mut String) {
        if let GenRepr::Pair(pair) = self.input {
            gen_call_infix(ctx, s, self.func, *pair);
        } else {
            gen_call(ctx, s, self.func, self.input);
        }
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
    s.push_str(EMPTY);
}

fn gen_call_right(ctx: GenCtx, s: &mut String, func: GenRepr, input: GenRepr) {
    s.push_str(EMPTY);
    s.push(' ');
    gen_scope_if_need(ctx, s, func);
    s.push(' ');
    input.gen_(ctx, s);
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
