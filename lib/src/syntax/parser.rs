use std::{
    hash::Hash,
    num::ParseIntError,
};

use nom::{
    branch::alt,
    bytes::complete::{
        is_not,
        tag_no_case,
        take_while,
        take_while1,
        take_while_m_n,
    },
    character::complete::{
        anychar,
        char as char1,
        digit1,
        hex_digit1,
        multispace1,
        one_of,
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
        pair,
        preceded,
        terminated,
        tuple,
    },
    AsChar,
    Finish,
    IResult,
    Parser,
};

use crate::{
    annotation::Annotation,
    bool::Bool,
    bytes::Bytes,
    call::Call,
    float::Float,
    int::Int,
    list::List,
    map::Map,
    pair::Pair,
    reverse::Reverse,
    string::Str,
    symbol::Symbol,
    syntax::{
        is_special,
        ANNOTATION_INFIX,
        BYTES_PREFIX,
        CALL_INFIX,
        FALSE,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        PAIR_INFIX,
        REVERSE_INFIX,
        SEPARATOR,
        STRING_QUOTE,
        SYMBOL_QUOTE,
        TOKENS_QUOTE,
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
    + From<Float>
    + From<Bytes>
    + From<Symbol>
    + From<Str>
    + From<Box<Pair<Self, Self>>>
    + From<Box<Call<Self, Self>>>
    + From<Box<Reverse<Self, Self>>>
    + From<List<Self>>
    + Eq
    + Hash
    + From<Map<Self, Self>>
    + From<Box<Annotation<Self, Self>>>
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
    let f = all_consuming(trim(compose));
    context("top", f)(src)
}

fn trim<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(delimiter0, f, delimiter0)
}

fn delimiter0<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while(is_delimiter)(src)
}

fn delimiter1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    take_while1(is_delimiter)(src)
}

fn is_delimiter(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn brackets<'a, T, E, F>(
    left: char,
    right: char,
    f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, T, E>,
{
    delimited(char1(left), cut(trim(f)), cut(char1(right)))
}

fn wrap<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = brackets(WRAP_LEFT, WRAP_RIGHT, compose);
    context("wrap", f)(src)
}

fn token<'a, T: ParseRepr, E>(src: &'a str) -> IResult<&'a str, Token<T>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    // a parsing rule is decided by first 2 chars
    let (src, (first, second)) = peek(pair(anychar, opt(anychar)))(src)?;

    let parser = match first {
        LIST_LEFT => |s| map(list1, Token::Default)(s),
        LIST_RIGHT => fail,
        MAP_LEFT => |s| map(map1, Token::Default)(s),
        MAP_RIGHT => fail,
        WRAP_LEFT => |s| map(wrap, Token::Default)(s),
        WRAP_RIGHT => fail,
        SEPARATOR => fail,
        TOKENS_QUOTE => |s| map(tokens, Token::Default)(s),
        STRING_QUOTE => |s| map(string, Token::Default)(s),
        SYMBOL_QUOTE => |s| map(quoted_symbol, Token::Default)(s),
        BYTES_PREFIX => |s| map(bytes, Token::Default)(s),
        '0'..='9' => |s| map(number, Token::Default)(s),
        '+' | '-' if matches!(second, Some('0'..='9')) => |s| map(number, Token::Default)(s),
        s if is_symbol(s) => unquote_symbol,
        _ => fail,
    };
    context("token", parser)(src)
}

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

fn non_tokens<'a, T: ParseRepr, E>(src: &'a str) -> IResult<&'a str, Token<T>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let (src, first) = peek(anychar)(src)?;
    let parser = match first {
        TOKENS_QUOTE => fail,
        _ => token,
    };
    context("non_tokens", parser)(src)
}

fn tokens<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let collect = brackets(
        TOKENS_QUOTE,
        TOKENS_QUOTE,
        separated_list0(delimiter1, non_tokens),
    );
    let f = map(collect, |tokens| {
        let list = tokens
            .into_iter()
            .map(Token::into_repr)
            .collect::<List<_>>();
        From::from(list)
    });
    context("tokens", f)(src)
}

fn compose<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let delimited_tokens = separated_list1(delimiter1, token);
    let f = map_opt(delimited_tokens, compose_tokens);
    context("compose", f)(src)
}

fn compose_tokens<T: ParseRepr>(tokens: Vec<Token<T>>) -> Option<T> {
    let len = tokens.len();
    let mut iter = tokens.into_iter();
    if len == 2 {
        let func = iter.next().unwrap();
        let input = iter.next().unwrap();
        return compose_two(func, input);
    } else if len % 2 == 0 {
        return None;
    }
    let mut iter = iter.rev();
    let last = iter.next().unwrap().into_repr();
    let mut right = last;
    loop {
        let Some(middle) = iter.next() else {
            break;
        };
        let left = iter.next()?.into_repr();
        let repr = compose_infix(left, middle, right);
        right = repr;
    }
    Some(right)
}

fn compose_two<T: ParseRepr>(func: Token<T>, input: Token<T>) -> Option<T> {
    let input = input.into_repr();
    let repr = match func {
        Token::Unquote(s) => match &*s {
            PAIR_INFIX => {
                let pair = Box::new(Pair::new(input.clone(), input));
                From::from(pair)
            }
            CALL_INFIX => {
                let call = Box::new(Call::new(input.clone(), input));
                From::from(call)
            }
            REVERSE_INFIX => {
                let reverse = Box::new(Reverse::new(input.clone(), input));
                From::from(reverse)
            }
            ANNOTATION_INFIX => {
                let annotation = Box::new(Annotation::new(input.clone(), input));
                From::from(annotation)
            }
            _ => {
                let func = From::from(s);
                let call = Box::new(Call::new(func, input));
                From::from(call)
            }
        },
        Token::Default(func) => {
            let call = Box::new(Call::new(func, input));
            From::from(call)
        }
    };
    Some(repr)
}

fn compose_infix<T: ParseRepr>(left: T, middle: Token<T>, right: T) -> T {
    match middle {
        Token::Unquote(s) => match &*s {
            PAIR_INFIX => {
                let pair = Box::new(Pair::new(left, right));
                From::from(pair)
            }
            CALL_INFIX => {
                let call = Box::new(Call::new(left, right));
                From::from(call)
            }
            REVERSE_INFIX => {
                let reverse = Box::new(Reverse::new(left, right));
                From::from(reverse)
            }
            ANNOTATION_INFIX => {
                let annotation = Box::new(Annotation::new(left, right));
                From::from(annotation)
            }
            _ => {
                let middle = From::from(s);
                let pair = Box::new(Pair::new(left, right));
                let pair = From::from(pair);
                let infix = Box::new(Call::new(middle, pair));
                From::from(infix)
            }
        },
        Token::Default(middle) => {
            let pair = Box::new(Pair::new(left, right));
            let pair = From::from(pair);
            let infix = Box::new(Call::new(middle, pair));
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
    let items = items(compose, char1(SEPARATOR), compose);
    let delimited_items = brackets(LIST_LEFT, LIST_RIGHT, items);
    let f = map(delimited_items, |list| From::from(List::from(list)));
    context("list", f)(src)
}

fn map1<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(key_value_pair, char1(SEPARATOR), key_value_pair);
    let delimited_items = brackets(MAP_LEFT, MAP_RIGHT, items);
    let f = map(delimited_items, |pairs| From::from(Map::from_iter(pairs)));
    context("map", f)(src)
}

fn key_value_pair<'a, T, E>(src: &'a str) -> IResult<&'a str, (T, T), E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = map(compose, |repr: T| {
        repr.try_into_pair()
            .unwrap_or_else(|repr| (repr, From::from(Unit)))
    });
    context("pair", f)(src)
}

fn is_trivial_symbol(c: char) -> bool {
    if is_special(c) {
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
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let symbols = take_while(is_trivial_symbol);
    let f = map(symbols, unquote_symbol_to_token);
    context("unquote_symbol", f)(src)
}

fn unquote_symbol_to_token<T: ParseRepr>(s: &str) -> Token<T> {
    let token = match s {
        UNIT => From::from(Unit),
        TRUE => From::from(Bool::t()),
        FALSE => From::from(Bool::f()),
        s => {
            let s = Symbol::from_str(s);
            return Token::Unquote(s);
        }
    };
    Token::Default(token)
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
    let delimited_string = delimited(
        char1(SYMBOL_QUOTE),
        cut(collect_fragments),
        cut(char1(SYMBOL_QUOTE)),
    );
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
    let delimited_string = delimited(
        char1(STRING_QUOTE),
        cut(collect_fragments),
        cut(char1(STRING_QUOTE)),
    );
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
    let delimited_digit = preceded(
        char1('u'),
        delimited(char1('('), cut(digit), cut(char1(')'))),
    );
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
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = alt((hex_int, bin_int, decimal));
    context("number", f)(src)
}

fn trim_num0<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list0(char1('_'), f), |s| s.join(""))
}

fn trim_num1<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list1(char1('_'), f), |s| s.join(""))
}

fn hex_int<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = tuple((
        opt(one_of("+-")),
        preceded(tag_no_case("0x"), cut(trim_num1(hex_digit1))),
    ));
    let f = map_res(digits, |(sign, digits): (Option<char>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 16);
        Ok(From::from(i))
    });
    context("hex_int", f)(src)
}

fn bin_int<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = tuple((
        opt(one_of("+-")),
        preceded(
            tag_no_case("0b"),
            cut(trim_num1(take_while1(|c: char| c == '0' || c == '1'))),
        ),
    ));
    let f = map_res(digits, |(sign, digits): (Option<char>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 2);
        Ok(From::from(i))
    });
    context("bin_int", f)(src)
}

fn decimal<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let sign = opt(one_of("+-"));
    let integral = trim_num1(digit1);
    let fractional = opt(preceded(char1('.'), cut(trim_num0(digit1))));
    let exponential = opt(preceded(
        tag_no_case("e"),
        cut(tuple((opt(one_of("+-")), trim_num1(digit1)))),
    ));
    let fragments = tuple((sign, integral, fractional, exponential));
    #[allow(clippy::type_complexity)]
    let f = map_res(
        fragments,
        |(sign, integral, fractional, exponential): (
            Option<char>,
            String,
            Option<String>,
            Option<(Option<char>, String)>,
        )| {
            if fractional.is_none() && exponential.is_none() {
                let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &integral, 10);
                Ok(From::from(i))
            } else {
                let f = Float::from_parts(
                    !matches!(sign, Some('-')),
                    &integral,
                    fractional.as_ref().map_or("", |s| s),
                    !matches!(exponential, Some((Some('-'), _))),
                    exponential.as_ref().map_or("", |(_, exp)| exp),
                );
                Ok(From::from(f))
            }
        },
    );
    context("decimal", f)(src)
}

fn bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = preceded(
        char1(BYTES_PREFIX),
        alt((hex_bytes, bin_bytes, empty_bytes)),
    );
    context("bytes", f)(src)
}

fn hex_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(hex_digit1, |s: &str| s.len() % 2 == 0);
    let tagged_digits = preceded(tag_no_case("x"), cut(trim_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(From::from(Bytes::from(
            utils::conversion::hex_str_to_vec_u8(&s)?,
        )))
    });
    context("hex_bytes", f)(src)
}

fn bin_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(take_while1(|c| c == '0' || c == '1'), |s: &str| {
        s.len() % 8 == 0
    });
    let tagged_digits = preceded(tag_no_case("b"), cut(trim_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(From::from(Bytes::from(
            utils::conversion::bin_str_to_vec_u8(&s)?,
        )))
    });
    context("bin_bytes", f)(src)
}

fn empty_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = success(From::from(Bytes::default()));
    context("empty_bytes", f)(src)
}
