use std::fmt::Alignment;
use std::fmt::Binary;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::LowerHex;
use std::fmt::Pointer;
use std::fmt::Write;
use std::fmt::from_fn;

use bigdecimal::BigDecimal;
use const_format::concatcp;
use num_bigint::BigInt;
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
use super::RIGHT;
use super::SCOPE_LEFT;
use super::SCOPE_RIGHT;
use super::SEPARATOR;
use super::TEXT_QUOTE;
use super::TRUE;
use super::UNIT;
use super::is_delimiter;
use super::keyword;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Decimal;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Default, Copy, Clone)]
pub struct FmtCtx {
    direction: Direction,
}

pub trait FmtRepr {
    /// '#' for pretty
    /// alignment for direction: none or '^' for smart direction
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result;

    fn is_call(&self) -> bool {
        false
    }

    fn is_pair(&self) -> bool {
        false
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        panic!("called `FmtRepr::to_pair()` on non-pair value")
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unit_fmt(f)
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unit_fmt(f)
    }
}

fn unit_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(UNIT)
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        bit_fmt(*self, f)
    }
}

impl Debug for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        bit_fmt(*self, f)
    }
}

fn bit_fmt(bit: Bit, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(if *bit { TRUE } else { FALSE })
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        key_fmt(self.clone(), f)
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        key_fmt(self.clone(), f)
    }
}

fn key_fmt(key: Key, f: &mut Formatter<'_>) -> std::fmt::Result {
    if f.sign_minus() {
        return key_esc(&key, f);
    }
    if f.sign_plus() || key_should_quote(&key) {
        f.write_char(KEY_QUOTE)?;
        key_esc(&key, f)?;
        f.write_char(KEY_QUOTE)
    } else {
        f.write_str(&key)
    }
}

fn key_esc(key: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    for c in key.chars() {
        match c {
            '^' => f.write_str("^^")?,
            KEY_QUOTE => f.write_str(concatcp!('^', TEXT_QUOTE))?,
            _ => f.write_char(c)?,
        }
    }
    Ok(())
}

fn key_should_quote(str: &str) -> bool {
    if str.is_empty() {
        return true;
    }
    if keyword(str) {
        return true;
    }
    let first = str.chars().next().unwrap();
    if first.is_ascii_digit() {
        return true;
    }
    str.chars().any(is_delimiter)
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        text_fmt(self, f)
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        text_fmt(self, f)
    }
}

// key encoding
impl Pointer for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !f.sign_minus() {
            f.write_char(TEXT_QUOTE)?;
        }
        text_key_encoding(self, f)?;
        if !f.sign_minus() {
            f.write_char(TEXT_QUOTE)?;
        }
        Ok(())
    }
}

fn text_fmt(text: &Text, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !f.sign_minus() {
        f.write_char(TEXT_QUOTE)?;
    }
    if f.alternate() && text.contains('\n') {
        text_raw(text, f)?;
    } else {
        text_esc(text, f)?;
    }
    if !f.sign_minus() {
        f.write_char(TEXT_QUOTE)?;
    }
    Ok(())
}

fn text_esc(str: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    for c in str.chars() {
        let escaped = match c {
            '^' => "^^",
            '\n' => "^n",
            '\r' => "^r",
            '\t' => "^t",
            TEXT_QUOTE => concatcp!('^', KEY_QUOTE),
            _ => {
                f.write_char(c)?;
                continue;
            }
        };
        f.write_str(escaped)?;
    }
    Ok(())
}

fn text_key_encoding(str: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    for c in str.chars() {
        let escaped = match c {
            '^' => "^^",
            '\n' => "^n",
            '\r' => "^r",
            '\t' => "^t",
            TEXT_QUOTE => concatcp!('^', KEY_QUOTE),
            c if Key::is_key(c) => &format!("{c}"),
            c => &format!("^u({:x})", c as u32),
        };
        f.write_str(escaped)?;
    }
    Ok(())
}

fn text_raw(str: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("\n|(")?;
    for line in str.split_inclusive('\n') {
        f.write_str(line)?;
        if line.ends_with('\n') {
            f.write_str("+ ")?;
        }
    }
    f.write_str("\n|)")
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        int_fmt(self, f)
    }
}

impl Debug for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        int_fmt(self, f)
    }
}

fn int_fmt(int: &Int, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !f.sign_minus() && int.is_negative() {
        f.write_char('0')?;
    }
    <BigInt as Display>::fmt(int, f)
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        decimal_fmt(self, f)
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        decimal_fmt(self, f)
    }
}

fn decimal_fmt(decimal: &Decimal, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !f.sign_minus() {
        f.write_char('0')?;
    }
    if decimal.is_negative() {
        f.write_char('-')?;
    }
    f.write_char('E')?;
    Display::fmt(&decimal.order_of_magnitude(), f)?;
    f.write_char('*')?;
    let (i, _exp) = decimal.abs().into_bigint_and_scale();
    let scale = (decimal.digits() - 1) as i64;
    let significand = BigDecimal::from_bigint(i, scale);
    let no_frac = significand.fractional_digit_count() <= 0;
    significand.write_plain_string(f)?;
    if no_frac {
        f.write_char('.')?;
    }
    Ok(())
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        byte_fmt(self, 16, f)
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        byte_fmt(self, 16, f)
    }
}

impl LowerHex for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        byte_fmt(self, 16, f)
    }
}

impl Binary for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        byte_fmt(self, 2, f)
    }
}

fn byte_fmt(byte: &Byte, radix: u8, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !f.sign_minus() {
        f.write_str(BYTE)?;
        f.write_char(KEY_QUOTE)?;
    }
    match radix {
        16 => {
            for &b in byte.iter() {
                write!(f, "{b:02x}")?;
            }
        }
        2 => {
            for &b in byte.iter() {
                write!(f, "{b:08b}")?;
            }
        }
        _ => unreachable!("invalid radix {radix}"),
    }
    if !f.sign_minus() {
        f.write_char(KEY_QUOTE)?;
    }
    Ok(())
}

impl<T: FmtRepr> Display for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> Debug for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> FmtRepr for Cell<T> {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(UNIT)?;
        f.write_char(SCOPE_LEFT)?;
        self.value.fmt(ctx, f)?;
        f.write_char(SCOPE_RIGHT)
    }
}

impl<T: FmtRepr> Display for Pair<T, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> Debug for Pair<T, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> FmtRepr for Pair<T, T> {
    fn fmt(&self, mut ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        let align = f.align().unwrap_or(Alignment::Center);
        match ctx.direction {
            Direction::Left => {
                if best_left(align, &self.left, &self.right) {
                    return pair_fmt_left(self, ctx, f);
                }
                f.write_str(RIGHT)?;
                f.write_char(SCOPE_LEFT)?;
                ctx.direction = Direction::Right;
                pair_fmt_right(self, ctx, f)?;
                f.write_char(SCOPE_RIGHT)
            }
            Direction::Right => {
                if best_right(align, &self.left, &self.right) {
                    return pair_fmt_right(self, ctx, f);
                }
                f.write_str(LEFT)?;
                f.write_char(SCOPE_LEFT)?;
                ctx.direction = Direction::Left;
                pair_fmt_left(self, ctx, f)?;
                f.write_char(SCOPE_RIGHT)
            }
        }
    }

    fn is_pair(&self) -> bool {
        true
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        Pair::new(&self.left, &self.right)
    }
}

fn pair_fmt_left<T: FmtRepr>(
    pair: &Pair<T, T>, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    pair.left.fmt(ctx, f)?;
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    closure(&pair.right, ctx, f)
}

fn pair_fmt_right<T: FmtRepr>(
    pair: &Pair<T, T>, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    closure(&pair.left, ctx, f)?;
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    pair.right.fmt(ctx, f)
}

impl<T: FmtRepr> Display for Call<T, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> Debug for Call<T, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> FmtRepr for Call<T, T> {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() && self.input.is_pair() {
            call_fmt_infix(&self.func, self.input.to_pair(), ctx, f)
        } else {
            call_fmt_default(&self.func, &self.input, ctx, f)
        }
    }

    fn is_call(&self) -> bool {
        true
    }
}

fn call_fmt_infix<T: FmtRepr>(
    func: &T, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, mut ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let align = f.align().unwrap_or(Alignment::Center);
    match ctx.direction {
        Direction::Left => {
            if best_left(align, pair.left, pair.right) {
                return call_fmt_infix_left(func, pair, ctx, f);
            }
            f.write_str(RIGHT)?;
            f.write_char(SCOPE_LEFT)?;
            ctx.direction = Direction::Right;
            call_fmt_infix_right(func, pair, ctx, f)?;
            f.write_char(SCOPE_RIGHT)
        }
        Direction::Right => {
            if best_right(align, pair.left, pair.right) {
                return call_fmt_infix_right(func, pair, ctx, f);
            }
            f.write_str(LEFT)?;
            f.write_char(SCOPE_LEFT)?;
            ctx.direction = Direction::Left;
            call_fmt_infix_left(func, pair, ctx, f)?;
            f.write_char(SCOPE_RIGHT)
        }
    }
}

fn call_fmt_infix_left<T: FmtRepr>(
    func: &T, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    pair.left.fmt(ctx, f)?;
    f.write_char(' ')?;
    closure(func, ctx, f)?;
    f.write_char(' ')?;
    closure(pair.right, ctx, f)
}

fn call_fmt_infix_right<T: FmtRepr>(
    func: &T, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    closure(pair.left, ctx, f)?;
    f.write_char(' ')?;
    closure(func, ctx, f)?;
    f.write_char(' ')?;
    pair.right.fmt(ctx, f)
}

fn call_fmt_default<T: FmtRepr>(
    func: &T, input: &T, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    match ctx.direction {
        Direction::Left => call_fmt_left(func, input, ctx, f),
        Direction::Right => call_fmt_right(func, input, ctx, f),
    }
}

fn call_fmt_left<T: FmtRepr>(
    func: &T, input: &T, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    input.fmt(ctx, f)?;
    f.write_char(' ')?;
    closure(func, ctx, f)?;
    f.write_char(' ')?;
    f.write_str(EMPTY)
}

fn call_fmt_right<T: FmtRepr>(
    func: &T, input: &T, ctx: FmtCtx, f: &mut Formatter<'_>,
) -> std::fmt::Result {
    f.write_str(EMPTY)?;
    f.write_char(' ')?;
    closure(func, ctx, f)?;
    f.write_char(' ')?;
    input.fmt(ctx, f)
}

fn best_left(align: Alignment, left: &dyn FmtRepr, right: &dyn FmtRepr) -> bool {
    best_direction(Direction::Left, align, left, right) == Direction::Left
}

fn best_right(align: Alignment, left: &dyn FmtRepr, right: &dyn FmtRepr) -> bool {
    best_direction(Direction::Right, align, left, right) == Direction::Right
}

fn best_direction(
    direction: Direction, align: Alignment, left: &dyn FmtRepr, right: &dyn FmtRepr,
) -> Direction {
    if align == Alignment::Left {
        return Direction::Left;
    }
    if align == Alignment::Right {
        return Direction::Right;
    }
    let left_open = is_open(left);
    let right_open = is_open(right);
    match direction {
        Direction::Left => {
            if !left_open && right_open {
                return Direction::Right;
            }
        }
        Direction::Right => {
            if left_open && !right_open {
                return Direction::Left;
            }
        }
    }
    direction
}

fn is_open(repr: &dyn FmtRepr) -> bool {
    repr.is_pair() || repr.is_call()
}

fn closure(repr: &dyn FmtRepr, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !is_open(repr) {
        return repr.fmt(ctx, f);
    }
    f.write_char(SCOPE_LEFT)?;
    repr.fmt(ctx, f)?;
    f.write_char(SCOPE_RIGHT)
}

impl<T: FmtRepr> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> Debug for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> FmtRepr for List<T> {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            f.write_char(LIST_LEFT)?;
            return f.write_char(LIST_RIGHT);
        }

        if self.len() == 1 {
            f.write_char(LIST_LEFT)?;
            self.first().unwrap().fmt(ctx, f)?;
            return f.write_char(LIST_RIGHT);
        }

        f.write_char(LIST_LEFT)?;
        if f.alternate() {
            f.write_char('\n')?;
            for repr in self {
                let repr = from_fn(|f| repr.fmt(ctx, f));
                indent(&repr, f)?;
            }
        } else {
            f.write_char(' ')?;
            for repr in self {
                repr.fmt(ctx, f)?;
                f.write_char(SEPARATOR)?;
                f.write_char(' ')?;
            }
        }
        f.write_char(LIST_RIGHT)
    }
}

impl<T: FmtRepr> Display for Map<Key, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> Debug for Map<Key, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl<T: FmtRepr> FmtRepr for Map<Key, T> {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            f.write_char(MAP_LEFT)?;
            return f.write_char(MAP_RIGHT);
        }

        if self.len() == 1 {
            f.write_char(MAP_LEFT)?;
            let (key, value) = self.iter().next().unwrap();
            kv_fmt(key.clone(), value, ctx, f)?;
            return f.write_char(MAP_RIGHT);
        }

        f.write_char(MAP_LEFT)?;
        if f.alternate() {
            f.write_char('\n')?;
            for (key, value) in self {
                let repr = from_fn(|f| kv_fmt(key.clone(), value, ctx, f));
                indent(&repr, f)?;
            }
        } else {
            f.write_char(' ')?;
            for (key, value) in self {
                kv_fmt(key.clone(), value, ctx, f)?;
                f.write_char(SEPARATOR)?;
                f.write_char(' ')?;
            }
        }
        f.write_char(MAP_RIGHT)
    }
}

fn kv_fmt<T: FmtRepr>(key: Key, value: &T, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&key, f)?;
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    value.fmt(ctx, f)
}

// TODO impl options lost
fn indent(repr: &dyn Display, f: &mut Formatter<'_>) -> std::fmt::Result {
    let align = f.align();
    let mut writer = Indent::new(f);
    match align {
        Some(Alignment::Left) => writeln!(writer, "{repr:<#}{SEPARATOR}"),
        Some(Alignment::Center) => writeln!(writer, "{repr:^#}{SEPARATOR}"),
        Some(Alignment::Right) => writeln!(writer, "{repr:>#}{SEPARATOR}"),
        None => writeln!(writer, "{repr:#}{SEPARATOR}"),
    }
}

struct Indent<'a, 'b> {
    fmt: &'a mut Formatter<'b>,
    on_newline: bool,
}

impl<'a, 'b> Write for Indent<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for s in s.split_inclusive('\n') {
            if self.on_newline {
                self.fmt.write_str("    ")?;
            }
            self.on_newline = s.ends_with('\n');
            self.fmt.write_str(s)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if self.on_newline {
            self.fmt.write_str("    ")?;
        }
        self.on_newline = c == '\n';
        self.fmt.write_char(c)
    }
}

impl<'a, 'b> Indent<'a, 'b> {
    fn new(fmt: &'a mut Formatter<'b>) -> Self {
        Indent { fmt, on_newline: true }
    }
}
