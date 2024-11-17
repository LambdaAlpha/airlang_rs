use std::{
    hash::Hash,
    ops::Neg,
    str::FromStr,
};

use nom::{
    Finish,
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{
        tag,
        take_while,
        take_while_m_n,
        take_while1,
    },
    character::complete::{
        anychar,
        char as char1,
        digit1,
        multispace0,
        multispace1,
    },
    combinator::{
        all_consuming,
        cut,
        fail,
        map,
        map_opt,
        opt,
        peek,
        recognize,
        success,
        value,
        verify,
    },
    error::{
        ContextError,
        ParseError,
        VerboseError,
        context,
        convert_error,
    },
    multi::{
        fold_many0,
        many0,
        separated_list0,
        separated_list1,
    },
    sequence::{
        delimited,
        preceded,
        terminated,
        tuple,
    },
};
use num_bigint::BigInt;
use num_traits::Num;

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
        INT,
        LEFT,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        MIDDLE,
        NUMBER,
        PAIR,
        RAW,
        RICH,
        RIGHT,
        SCOPE_LEFT,
        SCOPE_RIGHT,
        SEPARATOR,
        SPACE,
        SYMBOL_QUOTE,
        TEXT_QUOTE,
        TRUE,
        UNIT,
        is_delimiter,
    },
    text::Text,
    unit::Unit,
    utils,
};

pub(crate) trait ParseRepr:
    From<Unit>
    + From<Bool>
    + From<Int>
    + From<Number>
    + From<Byte>
    + From<Symbol>
    + From<Text>
    + From<Pair<Self, Self>>
    + From<Call<Self, Self>>
    + From<Ask<Self, Self>>
    + From<List<Self>>
    + Eq
    + Hash
    + From<Map<Self, Self>>
    + From<Adapt<Self, Self>>
    + Clone
{
}

pub(crate) fn parse<T: ParseRepr>(src: &str) -> Result<T, crate::syntax::ParseError> {
    let ret = top::<T, VerboseError<&str>>(src).finish();
    match ret {
        Ok(r) => Ok(r.1),
        Err(e) => {
            let msg = convert_error(src, e);
            Err(crate::syntax::ParseError { msg })
        }
    }
}

fn top<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = all_consuming(trim(compose::<2, A_RIGHT, TYPE_CALL, _, _>));
    context("top", f)(src)
}

fn trim<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(empty0, f, empty0)
}

fn empty0<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while(is_empty)(src)
}

fn empty1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while1(is_empty)(src)
}

fn is_empty(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn delimited_cut<'a, T, E, F>(
    left: char,
    f: F,
    right: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, T, E>,
{
    delimited(char1(left), cut(f), cut(char1(right)))
}

fn delimited_trim<'a, T, E, F>(
    left: char,
    f: F,
    right: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, T, E>,
{
    delimited_cut(left, trim(f), right)
}

const A_RIGHT: u8 = 0;
const A_LEFT: u8 = 1;
const A_NONE: u8 = 2;

const TYPE_CALL: u8 = 0;
const TYPE_ASK: u8 = 1;

fn scope<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = delimited_trim(SCOPE_LEFT, compose::<N, A, TYPE, _, _>, SCOPE_RIGHT);
    context("scope", f)(src)
}

fn token<'a, const N: u8, const A: u8, const TYPE: u8, T: ParseRepr, E>(
    src: &'a str,
) -> IResult<&'a str, Token<T>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let (src, first) = peek(anychar)(src)?;

    let parser = match first {
        // delimiters
        LIST_LEFT => |s| map(list1::<N, A, TYPE, _, _>, Token::Default)(s),
        LIST_RIGHT => fail,
        MAP_LEFT => |s| map(map1::<N, A, TYPE, _, _>, Token::Default)(s),
        MAP_RIGHT => fail,
        SCOPE_LEFT => |s| map(scope::<N, A, TYPE, _, _>, Token::Default)(s),
        SCOPE_RIGHT => fail,
        SEPARATOR => fail,
        SPACE => fail,
        TEXT_QUOTE => |s| map(rich_text, Token::Default)(s),
        SYMBOL_QUOTE => |s| map(rich_symbol, Token::Default)(s),

        s if is_symbol(s) => trivial_symbol::<N, A, TYPE, _, _>,
        _ => fail,
    };
    context("token", parser)(src)
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

fn trivial_symbol<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(
    src: &'a str,
) -> IResult<&'a str, Token<T>, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let (rest, s) = take_while(is_trivial_symbol)(src)?;

    // the only special case
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if first.is_ascii_digit() {
        let (_, token) = all_consuming(int_or_number)(s)?;
        return Ok((rest, Token::Default(token)));
    }

    let parser = |src| match s {
        UNIT => alt((scope::<1, A, TYPE, _, _>, success(From::from(Unit))))(src),
        TRUE => success(From::from(Bool::t()))(src),
        FALSE => success(From::from(Bool::f()))(src),
        MIDDLE => scope::<N, A_NONE, TYPE, _, _>(src),
        LEFT => scope::<N, A_LEFT, TYPE, _, _>(src),
        RIGHT => scope::<N, A_RIGHT, TYPE, _, _>(src),
        CALL => scope::<N, A, TYPE_CALL, _, _>(src),
        ASK => scope::<N, A, TYPE_ASK, _, _>(src),
        PAIR => scope::<2, A, TYPE, _, _>(src),
        INT => int(src),
        NUMBER => number(src),
        BYTE => byte(src),
        RICH => alt((rich_text, rich_symbol))(src),
        RAW => alt((raw_text, raw_symbol))(src),
        _ => fail(src),
    };
    let mut f = alt((
        map(parser, Token::Default),
        map(success(Symbol::from_str(s)), Token::Unquote),
    ));
    f(rest)
}

#[derive(Clone)]
enum Token<T> {
    Unquote(Symbol),
    Default(T),
}

impl<T: ParseRepr> Token<T> {
    fn into_repr(self) -> T {
        match self {
            Token::Unquote(s) => From::from(s),
            Token::Default(r) => r,
        }
    }
}

fn compose<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(
    src: &'a str,
) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let tokens = separated_list1(empty1, token::<N, A, TYPE, _, _>);
    let f = map_opt(tokens, |tokens| {
        compose_tokens::<N, A, TYPE, _, _>(tokens.into_iter())
    });
    context("compose", f)(src)
}

fn compose_tokens<const N: u8, const A: u8, const TYPE: u8, T, I>(mut tokens: I) -> Option<T>
where
    T: ParseRepr,
    I: ExactSizeIterator<Item = Token<T>> + DoubleEndedIterator<Item = Token<T>>,
{
    let len = tokens.len();
    if len == 0 {
        return None;
    }
    if len == 1 {
        let repr = tokens.next().unwrap().into_repr();
        return Some(repr);
    }
    if A == A_NONE {
        return compose_none(tokens);
    }
    if len == 2 {
        let func = tokens.next().unwrap().into_repr();
        let input = tokens.next().unwrap().into_repr();
        return Some(compose_two::<TYPE, _>(func, input));
    }
    if N == 2 {
        if len % 2 == 0 {
            return None;
        }
        return if A == A_RIGHT {
            compose_many2::<A_RIGHT, TYPE, _, _>(tokens.rev())
        } else {
            compose_many2::<A_LEFT, TYPE, _, _>(tokens)
        };
    }
    if N == 1 {
        return if A == A_RIGHT {
            compose_many1::<A_RIGHT, TYPE, _, _>(tokens.rev())
        } else {
            compose_many1::<A_LEFT, TYPE, _, _>(tokens)
        };
    }
    None
}

fn compose_none<T, I>(tokens: I) -> Option<T>
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>,
{
    let list = tokens.map(Token::into_repr).collect::<List<_>>();
    let list = From::from(list);
    let tag = From::from(Symbol::from_str(MIDDLE));
    let adapt = From::from(Adapt::new(tag, list));
    Some(adapt)
}

fn compose_two<const TYPE: u8, T: ParseRepr>(left: T, right: T) -> T {
    if TYPE == TYPE_CALL {
        From::from(Call::new(left, right))
    } else {
        From::from(Ask::new(left, right))
    }
}

fn compose_many1<const A: u8, const TYPE: u8, T, I>(mut iter: I) -> Option<T>
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>,
{
    let mut first = iter.next().unwrap().into_repr();
    loop {
        let Some(second) = iter.next() else {
            break;
        };
        first = if A == A_RIGHT {
            compose_two::<TYPE, _>(second.into_repr(), first)
        } else {
            compose_two::<TYPE, _>(first, second.into_repr())
        };
    }
    Some(first)
}

fn compose_many2<const A: u8, const TYPE: u8, T, I>(mut iter: I) -> Option<T>
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>,
{
    let mut first = iter.next().unwrap();
    loop {
        let Some(middle) = iter.next() else {
            break;
        };
        let last = iter.next()?;
        let (left, right) = if A == A_RIGHT {
            left_right(last.into_repr(), first)
        } else {
            left_right(first.into_repr(), last)
        };
        first = Token::Default(compose_infix::<TYPE, _>(left, middle, right));
    }
    Some(first.into_repr())
}

fn compose_infix<const TYPE: u8, T: ParseRepr>(left: T, middle: Token<T>, right: T) -> T {
    let middle = match middle {
        Token::Unquote(s) => match &*s {
            PAIR => return From::from(Pair::new(left, right)),
            CALL => return From::from(Call::new(left, right)),
            ASK => return From::from(Ask::new(left, right)),
            ADAPT => return From::from(Adapt::new(left, right)),
            _ => From::from(s),
        },
        Token::Default(middle) => middle,
    };
    let pair = Pair::new(left, right);
    let pair = From::from(pair);
    compose_two::<TYPE, _>(middle, pair)
}

fn left_right<T: ParseRepr>(left: T, right: Token<T>) -> (T, T) {
    if let Token::Unquote(s) = &right {
        if &**s == PAIR {
            return (left.clone(), left);
        }
    }
    (left, right.into_repr())
}

fn items<'a, O1, O2, E, S, F>(
    item: F,
    separator: S,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O2>, E>
where
    E: ParseError<&'a str>,
    S: Parser<&'a str, O1, E>,
    F: Parser<&'a str, O2, E> + Clone,
{
    let items = many0(terminated(item.clone(), trim(separator)));
    map(tuple((items, opt(item))), |(mut items, last)| {
        if let Some(last) = last {
            items.push(last);
        }
        items
    })
}

fn list1<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let items = items(compose::<N, A, TYPE, _, _>, char1(SEPARATOR));
    let delimited_items = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
    let f = map(delimited_items, |list| From::from(List::from(list)));
    context("list", f)(src)
}

fn map1<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let items = items(key_value::<N, A, TYPE, _, _>, char1(SEPARATOR));
    let delimited_items = delimited_trim(MAP_LEFT, items, MAP_RIGHT);
    let f = map(delimited_items, |pairs| From::from(Map::from_iter(pairs)));
    context("map", f)(src)
}

fn key_value<'a, const N: u8, const A: u8, const TYPE: u8, T, E>(
    src: &'a str,
) -> IResult<&'a str, (T, T), E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let (rest, tokens) = separated_list1(empty1, token::<N, A, TYPE, _, _>)(src)?;
    let mut tokens = tokens.into_iter();
    let key: T = tokens.next().unwrap().into_repr();
    if tokens.len() == 0 {
        return Ok((rest, (key, From::from(Unit))));
    }
    let Token::Unquote(s) = tokens.next().unwrap() else {
        return fail(src);
    };
    if &*s != PAIR {
        return fail(src);
    }
    if A != A_NONE && tokens.len() == 1 {
        let value = tokens.next().unwrap();
        return Ok((rest, left_right(key, value)));
    }
    let Some(value) = compose_tokens::<N, A, TYPE, _, _>(tokens) else {
        return fail(src);
    };
    Ok((rest, (key, value)))
}

fn rich_symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while1(|c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let fragment = alt((
        map(literal, StrFragment::Literal),
        map(symbol_escaped, StrFragment::Escaped),
        map(symbol_space, StrFragment::Space),
    ));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        fragment.push(&mut string);
        string
    });
    let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    let f = map(delimited_symbol, |s| From::from(Symbol::from_string(s)));
    context("rich_symbol", f)(src)
}

fn symbol_escaped<'a, E>(src: &'a str) -> IResult<&'a str, Option<char>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = preceded(
        char1('\\'),
        alt((
            value(Some('\\'), char1('\\')),
            value(Some(' '), char1('_')),
            value(Some(SYMBOL_QUOTE), char1(SYMBOL_QUOTE)),
            value(None, multispace1),
        )),
    );
    context("symbol_escaped", f)(src)
}

// ignore spaces following \n
fn symbol_space<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let empty = &src[0..0];
    let f = alt((
        value(empty, preceded(char1('\n'), multispace0)),
        value(empty, char1('\r')),
        value(empty, take_while1(|c| c == '\t')),
    ));
    context("symbol_space", f)(src)
}

fn raw_symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while(is_symbol);
    let fragment = separated_list1(char1(' '), terminated(literal, raw_symbol_newline));
    let collect_fragments = map(fragment, |fragments| fragments.join(""));
    let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    let f = map(delimited_symbol, |s| From::from(Symbol::from_string(s)));
    context("raw_symbol", f)(src)
}

fn raw_symbol_newline<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let newline = tuple((opt(char1('\r')), char1('\n')));
    let space = take_while(|c| matches!(c, ' ' | '\t'));
    let f = value((), tuple((newline, space, char1('|'))));
    context("raw_symbol_newline", f)(src)
}

fn rich_text<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while1(|c| !matches!(c, '"' | '\\' | '\n'));
    let space = terminated(tag("\n"), multispace0);
    let fragment = alt((
        map(literal, StrFragment::Literal),
        map(text_escaped, StrFragment::Escaped),
        map(space, StrFragment::Space),
    ));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        fragment.push(&mut string);
        string
    });
    let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
    let f = map(delimited_text, |s| From::from(Text::from(s)));
    context("rich_text", f)(src)
}

fn text_escaped<'a, E>(src: &'a str) -> IResult<&'a str, Option<char>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = preceded(
        char1('\\'),
        alt((
            map(unicode, Some),
            value(Some('\n'), char1('n')),
            value(Some('\r'), char1('r')),
            value(Some('\t'), char1('t')),
            value(Some('\\'), char1('\\')),
            value(Some(' '), char1('_')),
            value(Some(TEXT_QUOTE), char1(TEXT_QUOTE)),
            value(None, multispace1),
        )),
    );
    context("text_escaped", f)(src)
}

fn unicode<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let digit = take_while_m_n(1, 6, is_hexadecimal);
    let delimited_digit = preceded(char1('u'), delimited_trim(SCOPE_LEFT, digit, SCOPE_RIGHT));
    let code = map(delimited_digit, move |hex| {
        u32::from_str_radix(hex, 16).unwrap()
    });
    let f = map_opt(code, std::char::from_u32);
    context("unicode", f)(src)
}

fn raw_text<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while(|c| !matches!(c, '\r' | '\n'));
    let fragment = separated_list1(char1(' '), tuple((literal, raw_text_newline)));
    let collect_fragments = map(fragment, |fragments| {
        let mut s = String::new();
        for (literal, newline) in fragments {
            s.push_str(literal);
            s.push_str(newline);
        }
        s
    });
    let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
    let f = map(delimited_text, |s| From::from(Text::from(s)));
    context("raw_text", f)(src)
}

fn raw_text_newline<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let physical = recognize(tuple((opt(char1('\r')), char1('\n'))));
    let space = take_while(|c| matches!(c, ' ' | '\t'));
    let logical = alt((value(true, char1('-')), value(false, char1('|'))));
    let f = map(
        tuple((physical, space, logical)),
        |(physical, _, logical): (&str, _, _)| {
            if logical { physical } else { &physical[0..0] }
        },
    );
    context("raw_text_newline", f)(src)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StrFragment<'a> {
    Literal(&'a str),
    Escaped(Option<char>),
    Space(&'a str),
}

impl<'a> StrFragment<'a> {
    fn push(self, str: &mut String) {
        match self {
            StrFragment::Literal(s) => str.push_str(s),
            StrFragment::Escaped(c) => {
                if let Some(c) = c {
                    str.push(c);
                }
            }
            StrFragment::Space(s) => str.push_str(s),
        }
    }
}

fn int_or_number<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let norm = preceded(tag("0"), tuple((sign, significand, exponent)));
    let short = tuple((success(true), significand_radix(10, digit1), exponent));
    let f = map(alt((norm, short)), |(sign, significand, exponent)| {
        build_int_or_number(sign, significand, exponent)
    });
    context("int_or_number", f)(src)
}

fn int<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let int = map(tuple((sign, integral)), |(sign, i)| build_int(sign, i));
    let f = delimited_trim(SCOPE_LEFT, int, SCOPE_RIGHT);
    context("int", f)(src)
}

fn number<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let number = map(
        tuple((sign, significand, exponent)),
        |(sign, significand, exponent)| build_number(sign, significand, exponent),
    );
    let f = delimited_trim(SCOPE_LEFT, number, SCOPE_RIGHT);
    context("number", f)(src)
}

fn trim_num0<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list0(char1('_'), f), |s| s.join(""))
}

fn trim_num1<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list1(char1('_'), f), |s| s.join(""))
}

fn sign<'a, E>(src: &'a str) -> IResult<&'a str, bool, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = alt((
        value(true, char1('+')),
        value(false, char1('-')),
        success(true),
    ));
    context("sign", f)(src)
}

fn integral<'a, E>(src: &'a str) -> IResult<&'a str, BigInt, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let dec_no_tag = int_radix(10, digit1);
    let hex = preceded(tag("X"), cut(int_radix(16, hexadecimal1)));
    let bin = preceded(tag("B"), cut(int_radix(2, binary1)));
    let dec = preceded(tag("D"), cut(int_radix(10, digit1)));

    let f = alt((dec_no_tag, hex, bin, dec));
    context("integral", f)(src)
}

fn int_radix<'a, E, F>(radix: u8, f: F) -> impl FnMut(&'a str) -> IResult<&'a str, BigInt, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
{
    map(trim_num1(f), move |int| {
        BigInt::from_str_radix(&int, radix as u32).unwrap()
    })
}

struct Significand {
    int: BigInt,
    radix: u8,
    shift: Option<usize>,
}

fn significand<'a, E>(src: &'a str) -> IResult<&'a str, Significand, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let dec_no_tag = significand_radix(10, digit1);
    let hex = preceded(tag("X"), cut(significand_radix(16, hexadecimal1)));
    let bin = preceded(tag("B"), cut(significand_radix(2, binary1)));
    let dec = preceded(tag("D"), cut(significand_radix(10, digit1)));

    let f = alt((dec_no_tag, hex, bin, dec));
    context("significand", f)(src)
}

fn significand_radix<'a, E, F>(
    radix: u8,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Significand, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, &'a str, E> + Clone,
{
    let int = trim_num1(f.clone());
    let fraction = opt(preceded(char1('.'), cut(trim_num0(f))));
    map(tuple((int, fraction)), move |(int, fraction)| {
        build_significand(radix, int, fraction)
    })
}

fn build_significand(radix: u8, int: String, fraction: Option<String>) -> Significand {
    if let Some(fraction) = fraction {
        let sig = format!("{int}{fraction}");
        let int = BigInt::from_str_radix(&sig, radix as u32).unwrap();
        let shift = Some(fraction.len());
        Significand { int, radix, shift }
    } else {
        let int = BigInt::from_str_radix(&int, radix as u32).unwrap();
        Significand {
            int,
            radix,
            shift: None,
        }
    }
}

fn exponent<'a, E>(src: &'a str) -> IResult<&'a str, Option<BigInt>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let fragment = tuple((sign, trim_num1(digit1)));
    let exp = map(fragment, |(sign, exp)| build_exponent(sign, exp));
    let f = opt(preceded(tag("E"), cut(exp)));
    context("exponent", f)(src)
}

fn build_exponent(sign: bool, exp: String) -> BigInt {
    let i = BigInt::from_str(&exp).unwrap();
    if sign { i } else { i.neg() }
}

fn build_int<T>(sign: bool, i: BigInt) -> T
where
    T: ParseRepr,
{
    let i = Int::new(if sign { i } else { i.neg() });
    From::from(i)
}

fn build_number<T>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T
where
    T: ParseRepr,
{
    let int = significand.int;
    let int = if sign { int } else { int.neg() };
    let shift = significand.shift.unwrap_or(0);
    let exp = exp.unwrap_or_default() - shift;
    let n = Number::new(int, significand.radix, exp);
    From::from(n)
}

fn build_int_or_number<T>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T
where
    T: ParseRepr,
{
    if significand.shift.is_some() || exp.is_some() {
        build_number(sign, significand, exp)
    } else {
        build_int(sign, significand.int)
    }
}

fn byte<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let hex = preceded(tag("X"), cut(hexadecimal_byte));
    let bin = preceded(tag("B"), cut(binary_byte));
    let byte = alt((hex, bin, hexadecimal_byte));
    let f = delimited_trim(SCOPE_LEFT, byte, SCOPE_RIGHT);
    context("byte", f)(src)
}

fn hexadecimal_byte<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let digits = verify(hexadecimal1, |s: &str| s.len() % 2 == 0);
    let digits = trim_num0(digits);
    let f = map(digits, |s| {
        let byte = utils::conversion::hex_str_to_vec_u8(&s).unwrap();
        From::from(Byte::from(byte))
    });
    context("hexadecimal_byte", f)(src)
}

fn binary_byte<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let digits = verify(binary1, |s: &str| s.len() % 8 == 0);
    let digits = trim_num0(digits);
    let f = map(digits, |s| {
        let byte = utils::conversion::bin_str_to_vec_u8(&s).unwrap();
        From::from(Byte::from(byte))
    });
    context("binary_byte", f)(src)
}

fn hexadecimal1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while1(is_hexadecimal)(src)
}

fn is_hexadecimal(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

fn binary1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while1(is_binary)(src)
}

fn is_binary(c: char) -> bool {
    matches!(c, '0'..='1')
}
