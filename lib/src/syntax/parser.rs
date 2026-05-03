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
use winnow::token::take_until;
use winnow::token::take_while;

use super::BYTE;
use super::COMMENT;
use super::DECIMAL;
use super::Direction;
use super::EMPTY;
use super::EMPTY_CHAR;
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
use super::SOLVE;
use super::SPACE;
use super::TEXT_QUOTE;
use super::TOKEN;
use super::TRUE;
use super::UNIT;
use super::is_delimiter;
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
    + From<Cell<Self>>
    + From<Quote<Self>>
    + From<Pair<Self, Self>>
    + From<Call<Self, Self>>
    + From<Solve<Self, Self>>
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
    let ctx = ParseCtx::default();
    void_compose::<T>(ctx).parse(src).map_err(|e| super::ParseError { msg: e.to_string() })
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

fn spaces(i: &mut &str) -> ModalResult<()> {
    let spaces = take_while(1 .., |c| matches!(c, ' ' | '\t' | '\n'));
    let fail_spaces = fail
        .context(expect_char(' '))
        .context(expect_char('\t'))
        .context(expect_desc("`\\n`"))
        .context(expect_desc("`\\r\\n`"));
    let f = repeat(1 .., alt((spaces, "\r\n", fail_spaces)).void());
    f.context(label("spaces")).parse_next(i)
}

fn space_tab<'a>(range: impl Into<Range>) -> impl Parser<&'a str, (), E> {
    let f = take_while(range, |c| matches!(c, ' ' | '\t')).void();
    f.context(label("space_tab"))
}

fn void<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    let fail_void = fail
        .context(expect_char(' '))
        .context(expect_char('\t'))
        .context(expect_desc("`\\n`"))
        .context(expect_desc("`\\r\\n`"))
        .context(expect_desc("comment"));
    repeat(1 .., alt((spaces, comment(ctx), fail_void))).context(label("void"))
}

fn comment<'a>(ctx: ParseCtx) -> impl Parser<&'a str, (), E> {
    let comment_tokens = repeat(0 .., comment_token(ctx));
    let scope = delimited_cut(SCOPE_LEFT, comment_tokens, SCOPE_RIGHT);
    let comment =
        alt((scope, key.void(), text.void(), list::<C>(ctx).void(), map::<C>(ctx).void()));
    let f = preceded(COMMENT, comment);
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
impl_parse_repr_for_comment!(Cell<C> Quote<C> Pair<C, C> Call<C, C> Solve<C, C> List<C> Map<Key, C>);
impl ParseRepr for C {}

fn delimited_cut<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    let left = left.context(expect_char(left));
    let right = right.context(expect_char(right));
    delimited(left, cut_err(f), cut_err(right))
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

fn token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Either<'a, T, Token>, E> {
    let f = move |i: &mut _| match peek(any).parse_next(i)? {
        LIST_LEFT => list(ctx).map(Either::Repr).parse_next(i),
        LIST_RIGHT => fail.parse_next(i),
        MAP_LEFT => map(ctx).map(Either::Repr).parse_next(i),
        MAP_RIGHT => fail.parse_next(i),
        SCOPE_LEFT => scope(ctx).map(Either::Repr).parse_next(i),
        SCOPE_RIGHT => fail.parse_next(i),
        SEPARATOR => fail.parse_next(i),
        SPACE => fail.parse_next(i),
        TEXT_QUOTE => text.map(T::from).map(Either::Repr).parse_next(i),
        KEY_QUOTE => key.map(T::from).map(Either::Repr).parse_next(i),
        '0' ..= '9' => number.map(Either::Repr).parse_next(i),
        _ => cut_err(key_token(ctx)).parse_next(i),
    };
    f.context(label("token"))
}

fn key_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, Either<'a, T, Token>, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        let checkpoint = i.checkpoint();
        let key = trivial_key1.context(label("key")).parse_next(i)?;
        if i.starts_with(LEFT_DELIMITERS) {
            return prefix(key, ctx).map(Either::Repr).parse_next(i);
        }
        let token = match key {
            EMPTY => Either::Token { checkpoint, token: Token::Empty },
            UNIT => Either::Repr(T::from(Unit)),
            PAIR => Either::Token { checkpoint, token: Token::Pair },
            SOLVE => Either::Token { checkpoint, token: Token::Solve },
            TRUE => Either::Repr(T::from(Bit::true_())),
            FALSE => Either::Repr(T::from(Bit::false_())),
            key => Either::Repr(T::from(Key::from_str_unchecked(key))),
        };
        Ok(token)
    }
}

const LEFT_DELIMITERS: [char; 5] = [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, KEY_QUOTE, TEXT_QUOTE];

fn prefix<'a, T: ParseRepr>(prefix: &str, ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        match prefix {
            TOKEN => match i.chars().next().unwrap() {
                LIST_LEFT => list_token(ctx).parse_next(i),
                MAP_LEFT => map_token(ctx).parse_next(i),
                _ => fail.context(label("prefix token")).parse_next(i),
            },
            EMPTY => quote(ctx).parse_next(i),
            UNIT => cell(ctx).parse_next(i),
            INT => int.map(T::from).parse_next(i),
            DECIMAL => decimal.map(T::from).parse_next(i),
            BYTE => byte.map(T::from).parse_next(i),
            LEFT => scope(ctx.direction(Direction::Left)).parse_next(i),
            RIGHT => scope(ctx.direction(Direction::Right)).parse_next(i),
            _ => cut_err(fail.context(label("prefix token"))).parse_next(i),
        }
    }
}

fn scope<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    delimited_cut(SCOPE_LEFT, void_compose(ctx), SCOPE_RIGHT).context(label("scope"))
}

fn void_compose<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    preceded(opt(void(ctx)), compose(ctx))
}

fn compose<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| match ctx.direction {
        Direction::Left => compose_left(ctx).parse_next(i),
        Direction::Right => compose_right(ctx).parse_next(i),
    }
}

fn compose_left<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let mut void = void(ctx);
        let mut input = input_token(ctx);
        let mut opt_func = opt_func_token(ctx);
        let left = input.parse_next(i)?;
        let Some(middle) = opt_func.parse_next(i)? else {
            return input_repr(i, left);
        };
        void.parse_next(i)?;
        let right = input.parse_next(i)?;
        let mut left = compose_infix(i, left, middle, right)?;
        loop {
            let Some(middle) = opt_func.parse_next(i)? else {
                return Ok(left);
            };
            void.parse_next(i)?;
            let right = input.parse_next(i)?;
            left = compose_left_one(i, left, middle, right)?;
        }
    }
}

fn compose_left_one<'a, T: ParseRepr>(
    i: &mut &'a str, left: T, middle: Either<'a, T, FuncToken>, right: Either<'a, T, InputToken>,
) -> ModalResult<T> {
    let func = match middle {
        Either::Repr(func) => func,
        Either::Token { token, .. } => {
            let FuncToken::Pair = token;
            return match right {
                Either::Repr(right) => Ok(T::from(Pair::new(left, right))),
                Either::Token { checkpoint, token } => {
                    reset_expect(i, checkpoint, input_token_expect(token))
                },
            };
        },
    };
    let (is_solve, argument) = match right {
        Either::Repr(right) => (false, T::from(Pair::new(left, right))),
        Either::Token { token: right, .. } => (right == InputToken::Solve, left),
    };
    Ok(call_solve(is_solve, func, argument))
}

fn compose_right<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| {
        let left = input_token(ctx).parse_next(i)?;
        let Some(middle) = opt_func_token(ctx).parse_next(i)? else {
            return input_repr(i, left);
        };
        void(ctx).parse_next(i)?;
        compose_right_recursive(ctx, i, left, middle)
    }
}

fn compose_right_recursive<'a, T: ParseRepr>(
    ctx: ParseCtx, i: &mut &'a str, left: Either<'a, T, InputToken>,
    middle: Either<'a, T, FuncToken>,
) -> ModalResult<T> {
    let right = input_token(ctx).parse_next(i)?;
    let Some(middle2) = opt_func_token(ctx).parse_next(i)? else {
        return compose_infix(i, left, middle, right);
    };
    void(ctx).parse_next(i)?;
    let right = compose_right_recursive(ctx, i, right, middle2)?;
    let func = match middle {
        Either::Repr(func) => func,
        Either::Token { checkpoint, token } => {
            let FuncToken::Pair = token;
            return if let Either::Repr(left) = left {
                Ok(T::from(Pair::new(left, right)))
            } else {
                reset_expect(i, checkpoint, QUOTE_PAIR)
            };
        },
    };
    let (is_solve, argument) = match left {
        Either::Repr(left) => (false, T::from(Pair::new(left, right))),
        Either::Token { token: left, .. } => (left == InputToken::Solve, right),
    };
    Ok(call_solve(is_solve, func, argument))
}

fn opt_func_token<'a, T: ParseRepr>(
    ctx: ParseCtx,
) -> impl Parser<&'a str, Option<Either<'a, T, FuncToken>>, E> {
    move |i: &mut _| {
        // consume void
        if opt(void(ctx)).parse_next(i)?.is_none() {
            return Ok(None);
        }
        opt(func_token(ctx)).parse_next(i)
    }
}

fn compose_infix<'a, T: ParseRepr>(
    i: &mut &'a str, left: Either<'a, T, InputToken>, func: Either<'a, T, FuncToken>,
    right: Either<'a, T, InputToken>,
) -> ModalResult<T> {
    let func = match func {
        Either::Repr(func) => func,
        Either::Token { checkpoint, token } => {
            let FuncToken::Pair = token;
            let left = match left {
                Either::Repr(left) => left,
                Either::Token { .. } => return reset_expect(i, checkpoint, QUOTE_PAIR),
            };
            let right = match right {
                Either::Repr(right) => right,
                Either::Token { checkpoint, token } => {
                    return reset_expect(i, checkpoint, input_token_expect(token));
                },
            };
            return Ok(T::from(Pair::new(left, right)));
        },
    };
    let (is_solve, argument) = match (left, right) {
        (Either::Repr(left), Either::Repr(right)) => (false, T::from(Pair::new(left, right))),
        (Either::Repr(left), Either::Token { token: right, .. }) => {
            (right == InputToken::Solve, left)
        },
        (Either::Token { token: left, .. }, Either::Repr(right)) => {
            (left == InputToken::Solve, right)
        },
        (Either::Token { .. }, Either::Token { checkpoint, token: right }) => {
            let expect = match right {
                InputToken::Empty => QUOTE_EMPTY,
                InputToken::Solve => QUOTE_SOLVE,
            };
            return reset_expect(i, checkpoint, expect);
        },
    };
    Ok(call_solve(is_solve, func, argument))
}

fn call_solve<T: ParseRepr>(is_solve: bool, func: T, argument: T) -> T {
    if is_solve { T::from(Solve::new(func, argument)) } else { T::from(Call::new(func, argument)) }
}

enum Either<'a, Repr, Token> {
    Repr(Repr),
    Token { checkpoint: Checkpoint<&'a str, &'a str>, token: Token },
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Token {
    Empty,
    Pair,
    Solve,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum FuncToken {
    Pair,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum InputToken {
    Empty,
    Solve,
}

const QUOTE_EMPTY: &str = concatcp!(KEY_QUOTE, EMPTY, KEY_QUOTE);
const QUOTE_PAIR: &str = concatcp!(KEY_QUOTE, PAIR, KEY_QUOTE);
const QUOTE_SOLVE: &str = concatcp!(KEY_QUOTE, SOLVE, KEY_QUOTE);

fn func_token<'a, T: ParseRepr>(
    ctx: ParseCtx,
) -> impl Parser<&'a str, Either<'a, T, FuncToken>, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Either::Repr(repr) => Ok(Either::Repr(repr)),
        Either::Token { checkpoint, token } => {
            let expect = match token {
                Token::Empty => QUOTE_EMPTY,
                Token::Pair => {
                    return Ok(Either::Token { checkpoint, token: FuncToken::Pair });
                },
                Token::Solve => QUOTE_SOLVE,
            };
            reset_expect(i, checkpoint, expect)
        },
    }
}

fn input_token<'a, T: ParseRepr>(
    ctx: ParseCtx,
) -> impl Parser<&'a str, Either<'a, T, InputToken>, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Either::Repr(repr) => Ok(Either::Repr(repr)),
        Either::Token { checkpoint, token } => {
            let token = match token {
                Token::Empty => InputToken::Empty,
                Token::Pair => return reset_expect(i, checkpoint, QUOTE_PAIR),
                Token::Solve => InputToken::Solve,
            };
            Ok(Either::Token { checkpoint, token })
        },
    }
}

fn repr<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    move |i: &mut _| match token(ctx).parse_next(i)? {
        Either::Repr(repr) => Ok(repr),
        Either::Token { checkpoint, token } => {
            let expect = match token {
                Token::Empty => QUOTE_EMPTY,
                Token::Pair => QUOTE_PAIR,
                Token::Solve => QUOTE_SOLVE,
            };
            reset_expect(i, checkpoint, expect)
        },
    }
}

fn input_repr<'a, T: ParseRepr>(
    i: &mut &'a str, input: Either<'a, T, InputToken>,
) -> ModalResult<T> {
    match input {
        Either::Repr(token) => Ok(token),
        Either::Token { checkpoint, token } => {
            reset_expect(i, checkpoint, input_token_expect(token))
        },
    }
}

fn input_token_expect(token: InputToken) -> &'static str {
    match token {
        InputToken::Empty => QUOTE_EMPTY,
        InputToken::Solve => QUOTE_SOLVE,
    }
}

fn reset_expect<'a, T>(
    i: &mut &'a str, checkpoint: Checkpoint<&'a str, &'a str>, expect: &'static str,
) -> ModalResult<T> {
    i.reset(&checkpoint);
    Err(cut_expect_desc(expect))
}

fn quote<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let quote = alt((scope(ctx), key.map(T::from), text.map(T::from), list(ctx), map(ctx)))
        .map(|v| T::from(Quote::new(v)));
    quote.context(label("quote"))
}

fn cell<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let cell = alt((scope(ctx), key.map(T::from), text.map(T::from), list(ctx), map(ctx)))
        .map(|v| T::from(Cell::new(v)));
    cell.context(label("cell"))
}

fn list<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut opt_void = opt(void(ctx));
        let mut repr = opt(compose(ctx));
        let mut separator = opt(SEPARATOR.context(expect_char(SEPARATOR)));
        let mut list = Vec::new();
        loop {
            opt_void.parse_next(i)?;
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
    let f = delimited_cut(LIST_LEFT, items, LIST_RIGHT);
    f.context(label("list"))
}

fn list_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut opt_void = opt(void(ctx));
        let mut opt_repr = opt(repr(ctx));
        let mut list = Vec::new();
        opt_void.parse_next(i)?;
        loop {
            let Some(item) = opt_repr.parse_next(i)? else {
                break;
            };
            list.push(item);
            if opt_void.parse_next(i)?.is_none() {
                break;
            }
        }
        Ok(T::from(List::from(list)))
    };
    let f = delimited_cut(LIST_LEFT, items, LIST_RIGHT);
    f.context(label("token list"))
}

fn map<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut opt_void = opt(void(ctx));
        let mut void = void(ctx);
        let mut opt_key = opt(any_key);
        let mut pair = opt(PAIR.void());
        let mut value = cut_err(compose(ctx).context(expect_desc("value")));
        let mut separator = opt(SEPARATOR.context(expect_char(SEPARATOR)));
        let mut duplicate = fail.context(expect_desc("no duplicate keys"));
        let mut map = Map::default();
        loop {
            opt_void.parse_next(i)?;
            let Some(k) = opt_key.parse_next(i)? else {
                break;
            };
            if map.contains_key(&k) {
                return duplicate.parse_next(i);
            }
            let v = if opt_void.parse_next(i)?.is_some() {
                if pair.parse_next(i)?.is_some() {
                    void.parse_next(i)?;
                    value.parse_next(i)?
                } else {
                    T::from(Unit)
                }
            } else {
                T::from(Unit)
            };
            map.insert(k, v);
            if separator.parse_next(i)?.is_none() {
                break;
            }
        }
        Ok(T::from(map))
    };
    let f = delimited_cut(MAP_LEFT, items, MAP_RIGHT);
    f.context(label("map"))
}

fn map_token<'a, T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&'a str, T, E> {
    let items = move |i: &mut _| {
        let mut opt_void = opt(void(ctx));
        let mut void = void(ctx);
        let mut opt_key = opt(any_key);
        let mut value = cut_err(repr(ctx).context(expect_desc("value")));
        let mut duplicate = fail.context(expect_desc("no duplicate keys"));
        let mut map = Map::default();
        opt_void.parse_next(i)?;
        loop {
            let Some(k) = opt_key.parse_next(i)? else {
                break;
            };
            if map.contains_key(&k) {
                return duplicate.parse_next(i);
            }
            void.parse_next(i)?;
            let v = value.parse_next(i)?;
            map.insert(k, v);
            if opt_void.parse_next(i)?.is_none() {
                break;
            }
        }
        Ok(T::from(map))
    };
    let f = delimited_cut(MAP_LEFT, items, MAP_RIGHT);
    f.context(label("token map"))
}

fn any_key(i: &mut &str) -> ModalResult<Key> {
    alt((trivial_key1.map(Key::from_str_unchecked), key)).parse_next(i)
}

fn key(i: &mut &str) -> ModalResult<Key> {
    let key = take_while(1 .., |c| is_key(c) && c != KEY_QUOTE);
    let text = take_while(1 .., |c| is_key(c) && c != TEXT_QUOTE);
    let comment = take_until(0 .., ('\r', '\n', SCOPE_RIGHT)).void();
    let token = take_while(1 .., |c| is_key(c) && c != ' ' && c != LIST_RIGHT)
        .verify_map(character)
        .verify(|c| is_key(*c));
    let newline = (line_ending, space_tab(0 ..), '|'.context(expect_char('|'))).value("");
    let key = key_text(key, text, comment, token, newline).map(Key::from_string_unchecked);
    preceded(peek(KEY_QUOTE), key).context(label("key")).parse_next(i)
}

fn text(i: &mut &str) -> ModalResult<Text> {
    let key = take_until(1 .., (KEY_QUOTE, '\r', '\n'));
    let text = take_until(1 .., (TEXT_QUOTE, '\r', '\n'));
    let comment = take_until(0 .., (SCOPE_RIGHT, '\n')).void();
    let token =
        take_while(1 .., |c| is_key(c) && c != ' ' && c != LIST_RIGHT).verify_map(character);
    let fail_newline =
        fail.context(expect_char('|')).context(expect_char('.')).context(expect_char(':'));
    let newline = alt((
        '|'.value(NewLine::None),
        '.'.value(NewLine::Lf),
        ':'.value(NewLine::Crlf),
        fail_newline,
    ));
    #[expect(clippy::redundant_closure_for_method_calls)]
    let newline = preceded((line_ending, space_tab(0 ..)), newline).map(|newline| newline.as_str());
    let text = key_text(key, text, comment, token, newline).map(Text::from);
    preceded(peek(TEXT_QUOTE), text).context(label("text")).parse_next(i)
}

#[derive(Copy, Clone)]
enum NewLine {
    None,
    Lf,
    Crlf,
}

impl NewLine {
    fn as_str(self) -> &'static str {
        match self {
            NewLine::None => "",
            NewLine::Lf => "\n",
            NewLine::Crlf => "\r\n",
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    Clean,
    More,
    Key,
    Text,
    Comment,
    Token,
}

fn key_text<'a>(
    mut key: impl Parser<&'a str, &'a str, E>, mut text: impl Parser<&'a str, &'a str, E>,
    mut comment: impl Parser<&'a str, (), E>, mut token: impl Parser<&'a str, char, E>,
    mut newline: impl Parser<&'a str, &'a str, E>,
) -> impl Parser<&'a str, String, E> {
    fn clean(i: &mut &str, state: &mut State) -> ModalResult<()> {
        *state = State::Clean;
        any.void().parse_next(i)
    }
    move |i: &mut _| {
        let i: &mut &str = i;
        let mut peek_one = peek(any);

        let mut state = State::Clean;
        let mut s = String::new();
        loop {
            match state {
                State::Clean => {
                    if i.is_empty() {
                        break;
                    }
                    state = match peek_one.parse_next(i)? {
                        KEY_QUOTE => State::Key,
                        TEXT_QUOTE => State::Text,
                        SCOPE_LEFT => State::Comment,
                        LIST_LEFT => State::Token,
                        EMPTY_CHAR => State::More,
                        _ => break,
                    };
                    any.parse_next(i)?;
                },
                State::More => {
                    state = match peek_one.parse_next(i)? {
                        KEY_QUOTE => State::Key,
                        TEXT_QUOTE => State::Text,
                        SCOPE_LEFT => State::Comment,
                        LIST_LEFT => State::Token,
                        '\r' | '\n' => {
                            s.push_str(newline.parse_next(i)?);
                            continue;
                        },
                        _ => {
                            return fail
                                .context(expect_char(KEY_QUOTE))
                                .context(expect_char(TEXT_QUOTE))
                                .context(expect_char(SCOPE_LEFT))
                                .context(expect_char(LIST_LEFT))
                                .context(expect_desc("`\\n`"))
                                .context(expect_desc("`\\r\\n`"))
                                .parse_next(i);
                        },
                    };
                    any.parse_next(i)?;
                },
                State::Key => match peek_one.parse_next(i)? {
                    KEY_QUOTE => clean(i, &mut state)?,
                    '\r' | '\n' => s.push_str(newline.parse_next(i)?),
                    _ => s.push_str(key.parse_next(i)?),
                },
                State::Text => match peek_one.parse_next(i)? {
                    TEXT_QUOTE => clean(i, &mut state)?,
                    '\r' | '\n' => s.push_str(newline.parse_next(i)?),
                    _ => s.push_str(text.parse_next(i)?),
                },
                State::Comment => match peek_one.parse_next(i)? {
                    SCOPE_RIGHT => clean(i, &mut state)?,
                    '\r' | '\n' => s.push_str(newline.parse_next(i)?),
                    _ => comment.parse_next(i)?,
                },
                State::Token => match peek_one.parse_next(i)? {
                    LIST_RIGHT => clean(i, &mut state)?,
                    '\r' | '\n' => s.push_str(newline.parse_next(i)?),
                    ' ' => any.void().parse_next(i)?,
                    _ => s.push(token.parse_next(i)?),
                },
            }
        }
        Ok(s)
    }
}

fn character(s: &str) -> Option<char> {
    if s.len() == 1 {
        return Some(s.chars().next().unwrap());
    }
    if let Some(code) = s.strip_prefix('X') {
        if !code.chars().all(is_hexadecimal) {
            return None;
        }
        let Ok(i) = u32::from_str_radix(code, 16) else {
            return None;
        };
        return char::from_u32(i);
    }
    let c = match s {
        "nul" => '\u{00}',
        "soh" => '\u{01}',
        "stx" => '\u{02}',
        "etx" => '\u{03}',
        "eot" => '\u{04}',
        "enq" => '\u{05}',
        "ack" => '\u{06}',
        "bel" => '\u{07}',
        "bs" => '\u{08}',
        "ht" => '\u{09}',
        "lf" => '\u{0A}',
        "vt" => '\u{0B}',
        "ff" => '\u{0C}',
        "cr" => '\u{0D}',
        "so" => '\u{0E}',
        "si" => '\u{0F}',
        "dle" => '\u{10}',
        "dc1" => '\u{11}',
        "dc2" => '\u{12}',
        "dc3" => '\u{13}',
        "dc4" => '\u{14}',
        "nak" => '\u{15}',
        "syn" => '\u{16}',
        "etb" => '\u{17}',
        "can" => '\u{18}',
        "em" => '\u{19}',
        "sub" => '\u{1A}',
        "esc" => '\u{1B}',
        "fs" => '\u{1C}',
        "gs" => '\u{1D}',
        "rs" => '\u{1E}',
        "us" => '\u{1F}',
        "sp" => '\u{20}',
        "del" => '\u{7F}',
        _ => return None,
    };
    Some(c)
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
