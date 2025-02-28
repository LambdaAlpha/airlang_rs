use std::{
    hash::Hash,
    marker::PhantomData,
    ops::Neg,
    str::FromStr,
};

use const_format::concatcp;
use nom::{
    Emit,
    Finish,
    IResult,
    Mode,
    OutputM,
    OutputMode,
    PResult,
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
        context,
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
    },
};
use nom_language::error::{
    VerboseError,
    convert_error,
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
    + Clone {
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
    Change,
    Call,
    Abstract,
    Ask,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

pub(crate) fn parse<T: ParseRepr>(src: &str) -> Result<T, crate::syntax::ParseError> {
    let mut top = TopParser::<T, VerboseError<&str>>::new();
    let ret = top.parse(src).finish();
    match ret {
        Ok(r) => Ok(r.1),
        Err(e) => {
            let msg = convert_error(src, e);
            Err(crate::syntax::ParseError { msg })
        }
    }
}

type EmitM<O, I> = OutputM<Emit, O, I>;

struct TopParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for TopParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = all_consuming(trim(ComposeParser::new(ParseCtx::default())));
        context("top", f).process::<OM>(input)
    }
}

impl<T, E> TopParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for TopParser<T, E> {}

impl<T, E> Clone for TopParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

fn trim<'a, O, E, F>(f: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = O, Error = E>, {
    delimited(empty0, f, empty0)
}

fn empty0<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where E: ParseError<&'a str> {
    take_while(is_empty)(src)
}

fn empty1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where E: ParseError<&'a str> {
    take_while1(is_empty)(src)
}

fn is_empty(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn delimited_cut<'a, T, E, F>(
    left: char, f: F, right: char,
) -> impl Parser<&'a str, Output = T, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = T, Error = E>, {
    delimited(char1(left), cut(f), cut(char1(right)))
}

fn delimited_trim<'a, T, E, F>(
    left: char, f: F, right: char,
) -> impl Parser<&'a str, Output = T, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = T, Error = E>, {
    delimited_cut(left, trim(f), right)
}

struct ScopeParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ScopeParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = delimited_trim(SCOPE_LEFT, ComposeParser::new(self.ctx), SCOPE_RIGHT);
        context("scope", f).process::<OM>(input)
    }
}

impl<'a, T, E> ScopeParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ScopeParser<'_, T, E> {}

impl<T, E> Clone for ScopeParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct CtxParser<'a, E> {
    ctx: ParseCtx<'a>,
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for CtxParser<'a, E>
where E: ParseError<&'a str>
{
    type Output = ParseCtx<'a>;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
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
                _ => return fail().process::<OM>(input),
            }
        }
        if direction > 1 || arity > 1 {
            return fail().process::<OM>(input);
        }
        Ok(("", OM::Output::bind(|| ctx)))
    }
}

impl<'a, E> CtxParser<'a, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx, e: PhantomData }
    }

    fn is_ctx(c: char) -> bool {
        matches!(c, LEFT | RIGHT | ARITY_2 | ARITY_3)
    }
}

impl<E> Copy for CtxParser<'_, E> {}

impl<E> Clone for CtxParser<'_, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TokenParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for TokenParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = Token<T>;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let mut first = peek(anychar);
        let (src, first) = first.process::<EmitM<OM::Error, OM::Incomplete>>(input)?;
        let dispatcher = DispatchParser::new(first, self.ctx);
        context("token", dispatcher).process::<OM>(src)
    }
}

impl<'a, T, E> TokenParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for TokenParser<'_, T, E> {}

impl<T, E> Clone for TokenParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct DispatchParser<'a, T, E> {
    first: char,
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for DispatchParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = Token<T>;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, s: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        match self.first {
            // delimiters
            LIST_LEFT => map(ListParser::new(self.ctx), Token::Default).process::<OM>(s),
            LIST_RIGHT => fail().process::<OM>(s),
            MAP_LEFT => map(MapParser::new(self.ctx), Token::Default).process::<OM>(s),
            MAP_RIGHT => fail().process::<OM>(s),
            SCOPE_LEFT => map(ScopeParser::new(self.ctx), Token::Default).process::<OM>(s),
            SCOPE_RIGHT => fail().process::<OM>(s),
            SEPARATOR => fail().process::<OM>(s),
            SPACE => fail().process::<OM>(s),
            TEXT_QUOTE => map(TextParser::new(), Token::Default).process::<OM>(s),
            SYMBOL_QUOTE => map(SymbolParser::new(), Token::Default).process::<OM>(s),

            sym if is_symbol(sym) => ExtParser::new(self.ctx).process::<OM>(s),
            _ => fail().process::<OM>(s),
        }
    }
}

impl<'a, T, E> DispatchParser<'a, T, E> {
    fn new(first: char, ctx: ParseCtx<'a>) -> Self {
        Self { first, ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for DispatchParser<'_, T, E> {}

impl<T, E> Clone for DispatchParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
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

struct ExtParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ExtParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = Token<T>;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        const LEFT_DELIMITERS: [char; 5] =
            [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, SYMBOL_QUOTE, TEXT_QUOTE];

        let mut trivial_symbols = take_while(is_trivial_symbol);
        let (rest, s) = trivial_symbols.process::<EmitM<OM::Error, OM::Incomplete>>(input)?;

        // the only special case
        let first = s.chars().next().unwrap();
        if first.is_ascii_digit() && !rest.starts_with(LEFT_DELIMITERS) {
            let mut int_num = all_consuming(IntNumParser::new());
            let (_, token) = int_num.process::<EmitM<OM::Error, OM::Incomplete>>(s)?;
            return Ok((rest, OM::Output::bind(|| Token::Default(token))));
        }

        let mut f = alt((
            map(PrefixParser::new(s, self.ctx), Token::Default),
            map(success(Symbol::from_str(s)), Token::Unquote),
        ));
        f.process::<OM>(rest)
    }
}

impl<'a, T, E> ExtParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ExtParser<'_, T, E> {}

impl<T, E> Clone for ExtParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct PrefixParser<'a, T, E> {
    prefix: &'a str,
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for PrefixParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        match self.prefix {
            UNIT => alt((
                TextRawParser::new(),
                SymbolRawParser::new(),
                ListParser::new_raw(self.ctx),
                success(T::from(Unit)),
            ))
            .process::<OM>(input),
            TRUE => success(T::from(Bit::true1())).process::<OM>(input),
            FALSE => success(T::from(Bit::false1())).process::<OM>(input),
            INT => IntParser::new().process::<OM>(input),
            NUMBER => NumParser::new().process::<OM>(input),
            BYTE => ByteParser::new().process::<OM>(input),
            PAIR => {
                let ctx = self.ctx.escape().with_struct(Struct::Pair);
                ScopeParser::new(ctx).process::<OM>(input)
            }
            CHANGE => {
                let ctx = self.ctx.escape().with_struct(Struct::Change);
                ScopeParser::new(ctx).process::<OM>(input)
            }
            CALL => {
                let ctx = self.ctx.escape().with_struct(Struct::Call);
                ScopeParser::new(ctx).process::<OM>(input)
            }
            ABSTRACT => {
                let ctx = self.ctx.escape().with_struct(Struct::Abstract);
                ScopeParser::new(ctx).process::<OM>(input)
            }
            ASK => {
                let ctx = self.ctx.escape().with_struct(Struct::Ask);
                ScopeParser::new(ctx).process::<OM>(input)
            }
            TAG => ScopeParser::new(self.ctx.escape()).process::<OM>(input),
            s if s.starts_with(TAG_CHAR) => {
                ScopeParser::new(self.ctx.tag(&s[1 ..])).process::<OM>(input)
            }
            s if s.chars().all(CtxParser::<E>::is_ctx) => {
                let mut ctx_parser = context("ctx", CtxParser::new(self.ctx));
                let (_, ctx) = ctx_parser.process::<EmitM<OM::Error, OM::Incomplete>>(s)?;
                ScopeParser::new(ctx).process::<OM>(input)
            }
            _ => fail().process::<OM>(input),
        }
    }
}

impl<'a, T, E> PrefixParser<'a, T, E> {
    fn new(prefix: &'a str, ctx: ParseCtx<'a>) -> Self {
        Self { prefix, ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for PrefixParser<'_, T, E> {}

impl<T, E> Clone for PrefixParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
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

struct ComposeParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ComposeParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let tokens = separated_list1(empty1, TokenParser::new(self.ctx));
        let f = map_opt(tokens, |tokens| self.compose_tokens(tokens.into_iter()));
        context("compose", f).process::<OM>(input)
    }
}

impl<'a, T, E> ComposeParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        ComposeParser { ctx, o: PhantomData, e: PhantomData }
    }

    fn compose_tokens<I>(&self, mut tokens: I) -> Option<T>
    where
        T: ParseRepr,
        I: ExactSizeIterator<Item = Token<T>> + DoubleEndedIterator<Item = Token<T>>, {
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

    fn compose_tag<I>(&self, tokens: I, tag: &str) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>, {
        let list = tokens.map(Token::into_repr).collect::<List<_>>();
        let list = T::from(list);
        let tag = T::from(Symbol::from_str(tag));
        let repr = self.compose_two(tag, list);
        Some(repr)
    }

    fn compose_two(&self, left: T, right: T) -> T
    where T: ParseRepr {
        match self.ctx.struct1 {
            Struct::Pair => T::from(Pair::new(left, right)),
            Struct::Change => T::from(Change::new(left, right)),
            Struct::Call => T::from(Call::new(left, right)),
            Struct::Abstract => T::from(Abstract::new(left, right)),
            Struct::Ask => T::from(Ask::new(left, right)),
        }
    }

    fn compose_many2<I>(&self, mut iter: I) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>, {
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

    fn compose_many3<I>(&self, mut iter: I) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>, {
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

    fn compose_infix(&self, left: T, middle: Token<T>, right: T) -> T
    where T: ParseRepr {
        let middle = match middle {
            Token::Unquote(s) => match &*s {
                PAIR => return T::from(Pair::new(left, right)),
                CHANGE => return T::from(Change::new(left, right)),
                CALL => return T::from(Call::new(left, right)),
                ABSTRACT => return T::from(Abstract::new(left, right)),
                ASK => return T::from(Ask::new(left, right)),
                _ => T::from(s),
            },
            Token::Default(middle) => middle,
        };
        let pair = Pair::new(left, right);
        let pair = T::from(pair);
        self.compose_two(middle, pair)
    }
}

impl<T, E> Copy for ComposeParser<'_, T, E> {}

impl<T, E> Clone for ComposeParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

fn items<'a, O1, O2, E, S, F>(
    item: F, separator: S,
) -> impl Parser<&'a str, Output = Vec<O2>, Error = E>
where
    E: ParseError<&'a str>,
    S: Parser<&'a str, Output = O1, Error = E>,
    F: Parser<&'a str, Output = O2, Error = E> + Clone, {
    let items = many0(terminated(item.clone(), trim(separator)));
    map((items, opt(item)), |(mut items, last)| {
        if let Some(last) = last {
            items.push(last);
        }
        items
    })
}

struct ListParser<'a, T, E> {
    raw: bool,
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ListParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        if self.raw {
            let items = separated_list0(empty1, TokenParser::new(self.ctx));
            let items = map(items, |tokens| {
                let list: List<T> = tokens.into_iter().map(Token::into_repr).collect();
                T::from(list)
            });
            let f = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
            context("raw_list", f).process::<OM>(input)
        } else {
            let items = items(ComposeParser::new(self.ctx), char1(SEPARATOR));
            let items = map(items, |list| T::from(List::from(list)));
            let f = delimited_trim(LIST_LEFT, items, LIST_RIGHT);
            context("list", f).process::<OM>(input)
        }
    }
}

impl<'a, T, E> ListParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        ListParser { raw: false, ctx, o: PhantomData, e: PhantomData }
    }

    fn new_raw(ctx: ParseCtx<'a>) -> Self {
        Self { raw: true, ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ListParser<'_, T, E> {}

impl<T, E> Clone for ListParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct MapParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for MapParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;

    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let items = items(KeyValueParser::new(self.ctx), char1(SEPARATOR));
        let delimited_items = delimited_trim(MAP_LEFT, items, MAP_RIGHT);
        let f = map(delimited_items, |pairs| T::from(Map::from_iter(pairs)));
        context("map", f).process::<OM>(input)
    }
}

impl<'a, T, E> MapParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        MapParser { ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for MapParser<'_, T, E> {}

impl<T, E> Clone for MapParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct KeyValueParser<'a, T, E> {
    ctx: ParseCtx<'a>,
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for KeyValueParser<'a, T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = (T, T);
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let mut tokens = separated_list1(empty1, TokenParser::new(self.ctx));
        let (rest, tokens) = tokens.process::<EmitM<OM::Error, OM::Incomplete>>(input)?;
        let mut tokens = tokens.into_iter();
        let key = [tokens.next().unwrap()].into_iter();
        let key = ComposeParser::<T, E>::new(self.ctx).compose_tokens(key).unwrap();
        if tokens.len() == 0 {
            return Ok((rest, OM::Output::bind(|| (key, T::from(Unit)))));
        }
        let Token::Unquote(s) = tokens.next().unwrap() else {
            return fail().process::<OM>(input);
        };
        if &*s != PAIR {
            return fail().process::<OM>(input);
        }
        let Some(value) = ComposeParser::<T, E>::new(self.ctx).compose_tokens(tokens) else {
            return fail().process::<OM>(input);
        };
        Ok((rest, OM::Output::bind(|| (key, value))))
    }
}

impl<'a, T, E> KeyValueParser<'a, T, E> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx, o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for KeyValueParser<'_, T, E> {}

impl<T, E> Clone for KeyValueParser<'_, T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct SymbolParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for SymbolParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let literal = take_while1(|c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
        let fragment = alt((literal, SymbolEscParser::new(), SymbolSpaceParser::new()));
        let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
            string.push_str(fragment);
            string
        });
        let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
        let f = map(delimited_symbol, |s| T::from(Symbol::from_string(s)));
        context("symbol", f).process::<OM>(input)
    }
}

impl<T, E> SymbolParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for SymbolParser<T, E> {}

impl<T, E> Clone for SymbolParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct SymbolEscParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for SymbolEscParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = &'a str;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = preceded(
            char1('\\'),
            alt((
                value("\\", char1('\\')),
                value(" ", char1('_')),
                value(concatcp!(SYMBOL_QUOTE), char1(SYMBOL_QUOTE)),
                value("", multispace1),
            )),
        );
        context("symbol_escaped", f).process::<OM>(input)
    }
}

impl<E> SymbolEscParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for SymbolEscParser<E> {}

impl<E> Clone for SymbolEscParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

// ignore spaces following \n
struct SymbolSpaceParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for SymbolSpaceParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = &'a str;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = alt((
            value("", preceded(char1('\n'), multispace0)),
            value("", char1('\r')),
            value("", take_while1(|c| c == '\t')),
        ));
        context("symbol_space", f).process::<OM>(input)
    }
}

impl<E> SymbolSpaceParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for SymbolSpaceParser<E> {}

impl<E> Clone for SymbolSpaceParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct SymbolRawParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for SymbolRawParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let literal = take_while(is_symbol);
        let fragment =
            separated_list1(char1(' '), terminated(literal, SymbolRawNewlineParser::new()));
        let collect_fragments = map(fragment, |fragments| fragments.join(""));
        let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
        let f = map(delimited_symbol, |s| T::from(Symbol::from_string(s)));
        context("raw_symbol", f).process::<OM>(input)
    }
}

impl<T, E> SymbolRawParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for SymbolRawParser<T, E> {}

impl<T, E> Clone for SymbolRawParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct SymbolRawNewlineParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for SymbolRawNewlineParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = ();
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let newline = (opt(char1('\r')), char1('\n'));
        let space = take_while(|c| matches!(c, ' ' | '\t'));
        let f = value((), (newline, space, char1('|')));
        context("raw_symbol_newline", f).process::<OM>(input)
    }
}

impl<E> SymbolRawNewlineParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for SymbolRawNewlineParser<E> {}

impl<E> Clone for SymbolRawNewlineParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TextParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for TextParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let literal = take_while1(|c| !matches!(c, '"' | '\\' | '\n'));
        let space = terminated(tag("\n"), multispace0);
        let fragment = alt((
            map(literal, StrFragment::Str),
            TextEscParser::new(),
            map(space, StrFragment::Str),
        ));
        let collect_fragments = fold_many0(fragment, String::new, |mut string, fragment| {
            fragment.push(&mut string);
            string
        });
        let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
        let f = map(delimited_text, |s| T::from(Text::from(s)));
        context("text", f).process::<OM>(input)
    }
}

impl<T, E> TextParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for TextParser<T, E> {}

impl<T, E> Clone for TextParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TextEscParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for TextEscParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = StrFragment<'a>;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = preceded(
            char1('\\'),
            alt((
                map(TextUnicodeParser::new(), StrFragment::Char),
                value(StrFragment::Char('\n'), char1('n')),
                value(StrFragment::Char('\r'), char1('r')),
                value(StrFragment::Char('\t'), char1('t')),
                value(StrFragment::Char('\\'), char1('\\')),
                value(StrFragment::Char(' '), char1('_')),
                value(StrFragment::Char(TEXT_QUOTE), char1(TEXT_QUOTE)),
                value(StrFragment::Str(""), multispace1),
            )),
        );
        context("text_escaped", f).process::<OM>(input)
    }
}

impl<E> TextEscParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for TextEscParser<E> {}

impl<E> Clone for TextEscParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TextUnicodeParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for TextUnicodeParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = char;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let digit = take_while_m_n(1, 6, is_hexadecimal);
        let delimited_digit = preceded(char1('u'), delimited_trim(SCOPE_LEFT, digit, SCOPE_RIGHT));
        let code = map(delimited_digit, move |hex| u32::from_str_radix(hex, 16).unwrap());
        let f = map_opt(code, std::char::from_u32);
        context("unicode", f).process::<OM>(input)
    }
}

impl<E> TextUnicodeParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for TextUnicodeParser<E> {}

impl<E> Clone for TextUnicodeParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TextRawParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for TextRawParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let literal = take_while(|c| !matches!(c, '\r' | '\n'));
        let fragment = separated_list1(char1(' '), (literal, TextRawNewlineParser::new()));
        let collect_fragments = map(fragment, |fragments| {
            let mut s = String::new();
            for (literal, newline) in fragments {
                s.push_str(literal);
                s.push_str(newline);
            }
            s
        });
        let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
        let f = map(delimited_text, |s| T::from(Text::from(s)));
        context("raw_text", f).process::<OM>(input)
    }
}

impl<T, E> TextRawParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for TextRawParser<T, E> {}

impl<T, E> Clone for TextRawParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct TextRawNewlineParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for TextRawNewlineParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = &'a str;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let physical = recognize((opt(char1('\r')), char1('\n')));
        let space = take_while(|c| matches!(c, ' ' | '\t'));
        let logical = alt((value(true, char1('+')), value(false, char1('|'))));
        let f = map(
            (physical, space, logical),
            |(physical, _, logical): (&str, _, _)| {
                if logical { physical } else { "" }
            },
        );
        context("raw_text_newline", f).process::<OM>(input)
    }
}

impl<E> TextRawNewlineParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for TextRawNewlineParser<E> {}

impl<E> Clone for TextRawNewlineParser<E> {
    fn clone(&self) -> Self {
        *self
    }
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

struct IntNumParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for IntNumParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let norm = preceded(
            tag("0"),
            (SignParser::new(), SignificandParser::new(), ExponentParser::new()),
        );
        let short = (success(true), significand_radix(10, digit1), ExponentParser::new());
        let f = map(alt((norm, short)), |(sign, significand, exponent)| {
            build_int_or_number(sign, significand, exponent)
        });
        context("int_or_number", f).process::<OM>(input)
    }
}

impl<T, E> IntNumParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for IntNumParser<T, E> {}

impl<T, E> Clone for IntNumParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct IntParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for IntParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let int = map((SignParser::new(), IntegralParser::new()), |(sign, i)| build_int(sign, i));
        let f = delimited_trim(SCOPE_LEFT, int, SCOPE_RIGHT);
        context("int", f).process::<OM>(input)
    }
}

impl<T, E> IntParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for IntParser<T, E> {}

impl<T, E> Clone for IntParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct NumParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for NumParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let number = map(
            (SignParser::new(), SignificandParser::new(), ExponentParser::new()),
            |(sign, significand, exponent)| build_number(sign, significand, exponent),
        );
        let f = delimited_trim(SCOPE_LEFT, number, SCOPE_RIGHT);
        context("number", f).process::<OM>(input)
    }
}

impl<T, E> NumParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for NumParser<T, E> {}

impl<T, E> Clone for NumParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

fn trim_num0<'a, E, F>(f: F) -> impl Parser<&'a str, Output = String, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = &'a str, Error = E>, {
    map(separated_list0(char1('_'), f), |s| s.join(""))
}

fn trim_num1<'a, E, F>(f: F) -> impl Parser<&'a str, Output = String, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = &'a str, Error = E>, {
    map(separated_list1(char1('_'), f), |s| s.join(""))
}

struct SignParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for SignParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = bool;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let f = alt((value(true, char1('+')), value(false, char1('-')), success(true)));
        context("sign", f).process::<OM>(input)
    }
}

impl<E> SignParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for SignParser<E> {}

impl<E> Clone for SignParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct IntegralParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for IntegralParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = BigInt;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let dec_no_tag = int_radix(10, digit1);
        let hex = preceded(tag("X"), cut(int_radix(16, hexadecimal1)));
        let bin = preceded(tag("B"), cut(int_radix(2, binary1)));
        let dec = preceded(tag("D"), cut(int_radix(10, digit1)));

        let f = alt((dec_no_tag, hex, bin, dec));
        context("integral", f).process::<OM>(input)
    }
}

fn int_radix<'a, E, F>(radix: u8, f: F) -> impl Parser<&'a str, Output = BigInt, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = &'a str, Error = E>, {
    map(trim_num1(f), move |int| BigInt::from_str_radix(&int, radix as u32).unwrap())
}

impl<E> IntegralParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for IntegralParser<E> {}

impl<E> Clone for IntegralParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct SignificandParser<E> {
    e: PhantomData<E>,
}

struct Significand {
    int: BigInt,
    radix: u8,
    shift: Option<usize>,
}

impl<'a, E> Parser<&'a str> for SignificandParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = Significand;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let dec_no_tag = significand_radix(10, digit1);
        let hex = preceded(tag("X"), cut(significand_radix(16, hexadecimal1)));
        let bin = preceded(tag("B"), cut(significand_radix(2, binary1)));
        let dec = preceded(tag("D"), cut(significand_radix(10, digit1)));

        let f = alt((dec_no_tag, hex, bin, dec));
        context("significand", f).process::<OM>(input)
    }
}

fn significand_radix<'a, E, F>(
    radix: u8, f: F,
) -> impl Parser<&'a str, Output = Significand, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = &'a str, Error = E> + Clone, {
    let int = trim_num1(f.clone());
    let fraction = opt(preceded(char1('.'), cut(trim_num0(f))));
    map((int, fraction), move |(int, fraction)| build_significand(radix, int, fraction))
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

impl<E> SignificandParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for SignificandParser<E> {}

impl<E> Clone for SignificandParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct ExponentParser<E> {
    e: PhantomData<E>,
}

impl<'a, E> Parser<&'a str> for ExponentParser<E>
where E: ParseError<&'a str> + ContextError<&'a str>
{
    type Output = Option<BigInt>;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let fragment = (SignParser::new(), trim_num1(digit1));
        let exp = map(fragment, |(sign, exp)| build_exponent(sign, exp));
        let f = opt(preceded(tag("E"), cut(exp)));
        context("exponent", f).process::<OM>(input)
    }
}

fn build_exponent(sign: bool, exp: String) -> BigInt {
    let i = BigInt::from_str(&exp).unwrap();
    if sign { i } else { i.neg() }
}

impl<E> ExponentParser<E> {
    fn new() -> Self {
        Self { e: PhantomData }
    }
}

impl<E> Copy for ExponentParser<E> {}

impl<E> Clone for ExponentParser<E> {
    fn clone(&self) -> Self {
        *self
    }
}

fn build_int<T>(sign: bool, i: BigInt) -> T
where T: ParseRepr {
    let i = Int::new(if sign { i } else { i.neg() });
    T::from(i)
}

fn build_number<T>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T
where T: ParseRepr {
    let int = significand.int;
    let int = if sign { int } else { int.neg() };
    let shift = significand.shift.unwrap_or(0);
    let exp = exp.unwrap_or_default() - shift;
    let n = Number::new(int, significand.radix, exp);
    T::from(n)
}

fn build_int_or_number<T>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T
where T: ParseRepr {
    if significand.shift.is_some() || exp.is_some() {
        build_number(sign, significand, exp)
    } else {
        build_int(sign, significand.int)
    }
}

struct ByteParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ByteParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let hex = preceded(tag("X"), cut(ByteHexParser::new()));
        let bin = preceded(tag("B"), cut(ByteBinParser::new()));
        let byte = alt((hex, bin, ByteHexParser::new()));
        let f = delimited_trim(SCOPE_LEFT, byte, SCOPE_RIGHT);
        context("byte", f).process::<OM>(input)
    }
}

impl<T, E> ByteParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ByteParser<T, E> {}

impl<T, E> Clone for ByteParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct ByteHexParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ByteHexParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let digits = verify(hexadecimal1, |s: &str| s.len() % 2 == 0);
        let digits = trim_num0(digits);
        let f = map(digits, |s| {
            let byte = utils::conversion::hex_str_to_vec_u8(&s).unwrap();
            T::from(Byte::from(byte))
        });
        context("hexadecimal_byte", f).process::<OM>(input)
    }
}

impl<T, E> ByteHexParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ByteHexParser<T, E> {}

impl<T, E> Clone for ByteHexParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

struct ByteBinParser<T, E> {
    o: PhantomData<T>,
    e: PhantomData<E>,
}

impl<'a, T, E> Parser<&'a str> for ByteBinParser<T, E>
where
    T: ParseRepr,
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    type Output = T;
    type Error = E;
    fn process<OM: OutputMode>(
        &mut self, input: &'a str,
    ) -> PResult<OM, &'a str, Self::Output, Self::Error> {
        let digits = verify(binary1, |s: &str| s.len() % 8 == 0);
        let digits = trim_num0(digits);
        let f = map(digits, |s| {
            let byte = utils::conversion::bin_str_to_vec_u8(&s).unwrap();
            T::from(Byte::from(byte))
        });
        context("binary_byte", f).process::<OM>(input)
    }
}

impl<T, E> ByteBinParser<T, E> {
    fn new() -> Self {
        Self { o: PhantomData, e: PhantomData }
    }
}

impl<T, E> Copy for ByteBinParser<T, E> {}

impl<T, E> Clone for ByteBinParser<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

fn hexadecimal1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where E: ParseError<&'a str> {
    take_while1(is_hexadecimal)(src)
}

fn is_hexadecimal(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

fn binary1<'a, E>(src: &'a str) -> IResult<&'a str, &'a str, E>
where E: ParseError<&'a str> {
    take_while1(is_binary)(src)
}

fn is_binary(c: char) -> bool {
    matches!(c, '0' ..= '1')
}
