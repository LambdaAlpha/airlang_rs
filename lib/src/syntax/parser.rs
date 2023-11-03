use {
    crate::{
        syntax::{
            CALL_SEPARATOR,
            COMMENT_SEPARATOR,
            ESCAPED_PREFIX,
            LIST_LEFT,
            LIST_RIGHT,
            MAP_LEFT,
            MAP_RIGHT,
            PAIR_SEPARATOR,
            REVERSE_SEPARATOR,
            SEPARATOR,
            STRING_QUOTE,
            WRAP_LEFT,
            WRAP_RIGHT,
        },
        types::{
            Bool,
            Bytes,
            Call,
            Float,
            Int,
            List,
            Map,
            Pair,
            Reverse,
            Str,
            Symbol,
            Unit,
        },
        utils,
    },
    nom::{
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
            char as exact_char,
            digit1,
            hex_digit1,
            multispace1,
            one_of,
        },
        combinator::{
            all_consuming,
            cut,
            eof,
            fail,
            map,
            map_opt,
            map_res,
            opt,
            peek,
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
            fold_many1,
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
    },
    std::{
        hash::Hash,
        num::ParseIntError,
    },
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
    let f = all_consuming(delimited(delimiter, repr, delimiter));
    context("top", f)(src)
}

fn delimiter<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = value((), take_while(is_delimiter));
    context("delimiter", f)(src)
}

fn is_delimiter(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn normed<'a, T, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, O, E>,
{
    preceded(delimiter, f)
}

fn wrap<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = delimited(
        exact_char(WRAP_LEFT),
        cut(normed::<T, _, _, _>(repr)),
        cut(normed::<T, _, _, _>(exact_char(WRAP_RIGHT))),
    );
    context("wrap", f)(src)
}

fn token<'a, T: ParseRepr, E>(src: &'a str) -> IResult<&'a str, Token<T>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    // a parsing rule is decided by first 2 chars
    let (src, (first, second)) = peek(pair(anychar, opt(anychar)))(src)?;

    let parser = match first {
        '0'..='9' => |s| map(number, Token::Default)(s),
        '+' | '-' => match second {
            Some('0'..='9') => |s| map(number, Token::Default)(s),
            _ => |s| map(symbol, Token::Default)(s),
        },
        STRING_QUOTE => |s| map(string, Token::Default)(s),
        ESCAPED_PREFIX => |s| map(escaped, Token::Default)(s),
        LIST_LEFT => |s| map(repr_list, Token::Default)(s),
        MAP_LEFT => |s| map(repr_map, Token::Default)(s),
        WRAP_LEFT => |s| map(wrap, Token::Default)(s),
        PAIR_SEPARATOR => match second {
            Some(second) if !is_delimiter(second) => |s| map(symbol, Token::Default)(s),
            _ => |s| map(exact_char(PAIR_SEPARATOR), |_| Token::Pair)(s),
        },
        CALL_SEPARATOR => match second {
            Some(second) if !is_delimiter(second) => |s| map(symbol, Token::Default)(s),
            _ => |s| map(exact_char(CALL_SEPARATOR), |_| Token::Call)(s),
        },
        REVERSE_SEPARATOR => match second {
            Some(second) if !is_delimiter(second) => |s| map(symbol, Token::Default)(s),
            _ => |s| map(exact_char(REVERSE_SEPARATOR), |_| Token::Reverse)(s),
        },
        COMMENT_SEPARATOR => match second {
            Some(second) if !is_delimiter(second) => |s| map(symbol, Token::Default)(s),
            _ => |s| map(exact_char(COMMENT_SEPARATOR), |_| Token::Comment)(s),
        },
        s if is_symbol(s) => |s| map(symbol, Token::Default)(s),
        _ => fail,
    };
    context("token", parser)(src)
}

enum Token<T> {
    Pair,
    Call,
    Reverse,
    Comment,
    Default(T),
}

fn repr<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    context("repr", associate)(src)
}

fn fold_tokens<T: ParseRepr>(tokens: Vec<Token<T>>) -> Option<T> {
    let len = tokens.len();
    let mut iter = tokens.into_iter();
    if len == 2 {
        let func = iter.next().unwrap();
        let Token::Default(input) = iter.next().unwrap() else {
            return None;
        };
        return match func {
            Token::Pair => {
                let pair = Box::new(Pair::new(input.clone(), input));
                Some(<T as From<Box<Pair<T, T>>>>::from(pair))
            }
            Token::Call => {
                let call = Box::new(Call::new(input.clone(), input));
                Some(<T as From<Box<Call<T, T>>>>::from(call))
            }
            Token::Reverse => {
                let reverse = Box::new(Reverse::new(input.clone(), input));
                Some(<T as From<Box<Reverse<T, T>>>>::from(reverse))
            }
            Token::Comment => Some(input),
            Token::Default(func) => {
                let call = Box::new(Call::new(func, input));
                Some(<T as From<Box<Call<T, T>>>>::from(call))
            }
        };
    } else if len % 2 == 0 {
        return None;
    }
    let Token::Default(first) = iter.next().unwrap() else {
        return None;
    };
    iter.array_chunks::<2>()
        .try_fold(first, |left, [middle, right]| {
            let Token::Default(right) = right else {
                return None;
            };
            let repr = match middle {
                Token::Pair => {
                    let pair = Box::new(Pair::new(left, right));
                    <T as From<Box<Pair<T, T>>>>::from(pair)
                }
                Token::Call => {
                    let call = Box::new(Call::new(left, right));
                    <T as From<Box<Call<T, T>>>>::from(call)
                }
                Token::Reverse => {
                    let reverse = Box::new(Reverse::new(left, right));
                    <T as From<Box<Reverse<T, T>>>>::from(reverse)
                }
                Token::Comment => right,
                Token::Default(middle) => {
                    let pair = Box::new(Pair::new(left, right));
                    let pair = <T as From<Box<Pair<T, T>>>>::from(pair);
                    let infix = Box::new(Call::new(middle, pair));
                    <T as From<Box<Call<T, T>>>>::from(infix)
                }
            };
            Some(repr)
        })
}

fn associate<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let collect = fold_many1(
        normed::<T, _, _, _>(token),
        Vec::new,
        |mut tokens: Vec<_>, item| {
            tokens.push(item);
            tokens
        },
    );
    let f = map_opt(collect, fold_tokens);
    context("associate", f)(src)
}

fn items<'a, O1, O2, E, S, F, G>(
    item: F,
    separator: S,
    last: G,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O2>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    S: Parser<&'a str, O1, E>,
    F: Parser<&'a str, O2, E>,
    G: Parser<&'a str, O2, E>,
{
    let items_last = tuple((
        fold_many0(terminated(item, separator), Vec::new, |mut items, item| {
            items.push(item);
            items
        }),
        opt(last),
    ));
    map(items_last, |(mut items, last)| {
        if let Some(last) = last {
            items.push(last);
        }
        items
    })
}

fn repr_list<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(
        normed::<T, _, _, _>(repr::<T, _>),
        normed::<T, _, _, _>(exact_char(SEPARATOR)),
        normed::<T, _, _, _>(repr),
    );
    let delimited_items = delimited(
        exact_char(LIST_LEFT),
        cut(items),
        cut(normed::<T, _, _, _>(exact_char(LIST_RIGHT))),
    );
    let f = map(delimited_items, |list| {
        <T as From<List<T>>>::from(list.into())
    });
    context("list", f)(src)
}

fn repr_map<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(
        key_value_pair::<T, E>,
        normed::<T, _, _, _>(exact_char(SEPARATOR)),
        key_value_pair::<T, E>,
    );
    let delimited_items = delimited(
        exact_char(MAP_LEFT),
        cut(items),
        cut(normed::<T, _, _, _>(exact_char(MAP_RIGHT))),
    );
    let f = map(delimited_items, |pairs| {
        <T as From<Map<T, T>>>::from(Map::from_iter(pairs))
    });
    context("map", f)(src)
}

fn key_value_pair<'a, T, E>(src: &'a str) -> IResult<&'a str, (T, T), E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = map(
        normed::<T, _, _, _>(associate::<T, _>),
        |repr: T| match repr.try_into_pair() {
            Ok(pair) => pair,
            Err(repr) => (repr, <T as From<Unit>>::from(Unit)),
        },
    );
    context("pair", f)(src)
}

fn escaped<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let symbol = take_while(is_symbol);
    let escaped = preceded(exact_char('\''), symbol);
    let f = map_opt(escaped, |s: &str| {
        if let None | Some('a'..='z' | 'A'..='Z') = s.chars().next() {
            preserved(s)
        } else {
            Some(<T as From<Symbol>>::from(Symbol::from_str(s)))
        }
    });
    context("escaped", f)(src)
}

fn preserved<T>(src: &str) -> Option<T>
where
    T: ParseRepr,
{
    match src {
        "" => Some(<T as From<Unit>>::from(Unit)),
        "t" => Some(<T as From<Bool>>::from(Bool::t())),
        "f" => Some(<T as From<Bool>>::from(Bool::f())),
        _ => None,
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        LIST_LEFT | LIST_RIGHT | MAP_LEFT | MAP_RIGHT | WRAP_LEFT | WRAP_RIGHT | SEPARATOR => false,
        c => c.is_ascii_punctuation(),
    }
}

fn symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let symbols = take_while(is_symbol);
    let f = map(symbols, |s: &'a str| {
        <T as From<Symbol>>::from(Symbol::from_str(s))
    });
    context("symbol", f)(src)
}

fn string<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let fragment = alt((
        map(literal, StringFragment::Literal),
        map(escaped_char, StringFragment::Escaped),
        map(whitespace, StringFragment::Space),
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
        exact_char(STRING_QUOTE),
        cut(collect_fragments),
        cut(exact_char(STRING_QUOTE)),
    );
    let f = map(delimited_string, |s| <T as From<Str>>::from(Str::from(s)));
    context("string", f)(src)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    Escaped(char),
    Space(&'a str),
}

fn escaped_char<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = preceded(
        exact_char('\\'),
        alt((
            unicode,
            value('\n', exact_char('n')),
            value('\r', exact_char('r')),
            value('\t', exact_char('t')),
            value('\\', exact_char('\\')),
            value(STRING_QUOTE, exact_char(STRING_QUOTE)),
            value(' ', exact_char(' ')),
            value(' ', exact_char('s')),
        )),
    );
    context("escaped_char", f)(src)
}

fn unicode<'a, E>(src: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digit = take_while_m_n(1, 6, |c: char| c.is_hex_digit());
    let delimited_digit = preceded(
        exact_char('u'),
        delimited(exact_char('{'), cut(digit), cut(exact_char('}'))),
    );
    let parse_u32 = map_res(delimited_digit, move |hex| u32::from_str_radix(hex, 16));
    let f = map_opt(parse_u32, std::char::from_u32);
    context("unicode", f)(src)
}

// ignore space around \t, \r and \n
fn whitespace<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
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
    context("whitespace", f)(src)
}

fn literal<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
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
    let numbers = alt((hex_int, bin_int, hex_bytes, bin_bytes, decimal));
    let next = peek(alt((eof, take_while_m_n(1, 1, |c| !is_symbol(c)))));
    let f = terminated(numbers, next);
    context("number", f)(src)
}

fn normed_num0<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list0(exact_char('_'), f), |s| s.join(""))
}

fn normed_num1<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list1(exact_char('_'), f), |s| s.join(""))
}

fn hex_int<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = tuple((
        opt(one_of("+-")),
        preceded(tag_no_case("0x"), cut(normed_num1(hex_digit1))),
    ));
    let f = map_res(digits, |(sign, digits): (Option<char>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 16);
        Ok(<T as From<Int>>::from(i))
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
            cut(normed_num1(take_while1(|c: char| c == '0' || c == '1'))),
        ),
    ));
    let f = map_res(digits, |(sign, digits): (Option<char>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 2);
        Ok(<T as From<Int>>::from(i))
    });
    context("bin_int", f)(src)
}

fn hex_bytes<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(hex_digit1, |s: &str| s.len() % 2 == 0);
    let tagged_digits = preceded(tag_no_case("1x"), cut(normed_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(<T as From<Bytes>>::from(Bytes::from(
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
    let tagged_digits = preceded(tag_no_case("1b"), cut(normed_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(<T as From<Bytes>>::from(Bytes::from(
            utils::conversion::bin_str_to_vec_u8(&s)?,
        )))
    });
    context("bin_bytes", f)(src)
}

fn decimal<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let sign = opt(one_of("+-"));
    let integral = normed_num1(digit1);
    let fractional = opt(preceded(exact_char('.'), cut(normed_num0(digit1))));
    let exponential = opt(preceded(
        tag_no_case("e"),
        cut(tuple((opt(one_of("+-")), normed_num1(digit1)))),
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
                Ok(<T as From<Int>>::from(i))
            } else {
                let f = Float::from_parts(
                    !matches!(sign, Some('-')),
                    &integral,
                    fractional.as_ref().map_or("", |s| s),
                    !matches!(exponential, Some((Some('-'), _))),
                    exponential.as_ref().map_or("", |(_, exp)| exp),
                );
                Ok(<T as From<Float>>::from(f))
            }
        },
    );
    context("decimal", f)(src)
}
