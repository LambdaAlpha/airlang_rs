use std::hash::Hash;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use const_format::concatcp;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::Num;
use num_traits::Zero;
use winnow::ModalResult;
use winnow::Parser;
use winnow::Result;
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
use winnow::stream::Checkpoint;
use winnow::stream::Stream;
use winnow::token::any;
use winnow::token::one_of;
use winnow::token::take_until;
use winnow::token::take_while;

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
use super::SCOPE_LEFT;
use super::SCOPE_RIGHT;
use super::SEPARATOR;
use super::SPACE;
use super::TEXT_QUOTE;
use super::TRUE;
use super::UNIT;
use super::is_delimiter;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Decimal;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;
use crate::type_::Unit;
use crate::utils::conversion::bin_str_to_vec_u8;
use crate::utils::conversion::hex_str_to_vec_u8;

pub trait ParseRepr:
    From<Unit>
    + From<Bit>
    + From<Key>
    + From<Text>
    + From<Int>
    + From<Decimal>
    + From<Byte>
    + From<Pair<Self, Self>>
    + From<Call<Self, Self>>
    + From<List<Self>>
    + From<Map<Key, Self>> {
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

fn cut_expect_desc(description: &'static str) -> E {
    let mut ctx = ContextError::new();
    ctx.push(expect_desc(description));
    ErrMode::Cut(ctx)
}

fn top<T: ParseRepr>(src: &mut &str) -> ModalResult<T> {
    let ctx = ParseCtx::default();
    trim_comment(ctx, compose(ctx)).parse_next(src)
}

fn trim_comment<'a, O, F>(ctx: ParseCtx, f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(opt(spaces_comment(ctx)), f, opt(spaces_comment(ctx)))
}

fn spaces(i: &mut &str) -> ModalResult<()> {
    let spaces = take_while(1 .., |c| matches!(c, ' ' | '\t' | '\n'));
    let f = repeat(1 .., alt((spaces, "\r\n")).void());
    f.context(label("spaces")).parse_next(i)
}

fn space_tab0(i: &mut &str) -> ModalResult<()> {
    let f = take_while(0 .., |c| matches!(c, ' ' | '\t')).void();
    f.context(label("space_tab0")).parse_next(i)
}

fn spaces_comment<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    repeat(1 .., alt((spaces, comment(ctx))))
}

fn comment<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    let comment_tokens = repeat(0 .., comment_token(ctx));
    let f = preceded(EMPTY, delimited_cut(SCOPE_LEFT, comment_tokens, SCOPE_RIGHT));
    f.context(label("comment"))
}

fn comment_token<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    // to avoid error[E0720]: cannot resolve opaque type
    move |i: &mut _| {
        alt((spaces, SEPARATOR.void(), comment(ctx), token::<C>(ctx).void())).parse_next(i)
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

impl_parse_repr_for_comment!(Unit Bit Key Text Int Decimal Byte);
impl_parse_repr_for_comment!(Pair<C, C> Call<C, C> List<C> Map<Key, C>);
impl ParseRepr for C {}

fn delimited_cut<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    let left = left.context(expect_char(left));
    let right = right.context(expect_char(right));
    delimited(left, cut_err(f), cut_err(right))
}

fn delimited_trim_comment<'a, T, F>(
    ctx: ParseCtx, left: char, f: F, right: char,
) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_cut(left, trim_comment(ctx, f), right)
}

fn scoped_trim_comment<'a, T, F>(ctx: ParseCtx, f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim_comment(ctx, SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scope<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    scoped_trim_comment(ctx, compose(ctx)).context(label("scope"))
}

fn trivial_key1<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_trivial_key).parse_next(i)
}

fn is_trivial_key(c: char) -> bool {
    Key::is_key(c) && !is_delimiter(c)
}

fn is_key(c: char) -> bool {
    Key::is_key(c)
}

fn token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<'a, T>, E> {
    let f = move |i: &mut _| match peek(any).parse_next(i)? {
        LIST_LEFT => list(ctx).map(Token::Default).parse_next(i),
        LIST_RIGHT => fail.parse_next(i),
        MAP_LEFT => map(ctx).map(Token::Default).parse_next(i),
        MAP_RIGHT => fail.parse_next(i),
        SCOPE_LEFT => scope(ctx).map(Token::Default).parse_next(i),
        SCOPE_RIGHT => fail.parse_next(i),
        SEPARATOR => fail.parse_next(i),
        SPACE => fail.parse_next(i),
        TEXT_QUOTE => text.map(T::from).map(Token::Default).parse_next(i),
        KEY_QUOTE => key.map(T::from).map(Token::Default).parse_next(i),
        '0' ..= '9' => number.map(Token::Default).parse_next(i),
        _ => cut_err(key_token(ctx)).parse_next(i),
    };
    f.context(label("token"))
}

fn key_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Token<'a, T>, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        let checkpoint = i.checkpoint();
        let key = trivial_key1.context(label("key")).parse_next(i)?;
        if i.starts_with(LEFT_DELIMITERS) {
            return prefix(key, ctx).map(Token::Default).parse_next(i);
        }
        let token = match key {
            EMPTY => Token::Empty(checkpoint),
            UNIT => Token::Default(T::from(Unit)),
            PAIR => Token::Pair(checkpoint),
            TRUE => Token::Default(T::from(Bit::true_())),
            FALSE => Token::Default(T::from(Bit::false_())),
            key => Token::Default(T::from(Key::from_str_unchecked(key))),
        };
        Ok(token)
    }
}

const LEFT_DELIMITERS: [char; 5] = [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, KEY_QUOTE, TEXT_QUOTE];

fn prefix<'a, T: ParseRepr>(prefix: &str, ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        match prefix {
            EMPTY => match i.chars().next().unwrap() {
                LIST_LEFT => raw_list(ctx).parse_next(i),
                MAP_LEFT => raw_map(ctx).parse_next(i),
                _ => fail.context(label("prefix token")).parse_next(i),
            },
            INT => int.map(T::from).parse_next(i),
            DECIMAL => decimal.map(T::from).parse_next(i),
            BYTE => byte.map(T::from).parse_next(i),
            LEFT => scope(ctx.direction(Direction::Left)).parse_next(i),
            RIGHT => scope(ctx.direction(Direction::Right)).parse_next(i),
            _ => cut_err(fail.context(label("prefix token"))).parse_next(i),
        }
    }
}

fn compose<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| match ctx.direction {
        Direction::Left => compose_left(ctx).parse_next(i),
        Direction::Right => compose_right(ctx).parse_next(i),
    }
}

fn compose_left<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let mut input = preceded(spaces_comment(ctx), input_token(ctx));
        let mut func = preceded(spaces_comment(ctx), func_token(ctx));
        let left = input_token(ctx).parse_next(i)?;
        let Some(middle) = opt(func.by_ref()).parse_next(i)? else {
            return input_repr(i, left);
        };
        let right = input.parse_next(i)?;
        let mut left = compose_one(ctx, i, left, middle, right)?;
        loop {
            let Some(middle) = opt(func.by_ref()).parse_next(i)? else {
                return Ok(left);
            };
            let right = input.parse_next(i)?;
            left = compose_one_left(ctx, left, middle, right);
        }
    }
}

fn compose_right<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let left = input_token(ctx).parse_next(i)?;
        let Some(middle) = opt(preceded(spaces_comment(ctx), func_token(ctx))).parse_next(i)?
        else {
            return input_repr(i, left);
        };
        compose_right_recursive(ctx, i, left, middle)
    }
}

fn compose_right_recursive<'a, T: ParseRepr>(
    ctx: ParseCtx, i: &mut &'a str, left: InputToken<'a, T>, middle: FuncToken<T>,
) -> ModalResult<T> {
    let right = preceded(spaces_comment(ctx), input_token(ctx)).parse_next(i)?;
    let Some(middle2) = opt(preceded(spaces_comment(ctx), func_token(ctx))).parse_next(i)? else {
        return compose_one(ctx, i, left, middle, right);
    };
    let right = compose_right_recursive(ctx, i, right, middle2)?;
    Ok(compose_one_right(ctx, left, middle, right))
}

fn compose_one<'a, T: ParseRepr>(
    ctx: ParseCtx, i: &mut &'a str, left: InputToken<'a, T>, func: FuncToken<T>,
    right: InputToken<'a, T>,
) -> ModalResult<T> {
    let input = match (left, right) {
        (InputToken::Default(left), InputToken::Default(right)) => T::from(Pair::new(left, right)),
        (InputToken::Default(left), InputToken::Empty(_)) => left,
        (InputToken::Empty(_), InputToken::Default(right)) => right,
        (InputToken::Empty(_), InputToken::Empty(checkpoint)) => {
            i.reset(&checkpoint);
            return Err(cut_expect_desc(concatcp!("at most one ", EMPTY)));
        }
    };
    Ok(compose_func_input(ctx, func, input))
}

fn compose_one_left<T: ParseRepr>(
    ctx: ParseCtx, left: T, func: FuncToken<T>, right: InputToken<T>,
) -> T {
    let input = match right {
        InputToken::Default(right) => T::from(Pair::new(left, right)),
        InputToken::Empty(_) => left,
    };
    compose_func_input(ctx, func, input)
}

fn compose_one_right<T: ParseRepr>(
    ctx: ParseCtx, left: InputToken<T>, func: FuncToken<T>, right: T,
) -> T {
    let input = match left {
        InputToken::Default(left) => T::from(Pair::new(left, right)),
        InputToken::Empty(_) => right,
    };
    compose_func_input(ctx, func, input)
}

fn compose_func_input<T: ParseRepr>(_ctx: ParseCtx, func: FuncToken<T>, input: T) -> T {
    match func {
        FuncToken::Pair => input,
        FuncToken::Default(func) => T::from(Call::new(func, input)),
    }
}

enum Token<'a, T> {
    Empty(Checkpoint<&'a str, &'a str>),
    Pair(Checkpoint<&'a str, &'a str>),
    Default(T),
}

enum FuncToken<T> {
    Pair,
    Default(T),
}

enum InputToken<'a, T> {
    Empty(Checkpoint<&'a str, &'a str>),
    Default(T),
}

const QUOTE_EMPTY: &str = concatcp!(KEY_QUOTE, EMPTY, KEY_QUOTE);
const QUOTE_PAIR: &str = concatcp!(KEY_QUOTE, PAIR, KEY_QUOTE);

fn func_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, FuncToken<T>, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Token::Empty(checkpoint) => {
            i.reset(&checkpoint);
            Err(cut_expect_desc(QUOTE_EMPTY))
        }
        Token::Pair(_) => Ok(FuncToken::Pair),
        Token::Default(token) => Ok(FuncToken::Default(token)),
    }
}

fn input_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, InputToken<'a, T>, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Token::Empty(checkpoint) => Ok(InputToken::Empty(checkpoint)),
        Token::Pair(checkpoint) => {
            i.reset(&checkpoint);
            Err(cut_expect_desc(QUOTE_PAIR))
        }
        Token::Default(token) => Ok(InputToken::Default(token)),
    }
}

fn repr<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Token::Empty(checkpoint) => {
            i.reset(&checkpoint);
            Err(cut_expect_desc(QUOTE_EMPTY))
        }
        Token::Pair(checkpoint) => {
            i.reset(&checkpoint);
            Err(cut_expect_desc(QUOTE_PAIR))
        }
        Token::Default(token) => Ok(token),
    }
}

fn input_repr<'a, T: ParseRepr>(i: &mut &'a str, input: InputToken<'a, T>) -> ModalResult<T> {
    match input {
        InputToken::Default(token) => Ok(token),
        InputToken::Empty(checkpoint) => {
            i.reset(&checkpoint);
            Err(cut_expect_desc(QUOTE_EMPTY))
        }
    }
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
    let f = delimited_trim_comment(ctx, LIST_LEFT, items, LIST_RIGHT);
    f.context(label("list"))
}

fn raw_list<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let repr_list = separated(0 .., repr::<T>(ctx), spaces_comment(ctx))
        .map(|tokens: Vec<_>| T::from(List::from(tokens)));
    let f = delimited_trim_comment(ctx, LIST_LEFT, repr_list, LIST_RIGHT);
    f.context(label("raw list"))
}

fn map<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut map = Map::default();
        let mut key = opt(any_key);
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
    let f = delimited_trim_comment(ctx, MAP_LEFT, items, MAP_RIGHT);
    f.context(label("map"))
}

fn raw_map<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let kv = (any_key, spaces_comment(ctx), repr::<T>(ctx));
        let tokens: Vec<_> = separated(0 .., kv, spaces_comment(ctx)).parse_next(i)?;
        let mut duplicate = fail.context(expect_desc("no duplicate keys"));
        let mut map = Map::with_capacity(tokens.len());
        for (key, (), value) in tokens {
            if map.contains_key(&key) {
                return duplicate.parse_next(i);
            }
            map.insert(key, value);
        }
        Ok(T::from(map))
    };
    let f = delimited_trim_comment(ctx, MAP_LEFT, items, MAP_RIGHT);
    f.context(label("raw map"))
}

fn any_key(i: &mut &str) -> ModalResult<Key> {
    alt((trivial_key1.map(Key::from_str_unchecked), key)).parse_next(i)
}

fn key(i: &mut &str) -> ModalResult<Key> {
    let key = move |i: &mut _| {
        let mut s = String::new();
        let mut literal = take_while(1 .., |c| is_key(c) && c != '^' && c != KEY_QUOTE);
        let mut raw_literal = take_while(0 .., is_key);
        let mut raw = false;
        loop {
            if raw {
                match peek(any).parse_next(i)? {
                    '\r' | '\n' => {
                        key_newline.parse_next(i)?;
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
                    KEY_QUOTE => break,
                    '^' => s.push_str(key_escaped.parse_next(i)?),
                    '\r' | '\n' => {
                        key_newline.parse_next(i)?;
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
        Ok(Key::from_string_unchecked(s))
    };
    delimited_cut(KEY_QUOTE, key, KEY_QUOTE).context(label("key")).parse_next(i)
}

fn key_escaped<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    let f = preceded('^', move |i: &mut _| match any.parse_next(i)? {
        '^' => empty.value("^").parse_next(i),
        '_' => empty.value(" ").parse_next(i),
        TEXT_QUOTE => empty.value(concatcp!(KEY_QUOTE)).parse_next(i),
        ' ' | '\t' => space_tab0.value("").parse_next(i),
        _ => fail.parse_next(i),
    });
    f.context(expect_desc("escape character")).parse_next(i)
}

fn key_newline(i: &mut &str) -> ModalResult<()> {
    let f = (line_ending, space_tab0, '|'.context(expect_char('|'))).void();
    f.context(expect_desc("newline")).parse_next(i)
}

fn text(i: &mut &str) -> ModalResult<Text> {
    let text = move |i: &mut _| {
        let i: &mut &str = i;
        let mut s = String::new();
        let mut literal = take_until(1 .., ('"', '^', '\n'));
        let mut raw_literal = take_until(1 .., '\n');
        let mut raw = false;
        loop {
            if raw {
                match peek(any).parse_next(i)? {
                    '\n' => {
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
                    '^' => text_escaped.parse_next(i)?.push(&mut s),
                    '\n' => {
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
    let f = preceded('^', move |i: &mut _| match any.parse_next(i)? {
        'u' => unicode.map(StrFragment::Char).parse_next(i),
        'n' => empty.value(StrFragment::Char('\n')).parse_next(i),
        'r' => empty.value(StrFragment::Char('\r')).parse_next(i),
        't' => empty.value(StrFragment::Char('\t')).parse_next(i),
        '^' => empty.value(StrFragment::Char('^')).parse_next(i),
        '_' => empty.value(StrFragment::Char(' ')).parse_next(i),
        KEY_QUOTE => empty.value(StrFragment::Char(TEXT_QUOTE)).parse_next(i),
        ' ' | '\t' => space_tab0.value(StrFragment::Str("")).parse_next(i),
        _ => fail.parse_next(i),
    });
    f.context(expect_desc("escape character")).parse_next(i)
}

fn unicode(i: &mut &str) -> ModalResult<char> {
    let digit = take_while(1 .. 7, is_hexadecimal);
    let f = delimited_cut(SCOPE_LEFT, digit, SCOPE_RIGHT)
        .verify_map(|hex| char::from_u32(u32::from_str_radix(hex, 16).unwrap()));
    f.context(expect_desc("unicode")).parse_next(i)
}

fn text_newline<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    let newline = alt(('+'.value(true), '|'.value(false)))
        .context(expect_char('+'))
        .context(expect_char('|'));
    let f = preceded(("\n", space_tab0), newline).map(|new| if new { "\n" } else { "" });
    f.context(expect_desc("newline")).parse_next(i)
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

fn number<T: ParseRepr>(i: &mut &str) -> ModalResult<T> {
    let int = norm_int.map(T::from);
    let decimal = norm_decimal.map(T::from);
    let prefixed = preceded('0', alt((int, decimal, plain_number)));
    let f = alt((prefixed, plain_number));
    let end = not(one_of(|c| is_trivial_key(c) || LEFT_DELIMITERS.contains(&c)));
    cut_err(terminated(f, end).context(label("number"))).parse_next(i)
}

fn int(i: &mut &str) -> ModalResult<Int> {
    let f = key.verify_map(|key| alt((norm_int, plain_int)).parse(&*key).ok());
    cut_err(f).context(label("int")).parse_next(i)
}

fn decimal(i: &mut &str) -> ModalResult<Decimal> {
    let f = key.verify_map(|key| alt((norm_decimal, plain_decimal)).parse(&*key).ok());
    cut_err(f).context(label("decimal")).parse_next(i)
}

fn norm_int(i: &mut &str) -> ModalResult<Int> {
    let hex = preceded('X', cut_err(int_radix(16, is_hexadecimal, "hexadecimal")));
    let bin = preceded('B', cut_err(int_radix(2, is_binary, "binary")));
    let dec = preceded('D', cut_err(int_radix(10, is_decimal, "decimal")));
    let f = (sign, alt((hex, bin, dec))).verify_map(|(sign, i)| {
        if i.is_zero() && sign != Sign::NoSign {
            return None;
        }
        let int = if sign == Sign::Minus { -i } else { i };
        Some(Int::new(int))
    });
    f.context(label("norm_int")).parse_next(i)
}

fn norm_decimal(i: &mut &str) -> ModalResult<Decimal> {
    let exp = preceded('E', cut_err(plain_int));
    let head = one_of(is_decimal);
    let tail = take_while(0 .., is_decimal);
    let f = (sign, exp, '*', head, '.', tail).verify_map(|(sign, exp, _, head, _, tail)| {
        let Ok(exp) = i64::try_from(exp.unwrap()) else {
            return None;
        };
        let significand = format!("{head}{tail}");
        let significand = BigInt::from_str(&significand).unwrap();
        if significand.is_zero() {
            if sign != Sign::NoSign {
                return None;
            }
        } else {
            if head == '0' {
                return None;
            }
        }
        let significand = if sign == Sign::Minus { -significand } else { significand };
        let scale = tail.len() as i64 - exp;
        let decimal = BigDecimal::from_bigint(significand, scale);
        Some(Decimal::new(decimal))
    });
    f.context(label("norm_decimal")).parse_next(i)
}

fn plain_number<T: ParseRepr>(i: &mut &str) -> ModalResult<T> {
    let integral = take_while(1 .., is_decimal);
    let fraction = opt(preceded('.', take_while(0 .., is_decimal)));
    let f = (sign, integral, fraction).verify_map(|(sign, int, frac)| {
        if let Some(frac) = frac {
            build_decimal(sign, int, frac).map(T::from)
        } else {
            build_int(sign, int).map(T::from)
        }
    });
    f.context(label("plain_number")).parse_next(i)
}

fn plain_int(i: &mut &str) -> ModalResult<Int> {
    let int = take_while(1 .., is_decimal);
    let f = (sign, int).verify_map(|(sign, int)| build_int(sign, int));
    f.context(label("plain_int")).parse_next(i)
}

fn plain_decimal(i: &mut &str) -> ModalResult<Decimal> {
    let integral = take_while(1 .., is_decimal);
    let fraction = take_while(0 .., is_decimal);
    let f = (sign, integral, '.', fraction)
        .verify_map(|(sign, int, _, frac)| build_decimal(sign, int, frac));
    f.context(label("plain_decimal")).parse_next(i)
}

fn sign(i: &mut &str) -> ModalResult<Sign> {
    alt(('+'.value(Sign::Plus), '-'.value(Sign::Minus), empty.value(Sign::NoSign))).parse_next(i)
}

fn int_radix<'a>(
    radix: u8, f: fn(char) -> bool, desc: &'static str,
) -> impl Parser<&'a str, BigInt, E> {
    let f = take_while(1 .., f).map(move |int| BigInt::from_str_radix(int, radix as u32).unwrap());
    f.context(expect_desc(desc))
}

fn build_int(sign: Sign, int: &str) -> Option<Int> {
    let int = BigInt::from_str(int).unwrap();
    if int.is_zero() && sign != Sign::NoSign {
        return None;
    }
    let int = if sign == Sign::Minus { -int } else { int };
    Some(Int::from(int))
}

fn build_decimal(sign: Sign, int: &str, frac: &str) -> Option<Decimal> {
    let significand = format!("{int}{frac}");
    let significand = BigInt::from_str(&significand).unwrap();
    if significand.is_zero() && sign != Sign::NoSign {
        return None;
    }
    let significand = if sign == Sign::Minus { -significand } else { significand };
    let scale = frac.len() as i64;
    let decimal = BigDecimal::from_bigint(significand, scale);
    Some(Decimal::new(decimal))
}

fn byte(i: &mut &str) -> ModalResult<Byte> {
    let f = key.verify_map(|key| {
        let hex = preceded('X', cut_err(hexadecimal_byte));
        let bin = preceded('B', cut_err(binary_byte));
        let mut byte = alt((hex, bin, hexadecimal_byte));
        byte.parse(&*key).ok()
    });
    cut_err(f).context(label("byte")).parse_next(i)
}

fn hexadecimal_byte(i: &mut &str) -> ModalResult<Byte> {
    let f = take_while(0 .., is_hexadecimal)
        .verify(|s: &str| s.len().is_multiple_of(2))
        .map(|s| Byte::from(hex_str_to_vec_u8(s).unwrap()));
    f.context(expect_desc("hexadecimal")).parse_next(i)
}

fn binary_byte(i: &mut &str) -> ModalResult<Byte> {
    let f = take_while(0 .., is_binary)
        .verify(|s: &str| s.len().is_multiple_of(8))
        .map(|s| Byte::from(bin_str_to_vec_u8(s).unwrap()));
    f.context(expect_desc("binary")).parse_next(i)
}

#[expect(clippy::manual_is_ascii_check)]
fn is_decimal(c: char) -> bool {
    matches!(c, '0' ..= '9')
}

fn is_hexadecimal(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

fn is_binary(c: char) -> bool {
    matches!(c, '0' ..= '1')
}
