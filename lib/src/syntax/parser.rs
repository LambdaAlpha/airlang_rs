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
use winnow::ascii::multispace0;
use winnow::ascii::space0;
use winnow::ascii::till_line_ending;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::empty;
use winnow::combinator::eof;
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
use winnow::stream::Checkpoint;
use winnow::stream::Range;
use winnow::stream::Stream;
use winnow::token::any;
use winnow::token::one_of;
use winnow::token::take_while;

use super::BYTE;
use super::CALL_FORWARD;
use super::CALL_REVERSE;
use super::COMMENT;
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

#[derive(Copy, Clone, PartialEq, Eq)]
struct ParseCtx<'a> {
    tag: &'a str,
    direction: Direction,
    reverse: bool,
}

impl Default for ParseCtx<'_> {
    fn default() -> Self {
        Self { tag: "", direction: Direction::Right, reverse: false }
    }
}

impl<'a> ParseCtx<'a> {
    fn tag(mut self, tag: &'a str) -> Self {
        self.tag = tag;
        self
    }

    fn untag(mut self) -> Self {
        self.tag = "";
        self
    }

    fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

type E = ErrMode<ContextError>;

pub fn parse<T: ParseRepr>(src: &str) -> Result<T, super::ParseError> {
    terminated(top::<T>, eof.context(expect_desc("end")))
        .parse(src)
        .map_err(|e| super::ParseError { msg: e.to_string() })
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
    trim_comment(compose(ParseCtx::default())).parse_next(src)
}

fn trim<'a, O, F>(f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(spaces(0 ..), f, spaces(0 ..))
}

fn trim_comment<'a, O, F>(f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(spaces_comment(0 ..), f, spaces_comment(0 ..))
}

fn spaces<'a>(occurrences: impl Into<Range>) -> impl Parser<&'a str, (), E> {
    take_while(occurrences, is_spaces).context(label("spaces")).void()
}

fn spaces_comment<'a>(occurrences: impl Into<Range>) -> impl Parser<&'a str, (), E> {
    repeat(occurrences, alt((spaces(1 ..), comment)))
}

fn is_spaces(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn comment(i: &mut &str) -> ModalResult<()> {
    let comment = delimited_cut(SCOPE_LEFT, comment_tokens, SCOPE_RIGHT);
    preceded(COMMENT, comment).context(label("comment")).parse_next(i)
}

fn comment_tokens(i: &mut &str) -> ModalResult<()> {
    repeat(0 .., comment_token).parse_next(i)
}

fn comment_token(i: &mut &str) -> ModalResult<()> {
    match peek(any).parse_next(i)? {
        // delimiters
        LIST_LEFT => delimited_cut(LIST_LEFT, comment_tokens, LIST_RIGHT).parse_next(i),
        LIST_RIGHT => fail.parse_next(i),
        MAP_LEFT => delimited_cut(MAP_LEFT, comment_tokens, MAP_RIGHT).parse_next(i),
        MAP_RIGHT => fail.parse_next(i),
        SCOPE_LEFT => delimited_cut(SCOPE_LEFT, comment_tokens, SCOPE_RIGHT).parse_next(i),
        SCOPE_RIGHT => fail.parse_next(i),
        SEPARATOR => any.void().parse_next(i),
        c if is_spaces(c) => spaces(1 ..).parse_next(i),
        TEXT_QUOTE => {
            let text = take_while(0 .., |c| c != TEXT_QUOTE).void();
            delimited_cut(TEXT_QUOTE, text, TEXT_QUOTE).parse_next(i)
        }
        SYMBOL_QUOTE => {
            let symbol = take_while(0 .., |c| c != SYMBOL_QUOTE).void();
            delimited_cut(SYMBOL_QUOTE, symbol, SYMBOL_QUOTE).parse_next(i)
        }

        _ => {
            let symbol = trivial_symbol1.context(label("symbol")).parse_next(i)?;
            if symbol != UNIT || !i.starts_with([TEXT_QUOTE, SYMBOL_QUOTE]) {
                return empty.parse_next(i);
            }
            let quote = i.chars().next().unwrap();
            let line = (till_line_ending, line_ending, space0, one_of(is_symbol));
            let content = separated(1 .., line, ' ');
            delimited_cut(quote, content, quote).parse_next(i)
        }
    }
}

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

fn delimited_trim_comment<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_cut(left, trim_comment(f), right)
}

fn scoped<'a, T, F>(f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim(SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scoped_trim_comment<'a, T, F>(f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim_comment(SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scope<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    scoped_trim_comment(compose(ctx)).context(label("scope"))
}

fn full_symbol<'a, T, F>(f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    terminated(f, not(one_of(is_trivial_symbol)).context(expect_desc("no symbols")))
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

fn token<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, Token<T>, E> {
    (move |i: &mut _| match peek(any).parse_next(i)? {
        // delimiters
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

fn ext<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, Token<T>, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        let checkpoint = i.checkpoint();
        let symbol = trivial_symbol1.context(label("symbol")).parse_next(i)?;
        if i.starts_with(LEFT_DELIMITERS) {
            prefix(symbol, ctx).map(Token::Default).parse_next(i)
        } else {
            alt((
                keyword(symbol, checkpoint).map(Token::Default),
                empty.value(Token::Unquote(Symbol::from_str_unchecked(symbol))),
            ))
            .parse_next(i)
        }
    }
}

const LEFT_DELIMITERS: [char; 5] = [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, SYMBOL_QUOTE, TEXT_QUOTE];

fn keyword<'a, T: ParseRepr>(
    s: &'a str, checkpoint: Checkpoint<&'a str, &'a str>,
) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        match s {
            UNIT => Ok(T::from(Unit)),
            TRUE => Ok(T::from(Bit::true_())),
            FALSE => Ok(T::from(Bit::false_())),
            s if matches!(s.chars().next(), Some('0' ..= '9')) => {
                i.reset(&checkpoint);
                return cut_err(full_symbol(int_or_number).context(label("int or number")))
                    .parse_next(i);
            }
            _ => fail.parse_next(i),
        }
    }
}

fn prefix<'a, T: ParseRepr>(prefix: &'a str, ctx: ParseCtx<'a>) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        match prefix {
            UNIT => match i.chars().next().unwrap() {
                TEXT_QUOTE => raw_text.map(T::from).parse_next(i),
                SYMBOL_QUOTE => raw_symbol.map(T::from).parse_next(i),
                LIST_LEFT => raw_list(ctx).parse_next(i),
                MAP_LEFT => raw_map(ctx).parse_next(i),
                _ => fail.context(label("prefix token")).parse_next(i),
            },
            INT => int.map(T::from).parse_next(i),
            NUMBER => number.map(T::from).parse_next(i),
            BYTE => byte.map(T::from).parse_next(i),
            CALL_FORWARD => scope(ctx.untag().reverse(false)).parse_next(i),
            CALL_REVERSE => scope(ctx.untag().reverse(true)).parse_next(i),
            LEFT => scope(ctx.untag().direction(Direction::Left)).parse_next(i),
            RIGHT => scope(ctx.untag().direction(Direction::Right)).parse_next(i),
            s if s.ends_with(CALL_FORWARD) => {
                scope(ctx.reverse(false).tag(&s[.. s.len() - 1])).parse_next(i)
            }
            s if s.ends_with(CALL_REVERSE) => {
                scope(ctx.reverse(true).tag(&s[.. s.len() - 1])).parse_next(i)
            }
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

fn repr<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    token(ctx).map(Token::into_repr)
}

fn compose_token<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, Token<T>, E> {
    move |i: &mut _| {
        if !ctx.tag.is_empty() {
            let tokens = separated(1 .., repr::<T>(ctx), spaces_comment(1 ..)).parse_next(i)?;
            return Ok(Token::Default(tag(ctx, tokens, ctx.tag)));
        }
        match ctx.direction {
            Direction::Left => compose_left(ctx).parse_next(i),
            Direction::Right => compose_right(ctx).parse_next(i),
        }
    }
}

fn compose_left<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, Token<T>, E> {
    move |i: &mut _| {
        let mut left = token(ctx).parse_next(i)?;
        let mut next_token = preceded(spaces_comment(1 ..), token(ctx));
        loop {
            let Ok(func) = next_token.by_ref().parse_next(i) else {
                break;
            };
            let repr = match func {
                Token::Unquote(s) => match &*s {
                    PAIR => {
                        let right = next_token.by_ref().parse_next(i)?;
                        T::from(Pair::new(left.into_repr(), right.into_repr()))
                    }
                    CALL_FORWARD => {
                        let right = next_token.by_ref().parse_next(i)?;
                        T::from(Call::new(false, left.into_repr(), right.into_repr()))
                    }
                    CALL_REVERSE => {
                        let right = next_token.by_ref().parse_next(i)?;
                        T::from(Call::new(true, left.into_repr(), right.into_repr()))
                    }
                    _ => {
                        let right = next_token.by_ref().parse_next(i)?;
                        call(ctx, T::from(s), left_right(left, right))
                    }
                },
                Token::Default(func) => {
                    let right = next_token.by_ref().parse_next(i)?;
                    call(ctx, func, left_right(left, right))
                }
            };
            left = Token::Default(repr);
        }
        Ok(left)
    }
}

fn compose_right<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, Token<T>, E> {
    move |i: &mut _| {
        let left = token(ctx).parse_next(i)?;
        let Ok(func) = preceded(spaces_comment(1 ..), token(ctx)).parse_next(i) else {
            return Ok(left);
        };
        let mut tail = preceded(spaces_comment(1 ..), compose_token(ctx));
        let repr = match func {
            Token::Unquote(s) => match &*s {
                PAIR => {
                    let right = tail.parse_next(i)?;
                    T::from(Pair::new(left.into_repr(), right.into_repr()))
                }
                CALL_FORWARD => {
                    let right = tail.parse_next(i)?;
                    T::from(Call::new(false, left.into_repr(), right.into_repr()))
                }
                CALL_REVERSE => {
                    let right = tail.parse_next(i)?;
                    T::from(Call::new(true, left.into_repr(), right.into_repr()))
                }
                _ => {
                    let right = tail.parse_next(i)?;
                    call(ctx, T::from(s), left_right(left, right))
                }
            },
            Token::Default(func) => {
                let right = tail.parse_next(i)?;
                call(ctx, func, left_right(left, right))
            }
        };
        Ok(Token::Default(repr))
    }
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

fn compose<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    compose_token(ctx).map(Token::into_repr)
}

fn tag<T: ParseRepr>(ctx: ParseCtx, tokens: Vec<T>, tag: &str) -> T {
    let list = T::from(List::from(tokens));
    let tag = T::from(Symbol::from_str_unchecked(tag));
    call(ctx, tag, list)
}

fn call<T: ParseRepr>(ctx: ParseCtx, left: T, right: T) -> T {
    T::from(Call::new(ctx.reverse, left, right))
}

fn items<'a, O, F>(mut item: F) -> impl Parser<&'a str, Vec<O>, E>
where F: Parser<&'a str, O, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        let mut items = Vec::new();
        loop {
            let Some(item) = opt(item.by_ref()).parse_next(i)? else {
                break;
            };
            items.push(item);
            let sep = opt(trim_comment(SEPARATOR.context(expect_char(SEPARATOR)))).parse_next(i)?;
            if sep.is_none() {
                break;
            }
        }
        Ok(items)
    }
}

fn list<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    delimited_trim_comment(LIST_LEFT, items(compose(ctx)), LIST_RIGHT)
        .map(|list| T::from(List::from(list)))
        .context(label("list"))
}

fn raw_list<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    let repr_list = separated(0 .., repr::<T>(ctx), spaces_comment(1 ..));
    delimited_trim_comment(LIST_LEFT, repr_list, LIST_RIGHT)
        .map(|tokens: Vec<_>| T::from(List::from(tokens)))
        .context(label("raw list"))
}

fn map<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    delimited_trim_comment(MAP_LEFT, items(key_value(ctx)), MAP_RIGHT)
        .map(|pairs| T::from(Map::from_iter(pairs)))
        .context(label("map"))
}

fn key_value<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, (T, T), E> {
    move |i: &mut _| {
        let key = repr(ctx).parse_next(i)?;
        let key = if ctx.tag.is_empty() { key } else { tag(ctx, vec![key], ctx.tag) };
        let pair = opt(preceded(spaces_comment(1 ..), PAIR.void())).parse_next(i).unwrap();
        if pair.is_none() {
            return Ok((key, T::from(Unit)));
        }
        let value = cut_err(preceded(
            spaces_comment(1 ..).context(expect_desc("space")),
            compose(ctx).context(expect_desc("value")),
        ))
        .parse_next(i)?;
        Ok((key, value))
    }
}

fn raw_map<T: ParseRepr>(ctx: ParseCtx<'_>) -> impl Parser<&str, T, E> {
    let items = move |i: &mut _| {
        let tokens: Vec<_> = separated(0 .., repr::<T>(ctx), spaces_comment(1 ..)).parse_next(i)?;
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
    delimited_trim_comment(MAP_LEFT, items, MAP_RIGHT).context(label("raw map"))
}

fn symbol(i: &mut &str) -> ModalResult<Symbol> {
    let literal = take_while(1 .., |c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let fragment = alt((literal, symbol_escaped, symbol_space));
    let symbol = repeat(0 .., fragment).fold(String::new, |mut string, fragment| {
        string.push_str(fragment);
        string
    });
    delimited_cut(SYMBOL_QUOTE, symbol, SYMBOL_QUOTE)
        .map(Symbol::from_string_unchecked)
        .context(label("symbol"))
        .parse_next(i)
}

fn symbol_escaped<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    preceded('\\', move |i: &mut _| match any.parse_next(i)? {
        '\\' => empty.value("\\").parse_next(i),
        '_' => empty.value(" ").parse_next(i),
        QUOTE => empty.value(concatcp!(SYMBOL_QUOTE)).parse_next(i),
        ' ' | '\t' | '\r' | '\n' => multispace0.value("").parse_next(i),
        _ => fail.parse_next(i),
    })
    .context(expect_desc("escape character"))
    .parse_next(i)
}

// ignore spaces following \n
fn symbol_space<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    (move |i: &mut _| match any.parse_next(i)? {
        '\n' => multispace0.value("").parse_next(i),
        '\r' => empty.value("").parse_next(i),
        '\t' => take_while(0 .., |c| c == '\t').value("").parse_next(i),
        _ => fail.parse_next(i),
    })
    .context(expect_desc("spaces"))
    .parse_next(i)
}

fn raw_symbol(i: &mut &str) -> ModalResult<Symbol> {
    let literal = take_while(0 .., is_symbol);
    let symbol = separated(1 .., terminated(literal, raw_symbol_newline), ' ')
        .map(|fragments: Vec<_>| fragments.join(""));
    delimited_cut(SYMBOL_QUOTE, symbol, SYMBOL_QUOTE)
        .map(Symbol::from_string_unchecked)
        .context(label("raw symbol"))
        .parse_next(i)
}

fn raw_symbol_newline(i: &mut &str) -> ModalResult<()> {
    (line_ending, space0, '|'.context(expect_char('|')))
        .void()
        .context(expect_desc("newline"))
        .parse_next(i)
}

fn text(i: &mut &str) -> ModalResult<Text> {
    let literal = take_while(1 .., |c| !matches!(c, '"' | '\\' | '\n'));
    let space = terminated('\n', multispace0);
    let fragment = alt((literal.map(StrFragment::Str), text_escaped, space.map(StrFragment::Char)));
    let text = repeat(0 .., fragment).fold(String::new, |mut string, fragment| {
        fragment.push(&mut string);
        string
    });
    delimited_cut(TEXT_QUOTE, text, TEXT_QUOTE).map(Text::from).context(label("text")).parse_next(i)
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
        ' ' | '\t' | '\r' | '\n' => multispace0.value(StrFragment::Str("")).parse_next(i),
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

fn raw_text(i: &mut &str) -> ModalResult<Text> {
    let fragment = separated(1 .., (till_line_ending, raw_text_newline), ' ');
    let text = fragment.map(|fragments: Vec<_>| {
        let mut s = String::new();
        for (literal, newline) in fragments {
            s.push_str(literal);
            s.push_str(newline);
        }
        s
    });
    delimited_cut(TEXT_QUOTE, text, TEXT_QUOTE)
        .map(Text::from)
        .context(label("raw text"))
        .parse_next(i)
}

fn raw_text_newline<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    let newline = alt(('+'.value(true), '|'.value(false)))
        .context(expect_char('+'))
        .context(expect_char('|'));
    (line_ending, space0, newline)
        .map(|(ending, _, newline): (&str, _, _)| if newline { ending } else { "" })
        .context(expect_desc("newline"))
        .parse_next(i)
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    separated(0 .., f, '_').map(|s: Vec<&str>| s.join(""))
}

fn trim_num1<'a, F>(f: F) -> impl Parser<&'a str, String, E>
where F: Parser<&'a str, &'a str, E> {
    separated(1 .., f, '_').map(|s: Vec<&str>| s.join(""))
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
