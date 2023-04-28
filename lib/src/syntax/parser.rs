use {
    crate::{
        repr::{
            CallRepr,
            PairRepr,
            Repr,
            ReverseRepr,
        },
        syntax::{
            COMMENT_PREFIX,
            LIST_LEFT,
            LIST_RIGHT,
            MAP_LEFT,
            MAP_RIGHT,
            PAIR_SEPARATOR,
            PRESERVE_PREFIX,
            REVERSE_SEPARATOR,
            SEPARATOR,
            WRAP_LEFT,
            WRAP_RIGHT,
        },
        types::{
            Bool,
            Bytes,
            Float,
            Int,
            Map,
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
            char,
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
            many0,
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
        num::ParseIntError,
        primitive::char as StdChar,
    },
};

pub fn parse(src: &str) -> Result<Repr, crate::syntax::ParseError> {
    let ret = top::<VerboseError<&str>>(src).finish();
    match ret {
        Ok(r) => Ok(r.1),
        Err(e) => {
            let msg = convert_error(src, e);
            Err(crate::syntax::ParseError { msg })
        }
    }
}

fn top<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = all_consuming(delimited(informal, repr, informal));
    context("top", f)(src)
}

fn delimiter<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = value((), take_while1(is_delimiter));
    context("delimiter", f)(src)
}

fn is_delimiter(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn comment<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = value((), tuple((char(COMMENT_PREFIX), delimiter, cut(token))));
    context("comment", f)(src)
}

fn informal<'a, E>(src: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    value((), many0(alt((delimiter, comment))))(src)
}

fn normed<'a, O, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, O, E>,
{
    preceded(informal, f)
}

fn wrap<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = delimited(
        char(WRAP_LEFT),
        cut(normed(repr)),
        cut(normed(char(WRAP_RIGHT))),
    );
    context("wrap", f)(src)
}

fn pair_second<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let tagged_repr = preceded(char(PAIR_SEPARATOR), normed(token));
    let f = map_opt(tagged_repr, unwrap_token);
    context("pair", f)(src)
}

fn reverse_output<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let tagged_repr = preceded(char(REVERSE_SEPARATOR), normed(token));
    let f = map_opt(tagged_repr, unwrap_token);
    context("reverse", f)(src)
}

fn unwrap_token(token: TaggedRepr) -> Option<Repr> {
    match token.tag {
        TokenTag::Default => Some(token.repr),
        TokenTag::Wrap => Some(token.repr),
        _ => None,
    }
}

fn token<'a, E>(src: &'a str) -> IResult<&'a str, TaggedRepr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    // a parsing rule is decided by first 2 chars
    let (src, (first, second)) = peek(pair(anychar, opt(anychar)))(src)?;

    let parser = match first {
        '0'..='9' => number,
        '+' | '-' => match second {
            Some('0'..='9') => number,
            _ => symbol,
        },
        '"' => string,
        PRESERVE_PREFIX => preserved,
        LIST_LEFT => repr_list,
        MAP_LEFT => repr_map,
        WRAP_LEFT => wrap,
        PAIR_SEPARATOR => pair_second,
        REVERSE_SEPARATOR => reverse_output,
        COMMENT_PREFIX => match second {
            Some(second) => {
                if is_delimiter(second) {
                    fail
                } else {
                    symbol
                }
            }
            None => fail,
        },
        s if is_symbol(s) => symbol,
        _ => fail,
    };
    let tag = match first {
        WRAP_LEFT => TokenTag::Wrap,
        PAIR_SEPARATOR => TokenTag::Pair,
        REVERSE_SEPARATOR => TokenTag::Reverse,
        _ => TokenTag::Default,
    };
    let f = tag_repr(tag, parser);
    context("token", f)(src)
}

fn tag_repr<'a, E, F>(tag: TokenTag, f: F) -> impl FnMut(&'a str) -> IResult<&'a str, TaggedRepr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, Repr, E>,
{
    map(f, move |repr| TaggedRepr { tag, repr })
}

#[derive(Copy, Clone)]
enum TokenTag {
    Default,
    Wrap,
    Pair,
    Reverse,
}

struct TaggedRepr {
    tag: TokenTag,
    repr: Repr,
}

fn repr<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    // postfix has higher priority than infix
    context("repr", infix)(src)
}

fn infix<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = map_opt(postfix, |tokens| {
        let len = tokens.len();
        let mut iter = tokens.into_iter();
        if len == 2 {
            return Some(Repr::Call(Box::new(CallRepr::new(
                iter.next().unwrap(),
                iter.next().unwrap(),
            ))));
        } else if len % 2 == 0 {
            return None;
        }
        let first = iter.next().unwrap();
        let infix_repr = iter
            .array_chunks::<2>()
            .fold(first, |left, [middle, right]| {
                Repr::Call(Box::new(CallRepr::new(
                    middle,
                    Repr::Pair(Box::new(PairRepr::new(left, right))),
                )))
            });
        Some(infix_repr)
    });
    context("infix", f)(src)
}

fn postfix<'a, E>(src: &'a str) -> IResult<&'a str, Vec<Repr>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let fold = fold_many1(
        normed(token),
        || Some(Vec::new()),
        |tokens: Option<Vec<_>>, item| {
            if tokens.is_none() {
                return None;
            }
            let mut tokens = tokens.unwrap();
            let repr = match item.tag {
                TokenTag::Default => {
                    if tokens.is_empty() {
                        item.repr
                    } else {
                        match item.repr {
                            Repr::List(list) => {
                                let last = tokens.pop().unwrap();
                                Repr::Call(Box::new(CallRepr::new(last, Repr::List(list))))
                            }
                            Repr::Map(map) => {
                                let last = tokens.pop().unwrap();
                                Repr::Call(Box::new(CallRepr::new(last, Repr::Map(map))))
                            }
                            other => other,
                        }
                    }
                }
                TokenTag::Wrap => item.repr,
                TokenTag::Pair => {
                    if tokens.is_empty() {
                        return None;
                    } else {
                        let last = tokens.pop().unwrap();
                        Repr::Pair(Box::new(PairRepr::new(last, item.repr)))
                    }
                }
                TokenTag::Reverse => {
                    if tokens.is_empty() {
                        return None;
                    } else {
                        let last = tokens.pop().unwrap();
                        Repr::Reverse(Box::new(ReverseRepr::new(last, item.repr)))
                    }
                }
            };
            tokens.push(repr);
            Some(tokens)
        },
    );
    let f = map_opt(fold, |a| a);
    context("postfix", f)(src)
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
        if last.is_some() {
            items.push(last.unwrap());
        }
        items
    })
}

fn repr_list<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(normed(repr), normed(char(SEPARATOR)), normed(repr));
    let delimited_items = delimited(char(LIST_LEFT), cut(items), cut(normed(char(LIST_RIGHT))));
    let f = map(delimited_items, |list| Repr::List(list.into()));
    context("list", f)(src)
}

fn repr_map<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let items = items(key_value_pair, normed(char(SEPARATOR)), key_value_pair);
    let delimited_items = delimited(char(MAP_LEFT), cut(items), cut(normed(char(MAP_RIGHT))));
    let f = map(delimited_items, |pairs| Repr::Map(Map::from_iter(pairs)));
    context("map", f)(src)
}

fn key_value_pair<'a, E>(src: &'a str) -> IResult<&'a str, (Repr, Repr), E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = map(normed(repr), |repr| match repr {
        Repr::Pair(p) => (p.first, p.second),
        repr => (repr.clone(), repr),
    });
    context("pair", f)(src)
}

fn preserved<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let name = take_while(is_symbol);
    let preserved_word = preceded(char('\''), name);
    let f = map_opt(preserved_word, |s: &str| match s {
        "" => Some(Repr::Unit(Unit)),
        "t" => Some(Repr::Bool(Bool::t())),
        "f" => Some(Repr::Bool(Bool::f())),
        _ => None,
    });
    context("preserved", f)(src)
}

fn is_symbol(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        LIST_LEFT | LIST_RIGHT | MAP_LEFT | MAP_RIGHT | WRAP_LEFT | WRAP_RIGHT | SEPARATOR
        | PAIR_SEPARATOR | REVERSE_SEPARATOR => false,
        c => c.is_ascii_punctuation(),
    }
}

fn symbol<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let symbol_letter_digit = take_while(is_symbol);
    let f = map(symbol_letter_digit, |s: &'a str| {
        Repr::Symbol(Symbol::from_str(s))
    });
    context("symbol", f)(src)
}

fn string<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
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
    let delimited_string = delimited(char('"'), cut(collect_fragments), cut(char('"')));
    let f = map(delimited_string, |s| Repr::String(Str::from(s)));
    context("string", f)(src)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    Escaped(char),
    Space(&'a str),
}

fn escaped_char<'a, E>(src: &'a str) -> IResult<&'a str, StdChar, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = preceded(
        char('\\'),
        alt((
            unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\\', char('\\')),
            value('"', char('"')),
            value(' ', char(' ')),
            value(' ', char('s')),
        )),
    );
    context("escaped_char", f)(src)
}

fn unicode<'a, E>(src: &'a str) -> IResult<&'a str, StdChar, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digit = take_while_m_n(1, 6, |c: char| c.is_hex_digit());
    let delimited_digit = preceded(char('u'), delimited(char('{'), cut(digit), cut(char('}'))));
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

fn number<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let f = alt((hex_int, bin_int, hex_bytes, bin_bytes, decimal));
    context("number", f)(src)
}

fn normed_num0<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list0(char('_'), f), |s| s.join(""))
}

fn normed_num1<'a, E, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
    F: Parser<&'a str, &'a str, E>,
{
    map(separated_list1(char('_'), f), |s| s.join(""))
}

fn hex_int<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = tuple((
        opt(one_of("+-")),
        preceded(tag_no_case("0x"), cut(normed_num1(hex_digit1))),
    ));
    let f = map_res(digits, |(sign, digits): (Option<StdChar>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 16);
        Ok(Repr::Int(i))
    });
    context("hex_int", f)(src)
}

fn bin_int<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = tuple((
        opt(one_of("+-")),
        preceded(
            tag_no_case("0b"),
            cut(normed_num1(take_while1(|c: char| c == '0' || c == '1'))),
        ),
    ));
    let f = map_res(digits, |(sign, digits): (Option<StdChar>, String)| {
        let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &digits, 2);
        Ok(Repr::Int(i))
    });
    context("bin_int", f)(src)
}

fn hex_bytes<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(hex_digit1, |s: &str| s.len() % 2 == 0);
    let tagged_digits = preceded(tag_no_case("1x"), cut(normed_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(Repr::Bytes(Bytes::from(
            utils::conversion::hex_str_to_vec_u8(&s)?,
        )))
    });
    context("hex_bytes", f)(src)
}

fn bin_bytes<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let digits = verify(take_while1(|c| c == '0' || c == '1'), |s: &str| {
        s.len() % 8 == 0
    });
    let tagged_digits = preceded(tag_no_case("1b"), cut(normed_num0(digits)));
    let f = map_res(tagged_digits, |s: String| {
        Ok(Repr::Bytes(Bytes::from(
            utils::conversion::bin_str_to_vec_u8(&s)?,
        )))
    });
    context("bin_bytes", f)(src)
}

fn decimal<'a, E>(src: &'a str) -> IResult<&'a str, Repr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let sign = opt(one_of("+-"));
    let integral = normed_num1(digit1);
    let fractional = opt(preceded(char('.'), cut(normed_num0(digit1))));
    let exponential = opt(preceded(
        tag_no_case("e"),
        cut(tuple((opt(one_of("+-")), normed_num1(digit1)))),
    ));
    let fragments = tuple((sign, integral, fractional, exponential));
    let f = map_res(
        fragments,
        |(sign, integral, fractional, exponential): (
            Option<StdChar>,
            String,
            Option<String>,
            Option<(Option<StdChar>, String)>,
        )| {
            if fractional.is_none() && exponential.is_none() {
                let i = Int::from_sign_string_radix(!matches!(sign, Some('-')), &integral, 10);
                Ok(Repr::Int(i))
            } else {
                let f = Float::from_parts(
                    !matches!(sign, Some('-')),
                    &integral,
                    fractional.as_ref().map_or("", |s| s),
                    !matches!(exponential, Some((Some('-'), _))),
                    exponential.as_ref().map_or("", |(_, exp)| exp),
                );
                Ok(Repr::Float(f))
            }
        },
    );
    context("decimal", f)(src)
}
