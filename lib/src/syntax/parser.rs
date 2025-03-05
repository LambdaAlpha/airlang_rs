use std::{
    hash::Hash,
    ops::Neg,
    str::FromStr,
};

use const_format::concatcp;
use num_bigint::BigInt;
use num_traits::Num;
use winnow::{
    ModalResult,
    Parser,
    Result,
    ascii::{
        digit1,
        line_ending,
        multispace0,
        space0,
        till_line_ending,
    },
    combinator::{
        alt,
        cut_err,
        delimited,
        empty,
        eof,
        fail,
        opt,
        peek,
        preceded,
        repeat,
        separated,
        terminated,
    },
    dispatch,
    error::{
        ContextError,
        ErrMode,
        StrContext,
    },
    stream::Stream,
    token::{
        any,
        take_while,
    },
};

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
    + From<Change<Self, Self>>
    + From<Call<Self, Self>>
    + From<Abstract<Self, Self>>
    + From<Ask<Self, Self>>
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

type E = ErrMode<ContextError>;

pub(crate) fn parse<T: ParseRepr>(mut src: &str) -> Result<T, crate::syntax::ParseError> {
    terminated(top::<T>, eof)
        .parse(&mut src)
        .map_err(|e| crate::syntax::ParseError { msg: e.to_string() })
}

fn top<T: ParseRepr>(src: &mut &str) -> ModalResult<T> {
    trim(ComposeParser::new(ParseCtx::default())).context(StrContext::Label("top")).parse_next(src)
}

fn trim<'a, O, F>(f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(take_while(0 .., is_empty), f, take_while(0 .., is_empty))
}

fn empty1<'a>(src: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_empty).parse_next(src)
}

fn is_empty(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn delimited_cut<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited(left, cut_err(f), cut_err(right))
}

fn delimited_trim<'a, T, F>(left: char, f: F, right: char) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_cut(left, trim(f), right)
}

#[derive(Copy, Clone)]
struct ScopeParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T: ParseRepr> Parser<&'a str, T, E> for ScopeParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<T, E> {
        delimited_trim(SCOPE_LEFT, ComposeParser::new(self.ctx), SCOPE_RIGHT)
            .context(StrContext::Label("scope"))
            .parse_next(input)
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

impl<'a> Parser<&'a str, ParseCtx<'a>, E> for CtxParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<ParseCtx<'a>, E> {
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
                _ => return fail.parse_next(input),
            }
        }
        if direction > 1 || arity > 1 {
            return fail.parse_next(input);
        }
        input.finish();
        Ok(ctx)
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

impl<'a, T: ParseRepr> Parser<&'a str, Token<T>, E> for TokenParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<Token<T>, E> {
        dispatch! {peek(any);
            // delimiters
            LIST_LEFT => ListParser::new(self.ctx).map(Token::Default),
            LIST_RIGHT => fail,
            MAP_LEFT => MapParser::new(self.ctx).map(Token::Default),
            MAP_RIGHT => fail,
            SCOPE_LEFT => ScopeParser::new(self.ctx).map(Token::Default),
            SCOPE_RIGHT => fail,
            SEPARATOR => fail,
            SPACE => fail,
            TEXT_QUOTE => text.map(Token::Default),
            SYMBOL_QUOTE => symbol.map(Token::Default),

            sym if is_symbol(sym) => ExtParser::new(self.ctx),
            _ => fail,
        }
        .context(StrContext::Label("token"))
        .parse_next(input)
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

impl<'a, T: ParseRepr> Parser<&'a str, Token<T>, E> for ExtParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<Token<T>, E> {
        const LEFT_DELIMITERS: [char; 5] =
            [SCOPE_LEFT, LIST_LEFT, MAP_LEFT, SYMBOL_QUOTE, TEXT_QUOTE];

        let mut s = take_while(1 .., is_trivial_symbol).parse_next(input)?;

        // the only special case
        let first = s.chars().next().unwrap();
        if first.is_ascii_digit() && !input.starts_with(LEFT_DELIMITERS) {
            let token = terminated(int_or_number, eof).parse_next(&mut s)?;
            return Ok(Token::Default(token));
        }

        alt((
            PrefixParser::new(s, self.ctx).map(Token::Default),
            empty.value(Symbol::from_str(s)).map(Token::Unquote),
        ))
        .parse_next(input)
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

impl<'a, T: ParseRepr> Parser<&'a str, T, E> for PrefixParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<T, E> {
        match self.prefix {
            UNIT => alt((
                raw_text,
                raw_symbol,
                ListParser::new_raw(self.ctx),
                empty.value(From::from(Unit)),
            ))
            .parse_next(input),
            TRUE => empty.value(From::from(Bit::true1())).parse_next(input),
            FALSE => empty.value(From::from(Bit::false1())).parse_next(input),
            INT => int(input),
            NUMBER => number(input),
            BYTE => byte(input),
            PAIR => {
                let ctx = self.ctx.escape().with_struct(Struct::Pair);
                ScopeParser::new(ctx).parse_next(input)
            }
            CHANGE => {
                let ctx = self.ctx.escape().with_struct(Struct::Change);
                ScopeParser::new(ctx).parse_next(input)
            }
            CALL => {
                let ctx = self.ctx.escape().with_struct(Struct::Call);
                ScopeParser::new(ctx).parse_next(input)
            }
            ABSTRACT => {
                let ctx = self.ctx.escape().with_struct(Struct::Abstract);
                ScopeParser::new(ctx).parse_next(input)
            }
            ASK => {
                let ctx = self.ctx.escape().with_struct(Struct::Ask);
                ScopeParser::new(ctx).parse_next(input)
            }
            TAG => ScopeParser::new(self.ctx.escape()).parse_next(input),
            s if s.starts_with(TAG_CHAR) => {
                ScopeParser::new(self.ctx.tag(&s[1 ..])).parse_next(input)
            }
            mut s if s.chars().all(CtxParser::is_ctx) => {
                let ctx = CtxParser::new(self.ctx)
                    .context(StrContext::Label("ctx"))
                    .parse_next(&mut s)?;
                ScopeParser::new(ctx).parse_next(input)
            }
            _ => fail.parse_next(input),
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

impl<'a, T: ParseRepr> Parser<&'a str, T, E> for ComposeParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<T, E> {
        let tokens = separated(1 .., TokenParser::new(self.ctx), empty1);
        tokens
            .verify_map(|tokens: Vec<_>| self.compose_tokens(tokens.into_iter()))
            .context(StrContext::Label("compose"))
            .parse_next(input)
    }
}

impl<'a> ComposeParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        ComposeParser { ctx }
    }

    fn compose_tokens<T, I>(&self, mut tokens: I) -> Option<T>
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

    fn compose_tag<T, I>(&self, tokens: I, tag: &str) -> Option<T>
    where
        T: ParseRepr,
        I: Iterator<Item = Token<T>>, {
        let list = tokens.map(Token::into_repr).collect::<List<_>>();
        let list = From::from(list);
        let tag = From::from(Symbol::from_str(tag));
        let repr = self.compose_two(tag, list);
        Some(repr)
    }

    fn compose_two<T: ParseRepr>(&self, left: T, right: T) -> T {
        match self.ctx.struct1 {
            Struct::Pair => From::from(Pair::new(left, right)),
            Struct::Change => From::from(Change::new(left, right)),
            Struct::Call => From::from(Call::new(left, right)),
            Struct::Abstract => From::from(Abstract::new(left, right)),
            Struct::Ask => From::from(Ask::new(left, right)),
        }
    }

    fn compose_many2<T, I>(&self, mut iter: I) -> Option<T>
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

    fn compose_many3<T, I>(&self, mut iter: I) -> Option<T>
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

    fn compose_infix<T: ParseRepr>(&self, left: T, middle: Token<T>, right: T) -> T {
        let middle = match middle {
            Token::Unquote(s) => match &*s {
                PAIR => return From::from(Pair::new(left, right)),
                CHANGE => return From::from(Change::new(left, right)),
                CALL => return From::from(Call::new(left, right)),
                ABSTRACT => return From::from(Abstract::new(left, right)),
                ASK => return From::from(Ask::new(left, right)),
                _ => From::from(s),
            },
            Token::Default(middle) => middle,
        };
        let pair = Pair::new(left, right);
        let pair = From::from(pair);
        self.compose_two(middle, pair)
    }
}

fn items<'a, O1, O2, S, F>(item: F, separator: S) -> impl Parser<&'a str, Vec<O2>, E>
where
    S: Parser<&'a str, O1, E>,
    F: Parser<&'a str, O2, E> + Clone, {
    let items = repeat(0 .., terminated(item.clone(), trim(separator)));
    (items, opt(item)).map(|(mut items, last): (Vec<O2>, _)| {
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

impl<'a, T: ParseRepr> Parser<&'a str, T, E> for ListParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<T, E> {
        if self.raw {
            let items = separated(0 .., TokenParser::new(self.ctx), empty1);
            let items = items.map(|tokens: Vec<_>| {
                let list: List<T> = tokens.into_iter().map(Token::into_repr).collect();
                T::from(list)
            });
            delimited_trim(LIST_LEFT, items, LIST_RIGHT)
                .context(StrContext::Label("raw_list"))
                .parse_next(input)
        } else {
            let items = items(ComposeParser::new(self.ctx), SEPARATOR);
            let items = items.map(|list| From::from(List::from(list)));
            delimited_trim(LIST_LEFT, items, LIST_RIGHT)
                .context(StrContext::Label("list"))
                .parse_next(input)
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

impl<'a, T: ParseRepr> Parser<&'a str, T, E> for MapParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<T, E> {
        let items = items(KeyValueParser::new(self.ctx), SEPARATOR);
        delimited_trim(MAP_LEFT, items, MAP_RIGHT)
            .map(|pairs| From::from(Map::from_iter(pairs)))
            .context(StrContext::Label("map"))
            .parse_next(input)
    }
}

impl<'a> MapParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        MapParser { ctx }
    }
}

#[derive(Copy, Clone)]
struct KeyValueParser<'a> {
    ctx: ParseCtx<'a>,
}

impl<'a, T: ParseRepr> Parser<&'a str, (T, T), E> for KeyValueParser<'a> {
    fn parse_next(&mut self, input: &mut &'a str) -> Result<(T, T), E> {
        let tokens: Vec<_> =
            separated(1 .., TokenParser::new(self.ctx), empty1).parse_next(input)?;
        let mut tokens = tokens.into_iter();
        let key = [tokens.next().unwrap()].into_iter();
        let key = ComposeParser::new(self.ctx).compose_tokens(key).unwrap();
        if tokens.len() == 0 {
            return Ok((key, From::from(Unit)));
        }
        let Token::Unquote(s) = tokens.next().unwrap() else {
            return fail.parse_next(input);
        };
        if &*s != PAIR {
            return fail.parse_next(input);
        }
        let Some(value) = ComposeParser::new(self.ctx).compose_tokens(tokens) else {
            return fail.parse_next(input);
        };
        Ok((key, value))
    }
}

impl<'a> KeyValueParser<'a> {
    fn new(ctx: ParseCtx<'a>) -> Self {
        Self { ctx }
    }
}

fn symbol<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let literal = take_while(1 .., |c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let fragment = alt((literal, symbol_escaped, symbol_space));
    let collect_fragments = repeat(0 .., fragment).fold(String::new, |mut string, fragment| {
        string.push_str(fragment);
        string
    });
    let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    delimited_symbol
        .map(|s| From::from(Symbol::from_string(s)))
        .context(StrContext::Label("symbol"))
        .parse_next(input)
}

fn symbol_escaped<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    preceded('\\', dispatch! {any;
        '\\' => empty.value("\\"),
        '_' => empty.value(" "),
        SYMBOL_QUOTE => empty.value(concatcp!(SYMBOL_QUOTE)),
        ' ' | '\t' | '\r' | '\n' => multispace0.value(""),
        _ => fail,
    })
    .context(StrContext::Label("symbol_escaped"))
    .parse_next(input)
}

// ignore spaces following \n
fn symbol_space<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    dispatch! {any;
        '\n' => multispace0.value(""),
        '\r' => empty.value(""),
        '\t' => take_while(0 .., |c| c == '\t').value(""),
        _ => fail,
    }
    .context(StrContext::Label("symbol_space"))
    .parse_next(input)
}

fn raw_symbol<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let literal = take_while(0 .., is_symbol);
    let fragment = separated(1 .., terminated(literal, raw_symbol_newline), ' ');
    let collect_fragments = fragment.map(|fragments: Vec<_>| fragments.join(""));
    let delimited_symbol = delimited_cut(SYMBOL_QUOTE, collect_fragments, SYMBOL_QUOTE);
    delimited_symbol
        .map(|s| From::from(Symbol::from_string(s)))
        .context(StrContext::Label("raw_symbol"))
        .parse_next(input)
}

fn raw_symbol_newline(input: &mut &str) -> ModalResult<()> {
    (line_ending, space0, '|')
        .void()
        .context(StrContext::Label("raw_symbol_newline"))
        .parse_next(input)
}

fn text<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let literal = take_while(1 .., |c| !matches!(c, '"' | '\\' | '\n'));
    let space = terminated("\n", multispace0);
    let fragment = alt((literal.map(StrFragment::Str), text_escaped, space.map(StrFragment::Str)));
    let collect_fragments = repeat(0 .., fragment).fold(String::new, |mut string, fragment| {
        fragment.push(&mut string);
        string
    });
    let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
    delimited_text
        .map(|s| From::from(Text::from(s)))
        .context(StrContext::Label("text"))
        .parse_next(input)
}

fn text_escaped<'a>(input: &mut &'a str) -> ModalResult<StrFragment<'a>> {
    preceded('\\', dispatch! {any;
        'u' => unicode.map(StrFragment::Char),
        'n' => empty.value(StrFragment::Char('\n')),
        'r' => empty.value(StrFragment::Char('\r')),
        't' => empty.value(StrFragment::Char('\t')),
        '\\' => empty.value(StrFragment::Char('\\')),
        '_' => empty.value(StrFragment::Char(' ')),
        TEXT_QUOTE => empty.value(StrFragment::Char(TEXT_QUOTE)),
        ' ' | '\t' | '\r' | '\n' => multispace0.value(StrFragment::Str("")),
        _ => fail,
    })
    .context(StrContext::Label("text_escaped"))
    .parse_next(input)
}

fn unicode(input: &mut &str) -> ModalResult<char> {
    let digit = take_while(1 .. 7, is_hexadecimal);
    let delimited_digit = delimited_trim(SCOPE_LEFT, digit, SCOPE_RIGHT);
    let code = delimited_digit.map(move |hex| u32::from_str_radix(hex, 16).unwrap());
    code.verify_map(std::char::from_u32).context(StrContext::Label("unicode")).parse_next(input)
}

fn raw_text<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let fragment = separated(1 .., (till_line_ending, raw_text_newline), ' ');
    let collect_fragments = fragment.map(|fragments: Vec<_>| {
        let mut s = String::new();
        for (literal, newline) in fragments {
            s.push_str(literal);
            s.push_str(newline);
        }
        s
    });
    let delimited_text = delimited_cut(TEXT_QUOTE, collect_fragments, TEXT_QUOTE);
    delimited_text
        .map(|s| From::from(Text::from(s)))
        .context(StrContext::Label("raw_text"))
        .parse_next(input)
}

fn raw_text_newline<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let newline = alt(('+'.value(true), '|'.value(false)));
    (line_ending, space0, newline)
        .map(|(ending, _, newline): (&str, _, _)| if newline { ending } else { "" })
        .context(StrContext::Label("raw_text_newline"))
        .parse_next(input)
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

fn int_or_number<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let norm = preceded("0", (sign, significand, opt(exponent)));
    let short = (empty.value(true), significand_radix(10, digit1), opt(exponent));
    alt((norm, short))
        .map(|(sign, significand, exponent)| build_int_or_number(sign, significand, exponent))
        .context(StrContext::Label("int_or_number"))
        .parse_next(input)
}

fn int<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let int = (sign, integral).map(|(sign, i)| build_int(sign, i));
    delimited_trim(SCOPE_LEFT, int, SCOPE_RIGHT).context(StrContext::Label("int")).parse_next(input)
}

fn number<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let number = (sign, significand, opt(exponent))
        .map(|(sign, significand, exponent)| build_number(sign, significand, exponent));
    delimited_trim(SCOPE_LEFT, number, SCOPE_RIGHT)
        .context(StrContext::Label("number"))
        .parse_next(input)
}

fn trim_num0<'a, F>(f: F) -> impl Parser<&'a str, String, E>
where F: Parser<&'a str, &'a str, E> {
    separated(0 .., f, '_').map(|s: Vec<&str>| s.join(""))
}

fn trim_num1<'a, F>(f: F) -> impl Parser<&'a str, String, E>
where F: Parser<&'a str, &'a str, E> {
    separated(1 .., f, '_').map(|s: Vec<&str>| s.join(""))
}

fn sign(input: &mut &str) -> ModalResult<bool> {
    alt(('+'.value(true), '-'.value(false), empty.value(true)))
        .context(StrContext::Label("sign"))
        .parse_next(input)
}

fn integral(input: &mut &str) -> ModalResult<BigInt> {
    let dec_no_tag = int_radix(10, digit1);
    let hex = preceded("X", cut_err(int_radix(16, hexadecimal1)));
    let bin = preceded("B", cut_err(int_radix(2, binary1)));
    let dec = preceded("D", cut_err(int_radix(10, digit1)));

    alt((dec_no_tag, hex, bin, dec)).context(StrContext::Label("integral")).parse_next(input)
}

fn int_radix<'a, F>(radix: u8, f: F) -> impl Parser<&'a str, BigInt, E>
where F: Parser<&'a str, &'a str, E> {
    trim_num1(f).map(move |int| BigInt::from_str_radix(&int, radix as u32).unwrap())
}

struct Significand {
    int: BigInt,
    radix: u8,
    shift: Option<usize>,
}

fn significand(input: &mut &str) -> ModalResult<Significand> {
    let dec_no_tag = significand_radix(10, digit1);
    let hex = preceded("X", cut_err(significand_radix(16, hexadecimal1)));
    let bin = preceded("B", cut_err(significand_radix(2, binary1)));
    let dec = preceded("D", cut_err(significand_radix(10, digit1)));

    alt((dec_no_tag, hex, bin, dec)).context(StrContext::Label("significand")).parse_next(input)
}

fn significand_radix<'a, F>(radix: u8, f: F) -> impl Parser<&'a str, Significand, E>
where F: Parser<&'a str, &'a str, E> + Clone {
    let int = trim_num1(f.clone());
    let fraction = opt(preceded('.', cut_err(trim_num0(f))));
    (int, fraction).map(move |(int, fraction)| build_significand(radix, int, fraction))
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

fn exponent(input: &mut &str) -> ModalResult<BigInt> {
    let exp = (sign, trim_num1(digit1)).map(|(sign, exp)| build_exponent(sign, exp));
    preceded("E", cut_err(exp)).context(StrContext::Label("exponent")).parse_next(input)
}

fn build_exponent(sign: bool, exp: String) -> BigInt {
    let i = BigInt::from_str(&exp).unwrap();
    if sign { i } else { i.neg() }
}

fn build_int<T: ParseRepr>(sign: bool, i: BigInt) -> T {
    let i = Int::new(if sign { i } else { i.neg() });
    From::from(i)
}

fn build_number<T: ParseRepr>(sign: bool, significand: Significand, exp: Option<BigInt>) -> T {
    let int = significand.int;
    let int = if sign { int } else { int.neg() };
    let shift = significand.shift.unwrap_or(0);
    let exp = exp.unwrap_or_default() - shift;
    let n = Number::new(int, significand.radix, exp);
    From::from(n)
}

fn build_int_or_number<T: ParseRepr>(
    sign: bool, significand: Significand, exp: Option<BigInt>,
) -> T {
    if significand.shift.is_some() || exp.is_some() {
        build_number(sign, significand, exp)
    } else {
        build_int(sign, significand.int)
    }
}

fn byte<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let hex = preceded("X", cut_err(hexadecimal_byte));
    let bin = preceded("B", cut_err(binary_byte));
    let byte = alt((hex, bin, hexadecimal_byte));
    delimited_trim(SCOPE_LEFT, byte, SCOPE_RIGHT)
        .context(StrContext::Label("byte"))
        .parse_next(input)
}

fn hexadecimal_byte<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let digits = hexadecimal1.verify(|s: &str| s.len() % 2 == 0);
    trim_num0(digits)
        .map(|s| {
            let byte = utils::conversion::hex_str_to_vec_u8(&s).unwrap();
            From::from(Byte::from(byte))
        })
        .context(StrContext::Label("hexadecimal_byte"))
        .parse_next(input)
}

fn binary_byte<T: ParseRepr>(input: &mut &str) -> ModalResult<T> {
    let digits = binary1.verify(|s: &str| s.len() % 8 == 0);
    trim_num0(digits)
        .map(|s| {
            let byte = utils::conversion::bin_str_to_vec_u8(&s).unwrap();
            From::from(Byte::from(byte))
        })
        .context(StrContext::Label("binary_byte"))
        .parse_next(input)
}

fn hexadecimal1<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_hexadecimal).parse_next(input)
}

fn is_hexadecimal(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

fn binary1<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1 .., is_binary).parse_next(input)
}

fn is_binary(c: char) -> bool {
    matches!(c, '0' ..= '1')
}
