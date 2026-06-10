use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::ops::Deref;

use bigdecimal::BigDecimal;
use num_traits::Signed;

use super::BYTE;
use super::DECIMAL;
use super::DOUBLE_QUOTE;
use super::Direction;
use super::EMPTY;
use super::FALSE;
use super::INT;
use super::KEY;
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
use super::SINGLE_QUOTE;
use super::SOLVE;
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
use crate::type_::Solve;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Default, Copy, Clone)]
pub struct FmtOptions {
    /// key encoding, compact, deterministic
    pub id_mode: bool,
    /// quoted, prefixed, no shorthands
    pub normalized: bool,
    /// newline, indent, gap
    pub pretty: bool,
    pub direction: Direction,
    pub key_ctx: bool,
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
        Self { pretty: f.alternate(), ..Default::default() }
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
        if !options.key_ctx {
            f.write_str(KEY)?;
        }
        let mut single = use_single(self);
        f.write_char(if single { SINGLE_QUOTE } else { DOUBLE_QUOTE })?;
        for c in self.chars() {
            if single && c == SINGLE_QUOTE {
                f.write_char(SINGLE_QUOTE)?;
                if options.pretty {
                    f.write_str(EMPTY)?;
                }
                f.write_char(DOUBLE_QUOTE)?;
                single = false;
            } else if !single && c == DOUBLE_QUOTE {
                f.write_char(DOUBLE_QUOTE)?;
                if options.pretty {
                    f.write_str(EMPTY)?;
                }
                f.write_char(SINGLE_QUOTE)?;
                single = true;
            }
            f.write_char(c)?;
        }
        f.write_char(if single { SINGLE_QUOTE } else { DOUBLE_QUOTE })
    }

    fn get_type(&self) -> ReprType {
        ReprType::Key
    }
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
        if options.id_mode || options.normalized || !options.pretty {
            return fmt_many(self, true, options.pretty, f);
        }
        for line in self.split_inclusive('\n') {
            fmt_line(line, f)?;
        }
        if self.is_empty() || self.ends_with('\n') {
            f.write_char('\'')?;
            f.write_char('\'')?;
        }
        Ok(())
    }

    fn get_type(&self) -> ReprType {
        ReprType::Text
    }
}

fn fmt_line(s: &str, f: &mut dyn Write) -> std::fmt::Result {
    let (line, end) = if let Some(line) = s.strip_suffix("\r\n") {
        (line, "\r\n")
    } else if let Some(line) = s.strip_suffix("\n") {
        (line, "\n")
    } else {
        (s, "")
    };
    fmt_many(line, false, true, f)?;
    match end {
        "" => Ok(()),
        "\n" => {
            f.write_char('.')?;
            f.write_char('\n')
        },
        "\r\n" => {
            f.write_char(':')?;
            f.write_char('\n')
        },
        _ => unreachable!(),
    }
}

fn fmt_many(text: &str, key_encoding: bool, pretty: bool, f: &mut dyn Write) -> std::fmt::Result {
    let mut state = if use_single(text) { State::Single } else { State::Double };
    begin_state(state, f)?;
    for (index, c) in text.char_indices() {
        let tail = &text[index + c.len_utf8() ..];
        fmt_char(c, tail, key_encoding, pretty, &mut state, f)?;
    }
    end_state(state, f)
}

fn fmt_char(
    c: char, tail: &str, key_encoding: bool, pretty: bool, state: &mut State, f: &mut dyn Write,
) -> std::fmt::Result {
    if c == SINGLE_QUOTE {
        switch_state(State::Double, pretty, state, f)?;
        return f.write_char(SINGLE_QUOTE);
    }
    if c == DOUBLE_QUOTE {
        switch_state(State::Single, pretty, state, f)?;
        return f.write_char(DOUBLE_QUOTE);
    }
    if Key::is_key(c) {
        switch_quote(tail, pretty, state, f)?;
        return f.write_char(c);
    }
    if let Some(token) = ascii_name(c) {
        switch_state(State::Token, pretty, state, f)?;
        return f.write_str(token);
    }
    if !key_encoding && is_normal(c) {
        switch_quote(tail, pretty, state, f)?;
        return f.write_char(c);
    }
    switch_state(State::Token, pretty, state, f)?;
    f.write_char('X')?;
    write!(f, "{:x}", c as u32)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    Single,
    Double,
    Token,
}

fn switch_quote(
    text: &str, pretty: bool, state: &mut State, f: &mut dyn Write,
) -> std::fmt::Result {
    if *state != State::Token {
        return Ok(());
    }
    let target = if use_single(text) { State::Single } else { State::Double };
    switch_state(target, pretty, state, f)
}

// prefer single quote
fn use_single(s: &str) -> bool {
    for c in s.chars() {
        if c == SINGLE_QUOTE {
            return false;
        }
        if c == DOUBLE_QUOTE {
            return true;
        }
        if !is_normal(c) {
            return true;
        }
    }
    true
}

fn switch_state(
    target: State, pretty: bool, state: &mut State, f: &mut dyn Write,
) -> std::fmt::Result {
    if *state == target {
        if target == State::Token {
            f.write_char(' ')?;
        }
        return Ok(());
    }
    end_state(*state, f)?;
    *state = target;
    if pretty {
        f.write_str(EMPTY)?;
    }
    begin_state(target, f)
}

fn begin_state(state: State, f: &mut dyn Write) -> std::fmt::Result {
    match state {
        State::Single => f.write_char(SINGLE_QUOTE),
        State::Double => f.write_char(DOUBLE_QUOTE),
        State::Token => f.write_char(LIST_LEFT),
    }
}

fn end_state(state: State, f: &mut dyn Write) -> std::fmt::Result {
    match state {
        State::Single => f.write_char(SINGLE_QUOTE),
        State::Double => f.write_char(DOUBLE_QUOTE),
        State::Token => f.write_char(LIST_RIGHT),
    }
}

fn ascii_name(c: char) -> Option<&'static str> {
    let s = match c {
        '\u{00}' => "nul",
        '\u{01}' => "soh",
        '\u{02}' => "stx",
        '\u{03}' => "etx",
        '\u{04}' => "eot",
        '\u{05}' => "enq",
        '\u{06}' => "ack",
        '\u{07}' => "bel",
        '\u{08}' => "bs",
        '\u{09}' => "ht",
        '\u{0A}' => "lf",
        '\u{0B}' => "vt",
        '\u{0C}' => "ff",
        '\u{0D}' => "cr",
        '\u{0E}' => "so",
        '\u{0F}' => "si",
        '\u{10}' => "dle",
        '\u{11}' => "dc1",
        '\u{12}' => "dc2",
        '\u{13}' => "dc3",
        '\u{14}' => "dc4",
        '\u{15}' => "nak",
        '\u{16}' => "syn",
        '\u{17}' => "etb",
        '\u{18}' => "can",
        '\u{19}' => "em",
        '\u{1A}' => "sub",
        '\u{1B}' => "esc",
        '\u{1C}' => "fs",
        '\u{1D}' => "gs",
        '\u{1E}' => "rs",
        '\u{1F}' => "us",
        '\u{20}' => "sp",
        '\u{7F}' => "del",
        _ => {
            return None;
        },
    };
    Some(s)
}

/// Determines whether a character is considered "normal".
///
/// A normal character is defined as:
/// - Independently visible, occupying one printing position.
/// - Does not affect text direction (fixed left-to-right).
/// - Does not combine with other characters.
/// - Has no zero-width or formatting control behavior.
fn is_normal(c: char) -> bool {
    // Included Unicode blocks (all left-to-right, non-combining ordinary characters)
    const INCLUDE_RANGES: &[(u32, u32)] = &[
        // Basic ASCII printable characters (space, digits, letters, common punctuation)
        (0x20, 0x7e),
        // Latin-1 supplement (printable part, except soft hyphen U+00AD)
        (0xa0, 0xac),
        (0xae, 0xff),
        // Latin extended, IPA extensions, modifier letters (standalone symbols)
        (0x100, 0x24f),
        (0x250, 0x2af),
        (0x2b0, 0x2ff),
        // Greek, Cyrillic, Armenian, Georgian
        (0x370, 0x3ff),
        (0x400, 0x52f),
        (0x530, 0x58f),
        (0x10a0, 0x10ff),
        // Japanese kana (Hiragana, Katakana)
        (0x3040, 0x309f),
        (0x30a0, 0x30ff),
        // Korean (Hangul Jamo + full syllables)
        (0x1100, 0x11ff),
        (0x3130, 0x318f),
        (0xac00, 0xd7af),
        // CJK unified ideographs (basic, extension A, compatibility)
        (0x4e00, 0x9fff),
        (0x3400, 0x4dbf),
        (0xf900, 0xfaff),
        // CJK extension B (optional, to support more Hanzi)
        (0x20000, 0x2a6df),
        // Punctuation symbols, currency symbols, letterlike symbols, number forms,
        // arrows, mathematical symbols, miscellaneous technical symbols,
        // control pictures, geometric shapes, dingbats, supplemental punctuation,
        // fullwidth variants
        (0x2000, 0x200a), // various spaces (except zero-width) are visible
        (0x2010, 0x2027), // hyphens, quotes, etc.
        (0x2030, 0x205f), // per mille, numerals, etc.
        (0x20a0, 0x20cf), // currency symbols
        (0x2100, 0x214f), // letterlike symbols
        (0x2150, 0x218f), // number forms
        (0x2190, 0x21ff), // arrows
        (0x2200, 0x22ff), // mathematical symbols
        (0x2300, 0x23ff), // miscellaneous technical symbols
        (0x2400, 0x243f), // control pictures (visible representations of controls)
        (0x25a0, 0x25ff), // geometric shapes
        (0x2700, 0x27bf), // dingbats
        (0x2e00, 0x2e7f), // supplemental punctuation
        (0xff00, 0xffef), // fullwidth variants (excluding FFFE/FFFF)
    ];

    // Characters that must be excluded even if they fall into the above ranges
    const EXCLUDE: &[(u32, u32)] = &[
        // Soft hyphen (invisible control character)
        (0xad, 0xad),
        // Zero-width and direction controls
        (0x200b, 0x200f), // zero-width space, zero-width joiner, left/right marks
        (0x2028, 0x202e), // line separator, paragraph separator, bidi controls
        (0x2060, 0x206f), // invisible connector, variation inhibitors, etc.
        // Combining characters (attach to others)
        (0x0300, 0x036f), // combining diacritical marks
        (0x1ab0, 0x1aff), // combining diacritical marks extended
        (0x20d0, 0x20ff), // combining marks for symbols
        (0xfe00, 0xfe0f), // variation selectors
        (0xfe20, 0xfe2f), // combining half marks
        // Strong right-to-left scripts (break left-to-right layout)
        (0x0590, 0x05ff), // Hebrew
        (0x0600, 0x06ff), // Arabic
        (0x0700, 0x074f), // Syriac
        (0x0750, 0x077f), // Arabic supplement
        (0x08a0, 0x08ff), // Arabic extended
        (0xfb50, 0xfdff), // Arabic presentation forms A
        (0xfe70, 0xfeff), // Arabic presentation forms B
        // Indic and Southeast Asian scripts (rely on combined stacking)
        (0x0900, 0x097f), // Devanagari (contains combining vowels)
        (0x0980, 0x09ff), // Bengali
        (0x0a00, 0x0a7f), // Gurmukhi
        (0x0a80, 0x0aff), // Gujarati
        (0x0b00, 0x0b7f), // Oriya
        (0x0b80, 0x0bff), // Tamil
        (0x0c00, 0x0c7f), // Telugu
        (0x0c80, 0x0cff), // Kannada
        (0x0d00, 0x0d7f), // Malayalam
        (0x0e00, 0x0e7f), // Thai (includes tone marks)
        (0x0e80, 0x0eff), // Lao
        (0x0f00, 0x0fff), // Tibetan (many combining marks)
        // Surrogates, private use, noncharacters
        (0xd800, 0xdfff), // high/low surrogates
        (0xe000, 0xf8ff), // private use area
        (0xfdd0, 0xfdef), // noncharacters
        (0xfffe, 0xffff), // last two noncharacters of each plane
        (0x1fffe, 0x1ffff),
        (0x2fffe, 0x2ffff),
        (0x3fffe, 0x3ffff),
        (0x4fffe, 0x4ffff),
        (0x5fffe, 0x5ffff),
        (0x6fffe, 0x6ffff),
        (0x7fffe, 0x7ffff),
        (0x8fffe, 0x8ffff),
        (0x9fffe, 0x9ffff),
        (0xafffe, 0xaffff),
        (0xbfffe, 0xbffff),
        (0xcfffe, 0xcffff),
        (0xdfffe, 0xdffff),
        (0xefffe, 0xeffff),
        (0xffffe, 0xfffff),
        (0x10fffe, 0x10ffff),
    ];

    let cp = c as u32;
    if !INCLUDE_RANGES.iter().any(|&(s, e)| s <= cp && cp <= e) {
        return false;
    }
    !EXCLUDE.iter().any(|&(s, e)| s <= cp && cp <= e)
}

impl FmtRepr for Int {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        if options.normalized {
            f.write_str(INT)?;
            f.write_char(SINGLE_QUOTE)?;
            int_fmt(self, options, f)?;
            f.write_char(SINGLE_QUOTE)
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
            f.write_char(SINGLE_QUOTE)?;
            decimal_fmt(self, options, f)?;
            f.write_char(SINGLE_QUOTE)
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
    write!(f, "{}", decimal.order_of_magnitude())?;
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
        f.write_char(SINGLE_QUOTE)?;
        if options.normalized {
            f.write_char('X')?;
        }
        for &b in self.iter() {
            write!(f, "{b:02x}")?;
        }
        f.write_char(SINGLE_QUOTE)
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
        fmt_delimited(&self.value, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Quote
    }
}

fn fmt_delimited(repr: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
    if let ReprType::Text | ReprType::List | ReprType::Map = repr.get_type() {
        return repr.fmt(options, f);
    }
    f.write_char(SCOPE_LEFT)?;
    repr.fmt(options, f)?;
    f.write_char(SCOPE_RIGHT)
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
        Direction::Left => fmt_left(EMPTY, func, input, options, f),
        Direction::Right => fmt_right(EMPTY, func, input, options, f),
    }
}

impl<T: FmtRepr> FmtRepr for Solve<T, T> {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        match options.direction {
            Direction::Left => fmt_left(SOLVE, &self.func, &self.output, options, f),
            Direction::Right => fmt_right(SOLVE, &self.func, &self.output, options, f),
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Solve
    }
}

fn fmt_left(
    tag: &str, func: &dyn FmtRepr, argument: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    argument.fmt(options, f)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    f.write_str(tag)
}

fn fmt_right(
    tag: &str, func: &dyn FmtRepr, argument: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
) -> std::fmt::Result {
    f.write_str(tag)?;
    f.write_char(' ')?;
    closure(func, options, f)?;
    f.write_char(' ')?;
    argument.fmt(options, f)
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
    matches!(repr.get_type(), ReprType::Pair | ReprType::Call | ReprType::Solve)
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
        if options.id_mode || !options.pretty {
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
        if options.id_mode {
            let mut pairs: Vec<_> = self.iter().collect();
            pairs.sort_unstable_by_key(|(k, _)| (*k).deref());
            for (key, value) in pairs {
                kv_fmt(key.clone(), value, options, f)?;
                f.write_char(SEPARATOR)?;
            }
        } else if !options.pretty {
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

fn kv_fmt(
    key: Key, value: &dyn FmtRepr, options: FmtOptions, f: &mut dyn Write,
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

impl<T: FmtRepr + ?Sized> FmtRepr for &T {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        (*self).fmt(options, f)
    }

    fn get_type(&self) -> ReprType {
        (*self).get_type()
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        (*self).to_pair()
    }
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
impl_display_debug_for_fmt_repr!(<T> Solve<T, T>);

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
