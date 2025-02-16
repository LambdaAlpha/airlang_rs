use std::{
    hash::Hash,
    ops::Neg,
    str::FromStr,
};

use const_format::concatcp;
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
    abstract1::Abstract,
    ask::Ask,
    bit::Bit,
    byte::Byte,
    call::Call,
    change::Change,
    int::Int,
    list::List,
    map::Map,
    number::Number,
    pair::Pair,
    symbol::Symbol,
    syntax::{
        ABSTRACT,
        ARITY_2,
        ARITY_3,
        ASK,
        BYTE,
        CALL,
        CHANGE,
        FALSE,
        INT,
        LEFT,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        NUMBER,
        PAIR,
        RIGHT,
        SCOPE_LEFT,
        SCOPE_RIGHT,
        SEPARATOR,
        SPACE,
        SYMBOL_QUOTE,
        TAG,
        TAG_CHAR,
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
    + From<Bit>
    + From<Symbol>
    + From<Text>
    + From<Int>
    + From<Number>
    + From<Byte>
    + From<Pair<Self, Self>>
    + From<Abstract<Self, Self>>
    + From<Call<Self, Self>>
    + From<Ask<Self, Self>>
    + From<Change<Self, Self>>
    + From<List<Self>>
    + Eq
    + Hash
    + From<Map<Self, Self>>
    + Clone
{
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct ParseCtx<'a> {
    is_tag: bool,
    tag: &'a str,
    arity: Arity,
    struct1: Struct,
    direction: Direction,
}

impl Default for ParseCtx<'_> {
    fn default() -> Self {
        Self {
            is_tag: false,
            tag: "",
            arity: Arity::Three,
            struct1: Struct::Call,
            direction: Direction::Right,
        }
    }
}

impl<'a> ParseCtx<'a> {
    fn tag(mut self, tag: &'a str) -> Self {
        self.is_tag = true;
        self.tag = tag;
        self
    }

    fn escape(mut self) -> Self {
        self.is_tag = false;
        self
    }

    fn with_struct(mut self, struct1: Struct) -> Self {
        self.struct1 = struct1;
        self
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Arity {
    Two,
    Three,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Struct {
    Pair,
    Call,
    Abstract,
    Ask,
    Change,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
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
    let f = all_consuming(trim(ComposeParser::new(ParseCtx::default())));
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

#[derive(Copy, Clone)]
struct ScopeParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, T, E> for ScopeParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, T, E> {
        let f = delimited_trim(SCOPE_LEFT, ComposeParser::new(self.ctx), SCOPE_RIGHT);
        context("scope", f)(input)
    }
}

impl<'a> ScopeParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx }
    }
}

#[derive(Copy, Clone)]
struct CtxParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, E> Parser<&'a str, ParseCtx<'a>, E> for CtxParser<'a>
where
    E: ParseError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, ParseCtx<'a>, E> {
        let mut ctx = self.ctx.escape();
        let mut direction = 0;
        let mut arity = 0;
        for c in input.chars() {
            match c {
                LEFT => {
                    direction += 1;
                    ctx.direction = Direction::Left;
                }
                RIGHT => {
                    direction += 1;
                    ctx.direction = Direction::Right;
                }
                ARITY_2 => {
                    arity += 1;
                    ctx.arity = Arity::Two;
                }
                ARITY_3 => {
                    arity += 1;
                    ctx.arity = Arity::Three;
                }
                _ => return fail(input),
            }
        }
        if direction > 1 || arity > 1 {
            return fail(input);
        }
        Ok(("", ctx))
    }
}

impl<'a> CtxParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx }
    }

    fn is_ctx(c: char) -> bool {
        matches!(c, LEFT | RIGHT | ARITY_2 | ARITY_3)
    }
}

#[derive(Copy, Clone)]
struct TokenParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, Token<T>, E> for TokenParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, Token<T>, E> {
        let (src, first) = peek(anychar)(input)?;

        let parser = move |s| match first {
            // delimiters
            LIST_LEFT => map(ListParser::new(self.ctx), Token::Default)(s),
            LIST_RIGHT => fail(s),
            MAP_LEFT => map(MapParser::new(self.ctx), Token::Default)(s),
            MAP_RIGHT => fail(s),
            SCOPE_LEFT => map(ScopeParser::new(self.ctx), Token::Default)(s),
            SCOPE_RIGHT => fail(s),
            SEPARATOR => fail(s),
            SPACE => fail(s),
            TEXT_QUOTE => map(text, Token::Default)(s),
            SYMBOL_QUOTE => map(symbol, Token::Default)(s),

            sym if is_symbol(sym) => ExtParser::new(self.ctx).parse(s),
            _ => fail(s),
        };
        context("token", parser)(src)
    }
}

impl<'a> TokenParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx }
    }
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

#[derive(Copy, Clone)]
struct ExtParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, Token<T>, E> for ExtParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, Token<T>, E> {
        let (rest, s) = take_while(is_trivial_symbol)(input)?;

        // the only special case
        let first = s.chars().next().unwrap();
        const LEFT_DELIMITERS: [char; 5] =
            [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, SYMBOL_QUOTE, TEXT_QUOTE];
        if first.is_ascii_digit() && !rest.starts_with(LEFT_DELIMITERS) {
            let (_, token) = all_consuming(int_or_number)(s)?;
            return Ok((rest, Token::Default(token)));
        }

        let mut f = alt((
            map(PrefixParser::new(s, self.ctx), Token::Default),
            map(success(Symbol::from_str(s)), Token::Unquote),
        ));
        f(rest)
    }
}

impl<'a> ExtParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx }
    }
}

#[derive(Copy, Clone)]
struct PrefixParser<'a> {
    prefix: &'a str,
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, T, E> for PrefixParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, src: &'a str) -> IResult<&'a str, T, E> {
        match self.prefix {
            UNIT => alt((
                raw_text,
                raw_symbol,
                ListParser::new_raw(self.ctx),
                success(From::from(Unit)),
            ))(src),
            TRUE => success(From::from(Bit::true1()))(src),
            FALSE => success(From::from(Bit::false1()))(src),
            INT => int(src),
            NUMBER => number(src),
            BYTE => byte(src),
            PAIR => {
                let ctx = self.ctx.escape().with_struct(Struct::Pair);
                ScopeParser::new(ctx).parse(src)
            }
            CALL => {
                let ctx = self.ctx.escape().with_struct(Struct::Call);
                ScopeParser::new(ctx).parse(src)
            }
            ABSTRACT => {
                let ctx = self.ctx.escape().with_struct(Struct::Abstract);
                ScopeParser::new(ctx).parse(src)
            }
            ASK => {
                let ctx = self.ctx.escape().with_struct(Struct::Ask);
                ScopeParser::new(ctx).parse(src)
            }
            CHANGE => {
                let ctx = self.ctx.escape().with_struct(Struct::Change);
                ScopeParser::new(ctx).parse(src)
            }
            TAG => ScopeParser::new(self.ctx.escape()).parse(src),
            s if s.starts_with(TAG_CHAR) => ScopeParser::new(self.ctx.tag(&s[1..])).parse(src),
            s if s.chars().all(CtxParser::is_ctx) => {
                let ctx = context("ctx", CtxParser::new(self.ctx)).parse(s)?.1;
                ScopeParser::new(ctx).parse(src)
            }
            _ => fail(src),
        }
    }
}

impl<'a> PrefixParser<'a> {
    fn new(prefix: &'a str, ctx: ParseCtx<'a>) -> Self {
        Self { prefix, ctx }
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
            Token::Unquote(s) => From::from(s),
            Token::Default(r) => r,
        }
    }
}

#[derive(Copy, Clone)]
struct ComposeParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, T, E> for ComposeParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, T, E> {
        let tokens = separated_list1(empty1, TokenParser::new(self.ctx));
        let f = map_opt(tokens, |tokens| self.compose_tokens(tokens.into_iter()));
        context("compose", f)(input)
    }
}

impl<'a> ComposeParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        ComposeParser { ctx }
    }

    fn compose_tokens<T, I>(&self, mut tokens: I) -> Option<T>
    where
        T: ParseRepr,
        I: ExactSizeIterator<Item = Token<T>> + DoubleEndedIterator<Item = Token<T>>,
    {
        let len = tokens.len();
        if len == 0 {
            return None;
        }
        if self.ctx.is_tag {
            return self.compose_tag(tokens, self.ctx.tag);
        }
        if len == 1 {
            let repr = tokens.next().unwrap().into_repr();
            return Some(repr);
        }
        if len == 2 {
            let func = tokens.next().unwrap().into_repr();
            let input = tokens.next().unwrap().into_repr();
            return Some(self.compose_two(func, input));
        }
        match self.ctx.arity {
            Arity::Two => match self.ctx.direction {
                Direction::Left => self.compose_many2(tokens),
                Direction::Right => self.compose_many2(tokens.rev()),
            },
            Arity::Three => {
                if len % 2 == 0 {
                    return None;
                }
                match self.ctx.direction {
                    Direction::Left => self.compose_many3(tokens),
                    Direction::Right => self.compose_many3(tokens.rev()),
                }
            }
        }
    }

    fn compose_tag<T, I>(&self, tokens: I, tag: &str) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>,
    {
        let list = tokens.map(Token::into_repr).collect::<List<_>>();
        let list = From::from(list);
        let tag = From::from(Symbol::from_str(tag));
        let repr = self.compose_two(tag, list);
        Some(repr)
    }

    fn compose_two<T: ParseRepr>(&self, left: T, right: T) -> T {
        match self.ctx.struct1 {
            Struct::Pair => From::from(Pair::new(left, right)),
            Struct::Call => From::from(Call::new(left, right)),
            Struct::Abstract => From::from(Abstract::new(left, right)),
            Struct::Ask => From::from(Ask::new(left, right)),
            Struct::Change => From::from(Change::new(left, right)),
        }
    }

    fn compose_many2<T, I>(&self, mut iter: I) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>,
    {
        let mut first = iter.next().unwrap();
        loop {
            let Some(second) = iter.next() else {
                break;
            };
            let first1 = first.into_repr();
            let second = second.into_repr();
            let (left, right) = match self.ctx.direction {
                Direction::Left => (first1, second),
                Direction::Right => (second, first1),
            };
            first = Token::Default(self.compose_two(left, right));
        }
        Some(first.into_repr())
    }

    fn compose_many3<T, I>(&self, mut iter: I) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>,
    {
        let mut first = iter.next().unwrap();
        loop {
            let Some(middle) = iter.next() else {
                break;
            };
            let first1 = first.into_repr();
            let last = iter.next()?.into_repr();
            let (left, right) = match self.ctx.direction {
                Direction::Left => (first1, last),
                Direction::Right => (last, first1),
            };
            first = Token::Default(self.compose_infix(left, middle, right));
        }
        Some(first.into_repr())
    }

    fn compose_infix<T: ParseRepr>(&self, left: T, middle: Token<T>, right: T) -> T {
        let middle = match middle {
            Token::Unquote(s) => match &*s {
                PAIR => return From::from(Pair::new(left, right)),
                CALL => return From::from(Call::new(left, right)),
                ASK => return From::from(Ask::new(left, right)),
                ABSTRACT => return From::from(Abstract::new(left, right)),
                CHANGE => return From::from(Change::new(left, right)),
                _ => From::from(s),
            },
            Token::Default(middle) => middle,
        };
        let pair = Pair::new(left, right);
        let pair = From::from(pair);
        self.compose_two(middle, pair)
    }
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

#[derive(Copy, Clone)]
struct ListParser<'a> {
    raw: bool,
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, T, E> for ListParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, T, E> {
        if self.raw {
            let items = separated_list0(empty1, TokenParser::new(self.ctx));
            let items = map(items, |tokens| {
                let list: List<T> = tokens.into_iter().map(Token::into_repr).collect();
                T::from(list)
            });
            let f = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
            context("raw_list", f)(input)
        } else {
            let items = items(ComposeParser::new(self.ctx), char1(SEPARATOR));
            let items = map(items, |list| From::from(List::from(list)));
            let f = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
            context("list", f)(input)
        }
    }
}

impl<'a> ListParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        ListParser { raw: false, ctx }
    }

    fn new_raw(ctx: ParseCtx<'a>) -> Self {
        Self { raw: true, ctx }
    }
}

#[derive(Copy, Clone)]
struct MapParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T, E> Parser<&'a str, T, E> for MapParser<'a>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    fn parse(&mut self, input: &'a str) -> IResult<&'a str, T, E> {
        let items = items(|s| self.key_value(s), char1(SEPARATOR));
        let delimited_items = delimited_trim(MAP_LEFT, items, MAP_RIGHT);
        let f = map(delimited_items, |pairs| From::from(Map::from_iter(pairs)));
        context("map", f)(input)
    }
}

impl<'a> MapParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        MapParser { ctx }
    }

    fn key_value<T, E>(&self, src: &'a str) -> IResult<&'a str, (T, T), E>
    where
        T: ParseRepr,
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, tokens) = separated_list1(empty1, TokenParser::new(self.ctx))(src)?;
        let mut tokens = tokens.into_iter();
        let key = [tokens.next().unwrap()].into_iter();
        let key = ComposeParser::new(self.ctx).compose_tokens(key).unwrap();
        if tokens.len() == 0 {
            return Ok((rest, (key, From::from(Unit))));
        }
        let Token::Unquote(s) = tokens.next().unwrap() else {
            return fail(src);
        };
        if &*s != PAIR {
            return fail(src);
        }
        let Some(value) = ComposeParser::new(self.ctx).compose_tokens(tokens) else {
            return fail(src);
        };
        Ok((rest, (key, value)))
    }
}

fn symbol<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while1(|c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let fragment = alt((literal, symbol_escaped, symbol_space));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        string.push_str(fragment);
        string
    });
    let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    let f = map(delimited_symbol, |s| From::from(Symbol::from_string(s)));
    context("symbol", f)(src)
}

fn symbol_escaped<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = preceded(
        char1('\\'),
        alt((
            value("\\", char1('\\')),
            value(" ", char1('_')),
            value(concatcp!(SYMBOL_QUOTE), char1(SYMBOL_QUOTE)),
            value("", multispace1),
        )),
    );
    context("symbol_escaped", f)(src)
}

// ignore spaces following \n
fn symbol_space<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = alt((
        value("", preceded(char1('\n'), multispace0)),
        value("", char1('\r')),
        value("", take_while1(|c| c == '\t')),
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

fn text<'a, T, E>(src: &'a str) -> IResult<&'a str, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let literal = take_while1(|c| !matches!(c, '"' | '\\' | '\n'));
    let space = terminated(tag("\n"), multispace0);
    let fragment = alt((
        map(literal, StrFragment::Str),
        text_escaped,
        map(space, StrFragment::Str),
    ));
    let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
        fragment.push(&mut string);
        string
    });
    let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
    let f = map(delimited_text, |s| From::from(Text::from(s)));
    context("text", f)(src)
}

fn text_escaped<'a, E>(src: &'a str) -> IResult<&'a str, StrFragment<'a>, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let f = preceded(
        char1('\\'),
        alt((
            map(unicode, StrFragment::Char),
            value(StrFragment::Char('\n'), char1('n')),
            value(StrFragment::Char('\r'), char1('r')),
            value(StrFragment::Char('\t'), char1('t')),
            value(StrFragment::Char('\\'), char1('\\')),
            value(StrFragment::Char(' '), char1('_')),
            value(StrFragment::Char(TEXT_QUOTE), char1(TEXT_QUOTE)),
            value(StrFragment::Str(""), multispace1),
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
    let logical = alt((value(true, char1('+')), value(false, char1('|'))));
    let f = map(
        tuple((physical, space, logical)),
        |(physical, _, logical): (&str, _, _)| {
            if logical { physical } else { "" }
        },
    );
    context("raw_text_newline", f)(src)
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
