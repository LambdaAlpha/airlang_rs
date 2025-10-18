use std::hash::Hash;
use std::ops::Neg;
use std::str::FromStr;

use const_format::concatcp;
use num_bigint::BigInt;
use num_traits::Num;
use winnow::ModalResult;
use winnow::Parser;
use winnow::Result;
use winnow::ascii::digit1;
use winnow::ascii::line_ending;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::empty;
use winnow::combinator::fail;
use winnow::combinator::not;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::combinator::terminated;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::token::any;
use winnow::token::one_of;
use winnow::token::take_till;
use winnow::token::take_until;
use winnow::token::take_while;

use super::BYTE;
use super::COMMENT;
use super::Direction;
use super::FALSE;
use super::INT;
use super::LEFT;
use super::LIST_LEFT;
use super::LIST_RIGHT;
use super::MAP_LEFT;
use super::MAP_RIGHT;
use super::NUMBER;
use super::PAIR;
use super::QUOTE;
use super::RIGHT;
use super::SCOPE_LEFT;
use super::SCOPE_RIGHT;
use super::SEPARATOR;
use super::SPACE;
use super::SYMBOL_QUOTE;
use super::TEXT_QUOTE;
use super::TRUE;
use super::UNIT;
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
use crate::utils::conversion::bin_str_to_vec_u8;
use crate::utils::conversion::hex_str_to_vec_u8;

pub trait ParseRepr:
    From<Unit>
    + From<Bit>
    + From<Symbol>
    + From<Text>
    + From<Int>
    + From<Number>
    + From<Byte>
    + From<Pair<Self, Self>>
    + From<Call<Self, Self>>
    + From<List<Self>>
    + Eq
    + Hash
    + From<Map<Self, Self>>
    + Clone {
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct ParseCtx {
    direction: Direction,
}

impl ParseCtx {
    fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}

type E = ErrMode<ContextError>;

pub fn parse<T: ParseRepr>(src: &str) -> Result<T, super::ParseError> {
    top::<T>.parse(src).map_err(|e| super::ParseError { msg: e.to_string() })
}

fn label(label: &'static str) -> StrContext {
    StrContext::Label(label)
}

fn expect_desc(description: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(description))
}

fn expect_char(c: char) -> StrContext {
    StrContext::Expected(StrContextValue::CharLiteral(c))
}

fn top<T: ParseRepr>(src: &mut &str) -> ModalResult<T> {
    let ctx = ParseCtx::default();
    trim_comment(ctx, compose(ctx)).parse_next(src)
}

fn trim<'a, O, F>(f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(opt(spaces), f, opt(spaces))
}

fn trim_comment<'a, O, F>(ctx: ParseCtx, f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(opt(spaces_comment(ctx)), f, opt(spaces_comment(ctx)))
}

fn spaces(i: &mut &str) -> ModalResult<()> {
    let spaces = take_while(1 .., |c| matches!(c, ' ' | '\t' | '\n'));
    repeat(1 .., alt((spaces, "\r\n")).void()).context(label("spaces")).parse_next(i)
}

fn space_tab(i: &mut &str) -> ModalResult<()> {
    take_while(1 .., |c| matches!(c, ' ' | '\t')).void().context(label("space_tab")).parse_next(i)
}

fn spaces_comment<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    repeat(1 .., alt((spaces, comment(ctx))))
}

fn comment<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    let comment_tokens = repeat(0 .., comment_token(ctx));
    let comment = delimited_cut(SCOPE_LEFT, comment_tokens, SCOPE_RIGHT);
    preceded(COMMENT, comment).context(label("comment"))
}

fn comment_token<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    // to avoid error[E0720]: cannot resolve opaque type
    move |i: &mut _| {
        alt((spaces, comment(ctx), SEPARATOR.void(), token::<C>(ctx).void())).parse_next(i)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct C;

macro_rules! impl_parse_repr_for_comment {
    ($($t:ty)*) => {
        $(impl From<$t> for C {
            fn from(_: $t) -> C {
                C
            }
        })*
    };
}

impl_parse_repr_for_comment!(Unit Bit Symbol Text Int Number Byte);
impl_parse_repr_for_comment!(Pair<C, C> Call<C, C> List<C> Map<C, C>);
impl ParseRepr for C {}

fn delimited_cut<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    let left = left.context(expect_char(left));
    let right = right.context(expect_char(right));
    delimited(left, cut_err(f), cut_err(right))
}

fn delimited_trim<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_cut(left, trim(f), right)
}

fn delimited_trim_comment<'a, T, F>(
    ctx: ParseCtx, left: char, f: F, right: char,
) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_cut(left, trim_comment(ctx, f), right)
}

fn scoped<'a, T, F>(f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim(SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scoped_trim_comment<'a, T, F>(ctx: ParseCtx, f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim_comment(ctx, SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scope<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    scoped_trim_comment(ctx, compose(ctx)).context(label("scope"))
}

fn trivial_symbol1<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_trivial_symbol).parse_next(i)
}

fn is_trivial_symbol(c: char) -> bool {
    if is_delimiter(c) {
        return false;
    }
    Symbol::is_symbol(c)
}

fn is_symbol(c: char) -> bool {
    Symbol::is_symbol(c)
}

fn token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<T>, E> {
    (move |i: &mut _| match peek(any).parse_next(i)? {
        LIST_LEFT => list(ctx).map(Token::Default).parse_next(i),
        LIST_RIGHT => fail.parse_next(i),
        MAP_LEFT => map(ctx).map(Token::Default).parse_next(i),
        MAP_RIGHT => fail.parse_next(i),
        SCOPE_LEFT => scope(ctx).map(Token::Default).parse_next(i),
        SCOPE_RIGHT => fail.parse_next(i),
        SEPARATOR => fail.parse_next(i),
        SPACE => fail.parse_next(i),
        TEXT_QUOTE => text.map(T::from).map(Token::Default).parse_next(i),
        SYMBOL_QUOTE => symbol.map(T::from).map(Token::Default).parse_next(i),
        _ => cut_err(ext(ctx)).parse_next(i),
    })
    .context(label("token"))
}

fn ext<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<T>, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        if matches!(i.chars().next(), Some('0' ..= '9')) {
            let int_or_number = int_or_number.map(Token::Default);
            let end = not(one_of(is_trivial_symbol));
            return cut_err(terminated(int_or_number, end).context(label("int or number")))
                .parse_next(i);
        }
        let symbol = trivial_symbol1.context(label("symbol")).parse_next(i)?;
        if i.starts_with(LEFT_DELIMITERS) {
            return prefix(symbol, ctx).map(Token::Default).parse_next(i);
        }
        if let Some(keyword) = keyword(symbol) {
            return Ok(Token::Default(keyword));
        }
        Ok(Token::Unquote(Symbol::from_str_unchecked(symbol)))
    }
}

const LEFT_DELIMITERS: [char; 5] = [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, SYMBOL_QUOTE, TEXT_QUOTE];

fn keyword<T: ParseRepr>(s: &str) -> Option<T> {
    match s {
        UNIT => Some(T::from(Unit)),
        TRUE => Some(T::from(Bit::true_())),
        FALSE => Some(T::from(Bit::false_())),
        _ => None,
    }
}

fn prefix<'a, T: ParseRepr>(prefix: &str, ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        match prefix {
            UNIT => match i.chars().next().unwrap() {
                LIST_LEFT => raw_list(ctx).parse_next(i),
                MAP_LEFT => raw_map(ctx).parse_next(i),
                _ => fail.context(label("prefix token")).parse_next(i),
            },
            INT => int.map(T::from).parse_next(i),
            NUMBER => number.map(T::from).parse_next(i),
            BYTE => byte.map(T::from).parse_next(i),
            LEFT => scope(ctx.direction(Direction::Left)).parse_next(i),
            RIGHT => scope(ctx.direction(Direction::Right)).parse_next(i),
            _ => cut_err(fail.context(label("prefix token"))).parse_next(i),
        }
    }
}

#[derive(Clone)]
enum Token<T> {
    Unquote(Symbol),
    Default(T),
}

impl<T: ParseRepr> Token<T> {
    fn into_repr(self) -> T {
        match self {
            Token::Unquote(s) => T::from(s),
            Token::Default(r) => r,
        }
    }
}

fn repr<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    token(ctx).map(Token::into_repr)
}

fn compose_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<T>, E> {
    move |i: &mut _| match ctx.direction {
        Direction::Left => compose_left(ctx).parse_next(i),
        Direction::Right => compose_right(ctx).parse_next(i),
    }
}

fn compose_left<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<T>, E> {
    move |i: &mut _| {
        let mut left = token(ctx).parse_next(i)?;
        let mut next = preceded(spaces_comment(ctx), token(ctx));
        let last = &mut preceded(spaces_comment(ctx), token(ctx));
        loop {
            let Some(func) = opt(next.by_ref()).parse_next(i)? else {
                return Ok(left);
            };
            left = compose_one(ctx, i, &mut next, last, left, func)?;
        }
    }
}

fn compose_right<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<T>, E> {
    move |i: &mut _| {
        let left = token(ctx).parse_next(i)?;
        let mut next = preceded(spaces_comment(ctx), token(ctx));
        let last = &mut preceded(spaces_comment(ctx), compose_right(ctx));
        let Some(func) = opt(next.by_ref()).parse_next(i)? else {
            return Ok(left);
        };
        compose_one(ctx, i, &mut next, last, left, func)
    }
}

#[allow(clippy::single_match_else)]
fn compose_one<'a, T: ParseRepr>(
    ctx: ParseCtx, i: &mut &'a str, _next: &mut dyn Parser<&'a str, Token<T>, E>,
    last: &mut dyn Parser<&'a str, Token<T>, E>, left: Token<T>, func: Token<T>,
) -> ModalResult<Token<T>> {
    let repr = match func {
        Token::Unquote(s) => match &*s {
            PAIR => {
                let right = last.parse_next(i)?;
                T::from(Pair::new(left.into_repr(), right.into_repr()))
            }
            _ => {
                let right = last.parse_next(i)?;
                infix(ctx, left, T::from(s), right)
            }
        },
        Token::Default(func) => {
            let right = last.parse_next(i)?;
            infix(ctx, left, func, right)
        }
    };
    Ok(Token::Default(repr))
}

fn infix<T: ParseRepr>(_ctx: ParseCtx, left: Token<T>, func: T, right: Token<T>) -> T {
    let input = left_right(left, right);
    T::from(Call { func, input })
}

fn left_right<T: ParseRepr>(left: Token<T>, right: Token<T>) -> T {
    let left = match left {
        Token::Unquote(s) if *s == *COMMENT => None,
        left => Some(left.into_repr()),
    };
    let right = match right {
        Token::Unquote(s) if *s == *COMMENT => None,
        right => Some(right.into_repr()),
    };
    match (left, right) {
        (Some(left), Some(right)) => T::from(Pair::new(left, right)),
        (Some(left), None) => left,
        (None, Some(right)) => right,
        (None, None) => T::from(Unit),
    }
}

fn compose<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    compose_token(ctx).map(Token::into_repr)
}

fn list<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut list = Vec::new();
        let mut repr = opt(compose(ctx));
        let mut separator = opt(trim_comment(ctx, SEPARATOR.context(expect_char(SEPARATOR))));
        loop {
            let Some(item) = repr.parse_next(i)? else {
                break;
            };
            list.push(item);
            if separator.parse_next(i)?.is_none() {
                break;
            }
        }
        Ok(T::from(List::from(list)))
    };
    delimited_trim_comment(ctx, LIST_LEFT, items, LIST_RIGHT).context(label("list"))
}

fn raw_list<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let repr_list = separated(0 .., repr::<T>(ctx), spaces_comment(ctx));
    delimited_trim_comment(ctx, LIST_LEFT, repr_list, LIST_RIGHT)
        .map(|tokens: Vec<_>| T::from(List::from(tokens)))
        .context(label("raw list"))
}

fn map<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut map = Map::default();
        let mut key = opt(repr(ctx));
        let mut pair = opt(preceded(spaces_comment(ctx), PAIR.void()));
        let mut value = cut_err(preceded(
            spaces_comment(ctx).context(expect_desc("space")),
            compose(ctx).context(expect_desc("value")),
        ));
        let mut separator = opt(trim_comment(ctx, SEPARATOR.context(expect_char(SEPARATOR))));
        let mut duplicate = fail.context(expect_desc("no duplicate keys"));
        loop {
            let Some(k) = key.parse_next(i)? else {
                break;
            };
            if map.contains_key(&k) {
                return duplicate.parse_next(i);
            }
            let v = if pair.parse_next(i).unwrap().is_none() {
                T::from(Unit)
            } else {
                value.parse_next(i)?
            };
            map.insert(k, v);
            if separator.parse_next(i)?.is_none() {
                break;
            }
        }
        Ok(T::from(map))
    };
    delimited_trim_comment(ctx, MAP_LEFT, items, MAP_RIGHT).context(label("map"))
}

fn raw_map<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let tokens: Vec<_> = separated(0 .., repr::<T>(ctx), spaces_comment(ctx)).parse_next(i)?;
        if !tokens.len().is_multiple_of(2) {
            return cut_err(fail.context(expect_desc("even number of tokens"))).parse_next(i);
        }
        let mut map = Map::with_capacity(tokens.len() / 2);
        let mut tokens = tokens.into_iter();
        while let Some(key) = tokens.next() {
            let value = tokens.next().unwrap();
            map.insert(key, value);
        }
        Ok(T::from(map))
    };
    delimited_trim_comment(ctx, MAP_LEFT, items, MAP_RIGHT).context(label("raw map"))
}

fn symbol(i: &mut &str) -> ModalResult<Symbol> {
    let symbol = move |i: &mut _| {
        let mut s = String::new();
        let mut literal = take_while(1 .., |c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
        let mut raw_literal = take_while(0 .., is_symbol);
        let mut raw = false;
        loop {
            if raw {
                match peek(any).parse_next(i)? {
                    '\r' | '\n' => {
                        symbol_newline.parse_next(i)?;
                        match any.parse_next(i)? {
                            SCOPE_RIGHT => raw = false,
                            ' ' => {}
                            _ => return fail.parse_next(i),
                        }
                    }
                    _ => s.push_str(raw_literal.parse_next(i)?),
                }
            } else {
                match peek(any).parse_next(i)? {
                    SYMBOL_QUOTE => break,
                    '\\' => s.push_str(symbol_escaped.parse_next(i)?),
                    '\r' | '\n' => {
                        symbol_newline.parse_next(i)?;
                        match any.parse_next(i)? {
                            SCOPE_LEFT => raw = true,
                            ' ' => {}
                            _ => return fail.parse_next(i),
                        }
                    }
                    _ => s.push_str(literal.parse_next(i)?),
                }
            }
        }
        Ok(Symbol::from_string_unchecked(s))
    };
    delimited_cut(SYMBOL_QUOTE, symbol, SYMBOL_QUOTE).context(label("symbol")).parse_next(i)
}

fn symbol_escaped<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    preceded('\\', move |i: &mut _| match any.parse_next(i)? {
        '\\' => empty.value("\\").parse_next(i),
        '_' => empty.value(" ").parse_next(i),
        QUOTE => empty.value(concatcp!(SYMBOL_QUOTE)).parse_next(i),
        ' ' | '\t' => opt(space_tab).value("").parse_next(i),
        _ => fail.parse_next(i),
    })
    .context(expect_desc("escape character"))
    .parse_next(i)
}

fn symbol_newline(i: &mut &str) -> ModalResult<()> {
    (line_ending, opt(space_tab), '|'.context(expect_char('|')))
        .void()
        .context(expect_desc("newline"))
        .parse_next(i)
}

fn text(i: &mut &str) -> ModalResult<Text> {
    let text = move |i: &mut _| {
        let i: &mut &str = i;
        let mut s = String::new();
        let mut literal = take_till(1 .., |c| matches!(c, '"' | '\\' | '\r' | '\n'));
        let mut raw_literal = take_until(1 .., ('\r', '\n'));
        let mut raw = false;
        loop {
            if raw {
                match peek(any).parse_next(i)? {
                    c @ ('\r' | '\n') => {
                        if c == '\r' && !i.starts_with("\r\n") {
                            s.push('\r'.parse_next(i)?);
                            continue;
                        }
                        s.push_str(text_newline.parse_next(i)?);
                        match any.parse_next(i)? {
                            SCOPE_RIGHT => raw = false,
                            ' ' => {}
                            _ => return fail.parse_next(i),
                        }
                    }
                    _ => s.push_str(raw_literal.parse_next(i)?),
                }
            } else {
                match peek(any).parse_next(i)? {
                    TEXT_QUOTE => break,
                    '\\' => text_escaped.parse_next(i)?.push(&mut s),
                    c @ ('\r' | '\n') => {
                        if c == '\r' && !i.starts_with("\r\n") {
                            s.push('\r'.parse_next(i)?);
                            continue;
                        }
                        s.push_str(text_newline.parse_next(i)?);
                        match any.parse_next(i)? {
                            SCOPE_LEFT => raw = true,
                            ' ' => {}
                            _ => return fail.parse_next(i),
                        }
                    }
                    _ => s.push_str(literal.parse_next(i)?),
                }
            }
        }
        Ok(Text::from(s))
    };
    delimited_cut(TEXT_QUOTE, text, TEXT_QUOTE).context(label("text")).parse_next(i)
}

fn text_escaped<'a>(i: &mut &'a str) -> ModalResult<StrFragment<'a>> {
    preceded('\\', move |i: &mut _| match any.parse_next(i)? {
        'u' => unicode.map(StrFragment::Char).parse_next(i),
        'n' => empty.value(StrFragment::Char('\n')).parse_next(i),
        'r' => empty.value(StrFragment::Char('\r')).parse_next(i),
        't' => empty.value(StrFragment::Char('\t')).parse_next(i),
        '\\' => empty.value(StrFragment::Char('\\')).parse_next(i),
        '_' => empty.value(StrFragment::Char(' ')).parse_next(i),
        QUOTE => empty.value(StrFragment::Char(TEXT_QUOTE)).parse_next(i),
        ' ' | '\t' => opt(space_tab).value(StrFragment::Str("")).parse_next(i),
        _ => fail.parse_next(i),
    })
    .context(expect_desc("escape character"))
    .parse_next(i)
}

fn unicode(i: &mut &str) -> ModalResult<char> {
    let digit = take_while(1 .. 7, is_hexadecimal);
    scoped(digit)
        .map(move |hex| u32::from_str_radix(hex, 16).unwrap())
        .verify_map(std::char::from_u32)
        .context(expect_desc("unicode"))
        .parse_next(i)
}

fn text_newline<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    let newline = alt(('+'.value(true), '|'.value(false)))
        .context(expect_char('+'))
        .context(expect_char('|'));
    (line_ending, opt(space_tab), newline)
        .map(|(ending, _, newline): (&str, _, _)| if newline { ending } else { "" })
        .context(expect_desc("newline"))
        .parse_next(i)
}

#[derive(Clone)]
enum StrFragment<'a> {
    Str(&'a str),
    Char(char),
}

impl StrFragment<'_> {
    fn push(self, str: &mut String) {
        match self {
            StrFragment::Str(s) => str.push_str(s),
            StrFragment::Char(c) => str.push(c),
        }
    }
}

fn int_or_number<T: ParseRepr>(i: &mut &str) -> ModalResult<T> {
    let norm = preceded('0', (sign, significand, opt(exponent)));
    let short = (empty.value(true), significand_radix(10, digit1, "decimal"), opt(exponent));
    alt((norm, short))
        .map(|(sign, significand, exponent)| build_int_or_number(sign, significand, exponent))
        .context(label("int or number"))
        .parse_next(i)
}

// todo design support spaces
fn int(i: &mut &str) -> ModalResult<Int> {
    let int = (sign, integral).map(|(sign, i)| build_int(sign, i));
    scoped(int).context(label("int")).parse_next(i)
}

// todo design support spaces
fn number(i: &mut &str) -> ModalResult<Number> {
    let number = (sign, significand, opt(exponent))
        .map(|(sign, significand, exponent)| build_number(sign, significand, exponent));
    scoped(number).context(label("number")).parse_next(i)
}

fn trim_num0<'a, F>(f: F) -> impl Parser<&'a str, String, E>
where F: Parser<&'a str, &'a str, E> {
    separated(0 .., f, COMMENT).map(|s: Vec<&str>| s.join(""))
}

fn trim_num1<'a, F>(f: F) -> impl Parser<&'a str, String, E>
where F: Parser<&'a str, &'a str, E> {
    separated(1 .., f, COMMENT).map(|s: Vec<&str>| s.join(""))
}

fn sign(i: &mut &str) -> ModalResult<bool> {
    alt(('+'.value(true), '-'.value(false), empty.value(true))).parse_next(i)
}

fn integral(i: &mut &str) -> ModalResult<BigInt> {
    let dec_no_tag = int_radix(10, digit1, "decimal");
    let hex = preceded('X', cut_err(int_radix(16, hexadecimal1, "hexadecimal")));
    let bin = preceded('B', cut_err(int_radix(2, binary1, "binary")));
    let dec = preceded('D', cut_err(int_radix(10, digit1, "decimal")));

    alt((dec_no_tag, hex, bin, dec)).context(label("integral")).parse_next(i)
}

fn int_radix<'a, F>(radix: u8, f: F, desc: &'static str) -> impl Parser<&'a str, BigInt, E>
where F: Parser<&'a str, &'a str, E> {
    trim_num1(f)
        .map(move |int| BigInt::from_str_radix(&int, radix as u32).unwrap())
        .context(expect_desc(desc))
}

struct Significand {
    int: BigInt,
    radix: u8,
    shift: Option<usize>,
}

fn significand(i: &mut &str) -> ModalResult<Significand> {
    let dec_no_tag = significand_radix(10, digit1, "decimal");
    let hex = preceded('X', cut_err(significand_radix(16, hexadecimal1, "hexadecimal")));
    let bin = preceded('B', cut_err(significand_radix(2, binary1, "binary")));
    let dec = preceded('D', cut_err(significand_radix(10, digit1, "decimal")));

    alt((dec_no_tag, hex, bin, dec)).context(label("significand")).parse_next(i)
}

fn significand_radix<'a, F>(
    radix: u8, mut f: F, desc: &'static str,
) -> impl Parser<&'a str, Significand, E>
where F: Parser<&'a str, &'a str, E> {
    (move |i: &mut _| {
        let int = trim_num1(f.by_ref()).parse_next(i)?;
        let fraction =
            opt(preceded('.', cut_err(trim_num0(f.by_ref()).context(expect_desc("fraction")))))
                .parse_next(i)?;
        Ok(build_significand(radix, int, fraction))
    })
    .context(expect_desc(desc))
}

fn build_significand(radix: u8, int: String, fraction: Option<String>) -> Significand {
    if let Some(fraction) = fraction {
        let sig = format!("{int}{fraction}");
        let int = BigInt::from_str_radix(&sig, radix as u32).unwrap();
        let shift = Some(fraction.len());
        Significand { int, radix, shift }
    } else {
        let int = BigInt::from_str_radix(&int, radix as u32).unwrap();
        Significand { int, radix, shift: None }
    }
}

fn exponent(i: &mut &str) -> ModalResult<BigInt> {
    let exp = (sign, trim_num1(digit1)).map(|(sign, exp)| build_exponent(sign, exp));
    preceded('E', cut_err(exp)).context(label("exponent")).parse_next(i)
}

fn build_exponent(sign: bool, exp: String) -> BigInt {
    let i = BigInt::from_str(&exp).unwrap();
    if sign { i } else { i.neg() }
}

fn build_int(sign: bool, i: BigInt) -> Int {
    Int::new(if sign { i } else { i.neg() })
}

fn build_number(sign: bool, significand: Significand, exp: Option<BigInt>) -> Number {
    let int = significand.int;
    let int = if sign { int } else { int.neg() };
    let shift = significand.shift.unwrap_or(0);
    let exp = exp.unwrap_or_default() - shift;
    Number::new(int, significand.radix, exp)
}

fn build_int_or_number<T: ParseRepr>(
    sign: bool, significand: Significand, exp: Option<BigInt>,
) -> T {
    if significand.shift.is_some() || exp.is_some() {
        T::from(build_number(sign, significand, exp))
    } else {
        T::from(build_int(sign, significand.int))
    }
}

// todo design support spaces
fn byte(i: &mut &str) -> ModalResult<Byte> {
    let hex = preceded('X', cut_err(hexadecimal_byte));
    let bin = preceded('B', cut_err(binary_byte));
    let byte = alt((hex, bin, hexadecimal_byte));
    scoped(byte).context(label("byte")).parse_next(i)
}

fn hexadecimal_byte(i: &mut &str) -> ModalResult<Byte> {
    let digits = hexadecimal1.verify(|s: &str| s.len().is_multiple_of(2));
    trim_num0(digits)
        .map(|s| Byte::from(hex_str_to_vec_u8(&s).unwrap()))
        .context(expect_desc("hexadecimal"))
        .parse_next(i)
}

fn binary_byte(i: &mut &str) -> ModalResult<Byte> {
    let digits = binary1.verify(|s: &str| s.len().is_multiple_of(8));
    trim_num0(digits)
        .map(|s| Byte::from(bin_str_to_vec_u8(&s).unwrap()))
        .context(expect_desc("binary"))
        .parse_next(i)
}

fn hexadecimal1<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_hexadecimal).parse_next(i)
}

fn is_hexadecimal(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

fn binary1<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_binary).parse_next(i)
}

fn is_binary(c: char) -> bool {
    matches!(c, '0' ..= '1')
}
