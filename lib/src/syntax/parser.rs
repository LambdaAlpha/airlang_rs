use std::{
    hash::Hash,
    num::ParseIntError,
    ops::Neg,
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{
        is_not,
        tag,
        take_while,
        take_while1,
        take_while_m_n,
    },
    character::complete::{
        anychar,
        char as char1,
        digit1,
        multispace1,
    },
    combinator::{
        all_consuming,
        cut,
        fail,
        map,
        map_opt,
        map_res,
        opt,
        peek,
        success,
        value,
        verify,
    },
    error::{
        context,
        convert_error,
        ContextError,
        FromExternalError,
        ParseError,
        VerboseError,
    },
    multi::{
        fold_many0,
        separated_list0,
        separated_list1,
    },
    sequence::{
        delimited,
        preceded,
        terminated,
        tuple,
    },
    AsChar,
    Finish,
    IResult,
    Parser,
};
use num_bigint::BigInt;
use num_traits::Num;

use crate::{
    annotate::Annotate,
    ask::Ask,
    bool::Bool,
    bytes::Bytes,
    call::Call,
    int::Int,
    list::List,
    map::Map,
    number::Number,
    pair::Pair,
    string::Str,
    symbol::Symbol,
    syntax::{
        is_delimiter,
        ANNOTATE_INFIX,
        ASK_INFIX,
        BYTES_PREFIX,
        CALL_INFIX,
        FALSE,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR_INFIX,
        SEPARATOR,
        SHIFT_PREFIX,
        STRING_QUOTE,
        SYMBOL_QUOTE,
        TRUE,
        UNIT,
        WRAP_LEFT,
        WRAP_RIGHT,
    },
    unit::Unit,
    utils,
};

pub(crate) trait ParseRepr:
    From<Unit>
    + From<Bool>
    + From<Int>
    + From<Number>
    + From<Bytes>
    + From<Symbol>
    + From<Str>
    + From<Pair<Self, Self>>
    + From<Call<Self, Self>>
    + From<Ask<Self, Self>>
    + From<List<Self>>
    + Eq
    + Hash
    + From<Map<Self, Self>>
    + From<Annotate<Self, Self>>
    + Clone
{
    fn try_into_pair(self) -> Result<(Self, Self), Self>;
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
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = all_consuming(trim(compose_right_associative));
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

const RIGHT_ASSOCIATIVE: bool = true;
const LEFT_ASSOCIATIVE: bool = false;

fn wrap_left_associative<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    wrap::<LEFT_ASSOCIATIVE, _, _>(src)
}

fn wrap_right_associative<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    wrap::<RIGHT_ASSOCIATIVE, _, _>(src)
}

fn wrap<'a, const ASSOCIATIVITY: bool, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = delimited_trim(WRAP_LEFT, compose::<ASSOCIATIVITY, _, _>, WRAP_RIGHT);
    context("wrap", f)(src)
}

fn token<'a, T: ParseRepr, E>(src: &'a str) -> IResult<&'a str, Token<T>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let (src, first) = peek(anychar)(src)?;

    let parser = match first {
        LIST_LEFT => |s| map(list1, Token::Default)(s),
        LIST_RIGHT => fail,
        MAP_LEFT => |s| map(map1, Token::Default)(s),
        MAP_RIGHT => fail,
        WRAP_LEFT => |s| map(wrap_right_associative, Token::Default)(s),
        WRAP_RIGHT => fail,
        SEPARATOR => fail,

        STRING_QUOTE => |s| map(string, Token::Default)(s),
        SYMBOL_QUOTE => |s| map(quoted_symbol, Token::Default)(s),

        s if is_symbol(s) => unquote_symbol,
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

fn unquote_symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, Token<T>, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let (rest, src) = take_while(is_trivial_symbol)(src)?;

    let mut chars = src.chars();
    let first = chars.next().unwrap();

    match first {
        '0'..='9' => return token_all_consuming(src, rest, number),
        BYTES_PREFIX => return token_all_consuming(src, rest, bytes),
        _ => {}
    }

    let symbol = |s| Ok((s, Token::Unquote(Symbol::from_str(src))));

    match src {
        UNIT => Ok((rest, Token::Default(From::from(Unit)))),
        TRUE => Ok((rest, Token::Default(From::from(Bool::t())))),
        FALSE => Ok((rest, Token::Default(From::from(Bool::f())))),
        SHIFT_PREFIX => alt((
            map(wrap_left_associative, Token::Default),
            map(tokens, Token::Default),
            symbol,
        ))(rest),
        _ => symbol(rest),
    }
}

fn token_all_consuming<'a, T, E, F>(
    src: &'a str,
    rest: &'a str,
    f: F,
) -> IResult<&'a str, Token<T>, E>
where
    T: ParseRepr,
    E: ParseError<&'a str>,
    F: Parser<&'a str, T, E>,
{
    let (_, token) = all_consuming(f)(src)?;
    Ok((rest, Token::Default(token)))
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

fn tokens<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let collect = delimited_trim(LIST_LEFT, separated_list0(empty1, token), LIST_RIGHT);
    let f = map(collect, |tokens| {
        let list = tokens
            .into_iter()
            .map(Token::into_repr)
            .collect::<List<_>>();
        From::from(list)
    });
    context("tokens", f)(src)
}

fn compose_right_associative<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    compose::<RIGHT_ASSOCIATIVE, _, _>(src)
}

fn compose<'a, const ASSOCIATIVITY: bool, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let delimited_tokens = separated_list1(empty1, token);
    let f = map_opt(delimited_tokens, compose_tokens::<ASSOCIATIVITY, _>);
    context("compose", f)(src)
}

fn compose_tokens<const ASSOCIATIVITY: bool, T: ParseRepr>(tokens: Vec<Token<T>>) -> Option<T> {
    let len = tokens.len();
    let mut iter = tokens.into_iter();
    if len == 2 {
        let func = iter.next().unwrap();
        let input = iter.next().unwrap();
        return compose_two(func, input);
    } else if len % 2 == 0 {
        return None;
    }
    if ASSOCIATIVITY == RIGHT_ASSOCIATIVE {
        compose_many::<RIGHT_ASSOCIATIVE, _, _>(iter.rev())
    } else {
        compose_many::<LEFT_ASSOCIATIVE, _, _>(iter)
    }
}

fn compose_two<T: ParseRepr>(func: Token<T>, input: Token<T>) -> Option<T> {
    let input = input.into_repr();
    let repr = match func {
        Token::Unquote(s) => match &*s {
            PAIR_INFIX => {
                let pair = Pair::new(input.clone(), input);
                From::from(pair)
            }
            CALL_INFIX => {
                let call = Call::new(input.clone(), input);
                From::from(call)
            }
            ASK_INFIX => {
                let ask = Ask::new(input.clone(), input);
                From::from(ask)
            }
            ANNOTATE_INFIX => {
                let annotate = Annotate::new(input.clone(), input);
                From::from(annotate)
            }
            _ => {
                let func = From::from(s);
                let call = Call::new(func, input);
                From::from(call)
            }
        },
        Token::Default(func) => {
            let call = Call::new(func, input);
            From::from(call)
        }
    };
    Some(repr)
}

fn compose_many<const ASSOCIATIVITY: bool, T, I>(mut iter: I) -> Option<T>
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>,
{
    let mut first = iter.next().unwrap().into_repr();
    loop {
        let Some(middle) = iter.next() else {
            break;
        };
        let last = iter.next()?.into_repr();
        let repr = if ASSOCIATIVITY == RIGHT_ASSOCIATIVE {
            compose_infix(last, middle, first)
        } else {
            compose_infix(first, middle, last)
        };
        first = repr;
    }
    Some(first)
}

fn compose_infix<T: ParseRepr>(left: T, middle: Token<T>, right: T) -> T {
    match middle {
        Token::Unquote(s) => match &*s {
            PAIR_INFIX => {
                let pair = Pair::new(left, right);
                From::from(pair)
            }
            CALL_INFIX => {
                let call = Call::new(left, right);
                From::from(call)
            }
            ASK_INFIX => {
                let ask = Ask::new(left, right);
                From::from(ask)
            }
            ANNOTATE_INFIX => {
                let annotate = Annotate::new(left, right);
                From::from(annotate)
            }
            _ => {
                let middle = From::from(s);
                let pair = Pair::new(left, right);
                let pair = From::from(pair);
                let infix = Call::new(middle, pair);
                From::from(infix)
            }
        },
        Token::Default(middle) => {
            let pair = Pair::new(left, right);
            let pair = From::from(pair);
            let infix = Call::new(middle, pair);
            From::from(infix)
        }
    }
}

fn items<'a, O1, O2, E, S, F, G>(
    item: F,
    separator: S,
    last: G,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O2>, E>
where
    E: ParseError<&'a str>,
    S: Parser<&'a str, O1, E>,
    F: Parser<&'a str, O2, E>,
    G: Parser<&'a str, O2, E>,
{
    let items_last = tuple((
        fold_many0(
            terminated(item, trim(separator)),
            Vec::new,
            |mut items, item| {
                items.push(item);
                items
            },
        ),
        opt(last),
    ));
    map(items_last, |(mut items, last)| {
        if let Some(last) = last {
            items.push(last);
        }
        items
    })
}

fn list1<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(
        compose_right_associative,
        char1(SEPARATOR),
        compose_right_associative,
    );
    let delimited_items = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
    let f = map(delimited_items, |list| From::from(List::from(list)));
    context("list", f)(src)
}

fn map1<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(key_value_pair, char1(SEPARATOR), key_value_pair);
    let delimited_items = delimited_trim(MAP_LEFT, items, MAP_RIGHT);
    let f = map(delimited_items, |pairs| From::from(Map::from_iter(pairs)));
    context("map", f)(src)
}

fn key_value_pair<'a, T, E>(src: &'a str) -> IResult<&'a str, (T, T), E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = map(compose_right_associative, |repr: T| {
        repr.try_into_pair()
            .unwrap_or_else(|repr| (repr, From::from(Unit)))
    });
    context("pair", f)(src)
}

fn quoted_symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let fragment = alt((
        map(symbol_literal, StringFragment::Literal),
        map(symbol_escaped_char, StringFragment::Escaped),
        value(StringFragment::Space(""), symbol_whitespace),
    ));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::Escaped(c) => string.push(c),
            StringFragment::Space(_s) => {}
        }
        string
    });
    let delimited_string = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    let f = map(delimited_string, |s| From::from(Symbol::from_string(s)));
    context("quoted_symbol", f)(src)
}

fn symbol_escaped_char<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = preceded(
        char1('\\'),
        alt((
            value('\\', char1('\\')),
            value(SYMBOL_QUOTE, char1(SYMBOL_QUOTE)),
        )),
    );
    context("symbol_escaped_char", f)(src)
}

// ignore \t, \r, \n and spaces
fn symbol_whitespace<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = value((), multispace1);
    context("symbol_whitespace", f)(src)
}

fn symbol_literal<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let normal = take_while(|c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let f = verify(normal, |s: &str| !s.is_empty());
    context("symbol_literal", f)(src)
}

fn string<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let fragment = alt((
        map(string_literal, StringFragment::Literal),
        map(string_escaped_char, StringFragment::Escaped),
        map(string_whitespace, StringFragment::Space),
    ));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::Escaped(c) => string.push(c),
            StringFragment::Space(s) => string.push_str(s),
        }
        string
    });
    let delimited_string = delimited_cut(STRING_QUOTE, collect_fragments, STRING_QUOTE);
    let f = map(delimited_string, |s| From::from(Str::from(s)));
    context("string", f)(src)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    Escaped(char),
    Space(&'a str),
}

fn string_escaped_char<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = preceded(
        char1('\\'),
        alt((
            unicode,
            value('\n', char1('n')),
            value('\r', char1('r')),
            value('\t', char1('t')),
            value('\\', char1('\\')),
            value(STRING_QUOTE, char1(STRING_QUOTE)),
            value(' ', char1(' ')),
            value(' ', char1('s')),
        )),
    );
    context("string_escaped_char", f)(src)
}

fn unicode<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digit = take_while_m_n(1, 6, |c: char| c.is_hex_digit());
    let delimited_digit = preceded(char1('u'), delimited_trim(WRAP_LEFT, digit, WRAP_RIGHT));
    let parse_u32 = map_res(delimited_digit, move |hex| u32::from_str_radix(hex, 16));
    let f = map_opt(parse_u32, std::char::from_u32);
    context("unicode", f)(src)
}

// ignore \t, \r, \n and the spaces around them
fn string_whitespace<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = map(multispace1, |s: &str| {
        if s.chars().all(|c| c == ' ') {
            s
        } else {
            &s[0..0]
        }
    });
    context("string_whitespace", f)(src)
}

fn string_literal<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let normal = is_not("\"\\ \t\r\n");
    let f = verify(normal, |s: &str| !s.is_empty());
    context("string_literal", f)(src)
}

fn number<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let norm = preceded(tag("0"), tuple((sign, significand, exponent)));
    let short = tuple((success(true), significand_radix(10, digit1), exponent));
    let f = map(alt((norm, short)), |(sign, significand, exponent)| {
        build_number(sign, significand, exponent)
    });
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

fn build_number<T>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T
where
    T: ParseRepr,
{
    let int = if sign {
        significand.int
    } else {
        significand.int.neg()
    };
    if significand.shift.is_none() && exp.is_none() {
        let i = Int::new(int);
        return From::from(i);
    }
    let shift = significand.shift.unwrap_or(0);
    let exp = exp.unwrap_or_default() - shift;
    let n = Number::new(int, significand.radix, exp);
    From::from(n)
}

fn bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let hex = preceded(tag("X"), cut(hexadecimal_bytes));
    let bin = preceded(tag("B"), cut(binary_bytes));
    let bytes = alt((hex, bin, hexadecimal_bytes));
    let f = preceded(char1(BYTES_PREFIX), bytes);
    context("bytes", f)(src)
}

fn hexadecimal_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(hexadecimal1, |s: &str| s.len() % 2 == 0);
    let digits = trim_num0(digits);
    let f = map_res(digits, |s| {
        Ok(From::from(Bytes::from(
            utils::conversion::hex_str_to_vec_u8(&s)?,
        )))
    });
    context("hexadecimal_bytes", f)(src)
}

fn binary_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(binary1, |s: &str| s.len() % 8 == 0);
    let digits = trim_num0(digits);
    let f = map_res(digits, |s| {
        Ok(From::from(Bytes::from(
            utils::conversion::bin_str_to_vec_u8(&s)?,
        )))
    });
    context("binary_bytes", f)(src)
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
