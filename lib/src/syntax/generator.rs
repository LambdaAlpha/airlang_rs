use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use bigdecimal::BigDecimal;
use const_format::concatcp;
use derive_more::IsVariant;
use num_traits::Signed;

use super::BYTE;
use super::DECIMAL;
use super::Direction;
use super::EMPTY;
use super::FALSE;
use super::INT;
use super::KEY_QUOTE;
use super::LEFT;
use super::LIST_LEFT;
use super::LIST_RIGHT;
use super::MAP_LEFT;
use super::MAP_RIGHT;
use super::PAIR;
use super::RIGHT;
use super::ReprType;
use super::SCOPE_LEFT;
use super::SCOPE_RIGHT;
use super::SEPARATOR;
use super::TEXT_QUOTE;
use super::TOKEN;
use super::TOKEN_CHAR;
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
use crate::type_::Quote;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Default, Copy, Clone)]
pub struct FmtOptions {
    pub key_encoding: bool,
    pub normalized: bool,
    pub space: SpaceFmt,
    pub direction: Direction,
    pub key_ctx: bool,
}

#[derive(Default, Copy, Clone, IsVariant)]
pub enum SpaceFmt {
    #[default]
    Compact,
    Pretty,
}

pub trait FmtRepr {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result;

    fn get_type(&self) -> ReprType;

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        panic!("called `FmtRepr::to_pair()` on non-pair value")
    }
}

impl<'a> From<&Formatter<'a>> for FmtOptions {
    fn from(f: &Formatter<'a>) -> Self {
        Self {
            space: if f.alternate() { SpaceFmt::Pretty } else { SpaceFmt::Compact },
            ..Default::default()
        }
    }
}

impl FmtRepr for Unit {
    fn fmt(&self, _options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(UNIT)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Unit
    }
}

impl FmtRepr for Bit {
    fn fmt(&self, _options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(if **self { TRUE } else { FALSE })
    }

    fn get_type(&self) -> ReprType {
        ReprType::Bit
    }
}

impl FmtRepr for Key {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if !key_should_quote(self, options) {
            return f.write_str(self);
        }
        f.write_char(KEY_QUOTE)?;
        key_compact(self, f)?;
        f.write_char(KEY_QUOTE)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Key
    }
}

fn key_compact(key: &str, f: &mut dyn Write) -> std::fmt::Result {
    let mut token_mode = false;
    for c in key.chars() {
        if let TOKEN_CHAR | KEY_QUOTE = c {
            start_token(&mut token_mode, f)?;
        } else {
            stop_token(&mut token_mode, f)?;
        }
        f.write_char(c)?;
    }
    stop_token(&mut token_mode, f)
}

fn key_should_quote(key: &str, options: FmtOptions) -> bool {
    if options.normalized {
        return true;
    }
    if key.is_empty() {
        return true;
    }
    if options.key_ctx {
        return key.chars().any(is_delimiter);
    }
    if keyword(key) {
        return true;
    }
    let first = key.chars().next().unwrap();
    if first.is_ascii_digit() {
        return true;
    }
    key.chars().any(is_delimiter)
}

impl FmtRepr for Text {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_char(TEXT_QUOTE)?;
        if options.key_encoding {
            text_compact(self, true, f)?;
        } else if options.normalized {
            text_compact(self, false, f)?;
        } else if options.space.is_pretty() && self.contains('\n') {
            text_raw(self, f)?;
        } else {
            text_compact(self, false, f)?;
        }
        f.write_char(TEXT_QUOTE)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Text
    }
}

fn text_compact(text: &str, key_encoding: bool, f: &mut dyn Write) -> std::fmt::Result {
    let mut token_mode = false;
    for c in text.chars() {
        let mut is_token = true;
        let token = match c {
            '\n' => "lf",
            '\r' => "cr",
            '\t' => "ht",
            TOKEN_CHAR => TOKEN,
            TEXT_QUOTE => concatcp!(TEXT_QUOTE),
            _ => {
                is_token = false;
                ""
            },
        };
        if is_token {
            start_token(&mut token_mode, f)?;
            f.write_str(token)?;
            continue;
        }
        if key_encoding && !Key::is_key(c) {
            start_token(&mut token_mode, f)?;
            f.write_char('X')?;
            write!(f, "{:x}", c as u32)?;
        } else {
            stop_token(&mut token_mode, f)?;
            f.write_char(c)?;
        }
    }
    stop_token(&mut token_mode, f)
}

fn text_raw(text: &str, f: &mut dyn Write) -> std::fmt::Result {
    f.write_str("\n|")?;
    f.write_str(EMPTY)?;
    for line in text.split_inclusive('\n') {
        f.write_str(line)?;
        if line.ends_with('\n') {
            f.write_str("+ ")?;
        }
    }
    f.write_str("\n|")?;
    f.write_char(TEXT_QUOTE)
}

fn start_token(token_mode: &mut bool, f: &mut dyn Write) -> std::fmt::Result {
    if *token_mode {
        return f.write_char(' ');
    }
    *token_mode = true;
    f.write_char(TOKEN_CHAR)?;
    f.write_char(SCOPE_LEFT)
}

fn stop_token(token_mode: &mut bool, f: &mut dyn Write) -> std::fmt::Result {
    if !*token_mode {
        return Ok(());
    }
    *token_mode = false;
    f.write_char(SCOPE_RIGHT)
}

impl FmtRepr for Int {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if options.normalized {
            f.write_str(INT)?;
            f.write_char(KEY_QUOTE)?;
            int_fmt(self, options, f)?;
            f.write_char(KEY_QUOTE)
        } else {
            if self.is_negative() {
                f.write_char('0')?;
            }
            int_fmt(self, options, f)
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Int
    }
}

fn int_fmt(int: &Int, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
    if int.is_negative() {
        f.write_char('-')?;
    } else if int.is_positive() && options.normalized {
        f.write_char('+')?;
    }
    if options.normalized {
        f.write_char('D')?;
    }
    write!(f, "{}", int.magnitude())
}

impl FmtRepr for Decimal {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if options.normalized {
            f.write_str(DECIMAL)?;
            f.write_char(KEY_QUOTE)?;
            decimal_fmt(self, options, f)?;
            f.write_char(KEY_QUOTE)
        } else {
            f.write_char('0')?;
            decimal_fmt(self, options, f)
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Decimal
    }
}

fn decimal_fmt(decimal: &Decimal, options: FmtOptions, mut f: &mut dyn Write) -> std::fmt::Result {
    if decimal.is_negative() {
        f.write_char('-')?;
    } else if options.normalized && decimal.is_positive() {
        f.write_char('+')?;
    }
    f.write_char('E')?;
    write!(f, "{}", &decimal.order_of_magnitude())?;
    f.write_char('*')?;
    let (i, _exp) = decimal.abs().into_bigint_and_scale();
    let scale = (decimal.digits() - 1) as i64;
    let significand = BigDecimal::from_bigint(i, scale);
    let no_frac = significand.fractional_digit_count() <= 0;
    significand.write_plain_string(&mut f)?;
    if no_frac {
        f.write_char('.')?;
    }
    Ok(())
}

impl FmtRepr for Byte {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(BYTE)?;
        f.write_char(KEY_QUOTE)?;
        if options.normalized {
            f.write_char('X')?;
        }
        for &b in self.iter() {
            write!(f, "{b:02x}")?;
        }
        f.write_char(KEY_QUOTE)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Byte
    }
}

impl<T: FmtRepr> FmtRepr for Cell<T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(UNIT)?;
        fmt_delimited(&self.value, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Cell
    }
}

impl<T: FmtRepr> FmtRepr for Quote<T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(EMPTY)?;
        fmt_delimited(&self.source, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Quote
    }
}

fn fmt_delimited(
    repr: &dyn FmtRepr, mut options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    match repr.get_type() {
        ReprType::Text | ReprType::List | ReprType::Map => repr.fmt(options, f),
        ReprType::Key => {
            options.normalized = true;
            repr.fmt(options, f)
        },
        _ => {
            f.write_char(SCOPE_LEFT)?;
            repr.fmt(options, f)?;
            f.write_char(SCOPE_RIGHT)
        },
    }
}

impl<T: FmtRepr> FmtRepr for Pair<T, T> {
    fn fmt(&self, mut options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        match options.direction {
            Direction::Left => {
                if best_left(options, &self.left, &self.right) {
                    return pair_fmt_left(self, options, f);
                }
                f.write_str(RIGHT)?;
                f.write_char(SCOPE_LEFT)?;
                options.direction = Direction::Right;
                pair_fmt_right(self, options, f)?;
                f.write_char(SCOPE_RIGHT)
            },
            Direction::Right => {
                if best_right(options, &self.left, &self.right) {
                    return pair_fmt_right(self, options, f);
                }
                f.write_str(LEFT)?;
                f.write_char(SCOPE_LEFT)?;
                options.direction = Direction::Left;
                pair_fmt_left(self, options, f)?;
                f.write_char(SCOPE_RIGHT)
            },
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Pair
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        Pair::new(&self.left, &self.right)
    }
}

fn pair_fmt_left<T: FmtRepr>(
    pair: &Pair<T, T>, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    pair.left.fmt(options, f)?;
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    closure(&pair.right, options, f)
}

fn pair_fmt_right<T: FmtRepr>(
    pair: &Pair<T, T>, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    closure(&pair.left, options, f)?;
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    pair.right.fmt(options, f)
}

impl<T: FmtRepr> FmtRepr for Call<T, T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if !options.normalized && self.input.get_type().is_pair() {
            call_fmt_infix(&self.func, self.input.to_pair(), options, f)
        } else {
            call_fmt_normalized(&self.func, &self.input, options, f)
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Call
    }
}

fn call_fmt_infix(
    func: &dyn FmtRepr, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, mut options: FmtOptions,
    f: &mut dyn Write,
) -> std::fmt::Result {
    match options.direction {
        Direction::Left => {
            if best_left(options, pair.left, pair.right) {
                return call_fmt_infix_left(func, pair, options, f);
            }
            f.write_str(RIGHT)?;
            f.write_char(SCOPE_LEFT)?;
            options.direction = Direction::Right;
            call_fmt_infix_right(func, pair, options, f)?;
            f.write_char(SCOPE_RIGHT)
        },
        Direction::Right => {
            if best_right(options, pair.left, pair.right) {
                return call_fmt_infix_right(func, pair, options, f);
            }
            f.write_str(LEFT)?;
            f.write_char(SCOPE_LEFT)?;
            options.direction = Direction::Left;
            call_fmt_infix_left(func, pair, options, f)?;
            f.write_char(SCOPE_RIGHT)
        },
    }
}

fn call_fmt_infix_left(
    func: &dyn FmtRepr, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, options: FmtOptions,
    f: &mut dyn Write,
) -> std::fmt::Result {
    pair.left.fmt(options, f)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    closure(pair.right, options, f)
}

fn call_fmt_infix_right(
    func: &dyn FmtRepr, pair: Pair<&dyn FmtRepr, &dyn FmtRepr>, options: FmtOptions,
    f: &mut dyn Write,
) -> std::fmt::Result {
    closure(pair.left, options, f)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    pair.right.fmt(options, f)
}

fn call_fmt_normalized(
    func: &dyn FmtRepr, input: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    match options.direction {
        Direction::Left => call_fmt_left(func, input, options, f),
        Direction::Right => call_fmt_right(func, input, options, f),
    }
}

fn call_fmt_left(
    func: &dyn FmtRepr, input: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    input.fmt(options, f)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    f.write_str(EMPTY)
}

fn call_fmt_right(
    func: &dyn FmtRepr, input: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    f.write_str(EMPTY)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    input.fmt(options, f)
}

fn best_left(options: FmtOptions, left: &dyn FmtRepr, right: &dyn FmtRepr) -> bool {
    let direction = if options.normalized {
        options.direction
    } else {
        best_direction(Direction::Left, left, right)
    };
    direction == Direction::Left
}

fn best_right(options: FmtOptions, left: &dyn FmtRepr, right: &dyn FmtRepr) -> bool {
    let direction = if options.normalized {
        options.direction
    } else {
        best_direction(Direction::Right, left, right)
    };
    direction == Direction::Right
}

fn best_direction(direction: Direction, left: &dyn FmtRepr, right: &dyn FmtRepr) -> Direction {
    let left_open = is_open(left);
    let right_open = is_open(right);
    match direction {
        Direction::Left => {
            if !left_open && right_open {
                return Direction::Right;
            }
        },
        Direction::Right => {
            if left_open && !right_open {
                return Direction::Left;
            }
        },
    }
    direction
}

fn is_open(repr: &dyn FmtRepr) -> bool {
    matches!(repr.get_type(), ReprType::Pair | ReprType::Call)
}

fn closure(repr: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
    if !is_open(repr) {
        return repr.fmt(options, f);
    }
    f.write_char(SCOPE_LEFT)?;
    repr.fmt(options, f)?;
    f.write_char(SCOPE_RIGHT)
}

impl<T: FmtRepr> FmtRepr for List<T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if self.is_empty() {
            f.write_char(LIST_LEFT)?;
            return f.write_char(LIST_RIGHT);
        }

        if self.len() == 1 {
            f.write_char(LIST_LEFT)?;
            self.first().unwrap().fmt(options, f)?;
            return f.write_char(LIST_RIGHT);
        }

        f.write_char(LIST_LEFT)?;
        if options.key_encoding || options.space.is_compact() {
            for repr in self {
                repr.fmt(options, f)?;
                f.write_char(SEPARATOR)?;
            }
        } else {
            f.write_char('\n')?;
            for repr in self {
                let mut indent = Indent::new(f);
                repr.fmt(options, &mut indent)?;
                indent.write_char(SEPARATOR)?;
                f.write_char('\n')?;
            }
        }
        f.write_char(LIST_RIGHT)
    }

    fn get_type(&self) -> ReprType {
        ReprType::List
    }
}

impl<T: FmtRepr> FmtRepr for Map<Key, T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if self.is_empty() {
            f.write_char(MAP_LEFT)?;
            return f.write_char(MAP_RIGHT);
        }

        if self.len() == 1 {
            f.write_char(MAP_LEFT)?;
            let (key, value) = self.iter().next().unwrap();
            kv_fmt(key.clone(), value, options, f)?;
            return f.write_char(MAP_RIGHT);
        }

        f.write_char(MAP_LEFT)?;
        if options.key_encoding || options.space.is_compact() {
            for (key, value) in self {
                kv_fmt(key.clone(), value, options, f)?;
                f.write_char(SEPARATOR)?;
            }
        } else {
            f.write_char('\n')?;
            for (key, value) in self {
                let mut indent = Indent::new(f);
                kv_fmt(key.clone(), value, options, &mut indent)?;
                indent.write_char(SEPARATOR)?;
                f.write_char('\n')?;
            }
        }
        f.write_char(MAP_RIGHT)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Map
    }
}

fn kv_fmt<T: FmtRepr>(
    key: Key, value: &T, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    let mut key_options = options;
    key_options.key_ctx = true;
    FmtRepr::fmt(&key, key_options, f)?;
    if !options.normalized && value.get_type().is_unit() {
        return Ok(());
    }
    f.write_char(' ')?;
    f.write_str(PAIR)?;
    f.write_char(' ')?;
    value.fmt(options, f)
}

macro_rules! impl_display_debug_for_fmt_repr {
    (<$repr:tt> $ty: ident <$($arg:tt),*>) => {
        impl<$repr: FmtRepr> Display for $ty<$($arg),*>  {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                FmtRepr::fmt(self, FmtOptions::from(&*f), f)
            }
        }
        impl<$repr: FmtRepr> Debug for $ty<$($arg),*> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                FmtRepr::fmt(self, FmtOptions::from(&*f), f)
            }
        }
    };
    ($ty: ty) => {
        impl Display for $ty  {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                FmtRepr::fmt(self, FmtOptions::from(&*f), f)
            }
        }
        impl Debug for $ty {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                FmtRepr::fmt(self, FmtOptions::from(&*f), f)
            }
        }
    };
}

pub(crate) use impl_display_debug_for_fmt_repr;

impl_display_debug_for_fmt_repr!(Unit);
impl_display_debug_for_fmt_repr!(Bit);
impl_display_debug_for_fmt_repr!(Key);
impl_display_debug_for_fmt_repr!(Text);
impl_display_debug_for_fmt_repr!(Int);
impl_display_debug_for_fmt_repr!(Decimal);
impl_display_debug_for_fmt_repr!(Byte);
impl_display_debug_for_fmt_repr!(<T> Cell<T>);
impl_display_debug_for_fmt_repr!(<T> Pair<T, T>);
impl_display_debug_for_fmt_repr!(<T> List<T>);
impl_display_debug_for_fmt_repr!(<T> Map<Key, T>);
impl_display_debug_for_fmt_repr!(<T> Quote<T>);
impl_display_debug_for_fmt_repr!(<T> Call<T, T>);

struct Indent<'a> {
    writer: &'a mut dyn Write,
    on_newline: bool,
}

impl<'a> Write for Indent<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for s in s.split_inclusive('\n') {
            if self.on_newline {
                self.writer.write_str("    ")?;
            }
            self.on_newline = s.ends_with('\n');
            self.writer.write_str(s)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if self.on_newline {
            self.writer.write_str("    ")?;
        }
        self.on_newline = c == '\n';
        self.writer.write_char(c)
    }
}

impl<'a> Indent<'a> {
    fn new(writer: &'a mut dyn Write) -> Self {
        Indent { writer, on_newline: true }
    }
}
