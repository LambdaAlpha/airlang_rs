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
        not,
        opt,
        peek,
        preceded,
        repeat,
        separated,
        terminated,
    },
    error::{
        ContextError,
        ErrMode,
        StrContext,
        StrContextValue,
    },
    stream::{
        Checkpoint,
        Range,
        Stream,
    },
    token::{
        any,
        one_of,
        take_while,
    },
};

use crate::{
    abstract1::Abstract,
    bit::Bit,
    byte::Byte,
    call::Call,
    change::Change,
    int::Int,
    list::List,
    map::Map,
    number::Number,
    optimize::Optimize,
    pair::Pair,
    solve::Solve,
    symbol::Symbol,
    syntax::{
        ABSTRACT,
        ARITY_2,
        ARITY_3,
        BYTE,
        CALL,
        CHANGE,
        FALSE,
        INLINE_COMMENT,
        INT,
        LEFT,
        LIST_LEFT,
        LIST_RIGHT,
        MAP_LEFT,
        MAP_RIGHT,
        MULTILINE_COMMENT,
        NUMBER,
        OPTIMIZE,
        PAIR,
        RIGHT,
        SCOPE_LEFT,
        SCOPE_RIGHT,
        SEPARATOR,
        SOLVE,
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
    + From<Optimize<Self, Self>>
    + From<Solve<Self, Self>>
    + From<Abstract<Self>>
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

    fn esc_struct(mut self, struct1: Struct) -> Self {
        self.is_tag = false;
        self.struct1 = struct1;
        self
    }

    fn esc_direction(mut self, direction: Direction) -> Self {
        self.is_tag = false;
        self.direction = direction;
        self
    }

    fn esc_arity(mut self, arity: Arity) -> Self {
        self.is_tag = false;
        self.arity = arity;
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
    Optimize,
    Solve,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

type E = ErrMode<ContextError>;

pub(crate) fn parse<T: ParseRepr>(src: &str) -> Result<T, crate::syntax::ParseError> {
    terminated(top::<T>, eof.context(expect_desc("end")))
        .parse(src)
        .map_err(|e| crate::syntax::ParseError { msg: e.to_string() })
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
    trim(compose(ParseCtx::default())).parse_next(src)
}

fn trim<'a, O, F>(f: F) -> impl Parser<&'a str, O, E>
where F: Parser<&'a str, O, E> {
    delimited(space(0 ..), f, space(0 ..))
}

fn space<'a>(occurrences: impl Into<Range>) -> impl Parser<&'a str, (), E> {
    let spaces = take_while(1 .., is_spaces).context(label("spaces")).void();
    let inline_comment = preceded(INLINE_COMMENT, text).context(label("inline comment")).void();
    let multiline_comment =
        preceded(MULTILINE_COMMENT, raw_text).context(label("multiline comment")).void();
    repeat(occurrences, alt((spaces, inline_comment, multiline_comment)))
}

fn is_spaces(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
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

fn scoped<'a, T, F>(f: F) -> impl Parser<&'a str, T, E>
where F: Parser<&'a str, T, E> {
    delimited_trim(SCOPE_LEFT, f, SCOPE_RIGHT)
}

fn scope<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    scoped(compose(ctx)).context(label("scope"))
}

fn token<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, Token<T>, E> {
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

fn tokens<T: ParseRepr>(
    ctx: ParseCtx, occurrences: impl Into<Range>,
) -> impl Parser<&str, Vec<Token<T>>, E> {
    separated(occurrences, token(ctx), space(1 ..))
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

fn ext<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, Token<T>, E> {
    move |i: &mut _| {
        let i: &mut &str = i;
        let checkpoint = i.checkpoint();
        let symbol = trivial_symbol1.context(label("symbol")).parse_next(i)?;
        if i.starts_with(LEFT_DELIMITERS) {
            prefix(symbol, ctx).map(Token::Default).parse_next(i)
        } else {
            alt((
                keyword(symbol, checkpoint).map(Token::Default),
                empty.value(Token::Unquote(Symbol::from_str(symbol))),
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
            TRUE => Ok(T::from(Bit::true1())),
            FALSE => Ok(T::from(Bit::false1())),
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
            ABSTRACT => abstract1(ctx).parse_next(i),
            PAIR => scope(ctx.esc_struct(Struct::Pair)).parse_next(i),
            CHANGE => scope(ctx.esc_struct(Struct::Change)).parse_next(i),
            CALL => scope(ctx.esc_struct(Struct::Call)).parse_next(i),
            OPTIMIZE => scope(ctx.esc_struct(Struct::Optimize)).parse_next(i),
            SOLVE => scope(ctx.esc_struct(Struct::Solve)).parse_next(i),
            LEFT => scope(ctx.esc_direction(Direction::Left)).parse_next(i),
            RIGHT => scope(ctx.esc_direction(Direction::Right)).parse_next(i),
            ARITY_2 => scope(ctx.esc_arity(Arity::Two)).parse_next(i),
            ARITY_3 => scope(ctx.esc_arity(Arity::Three)).parse_next(i),
            TAG => scope(ctx.escape()).parse_next(i),
            s if s.starts_with(TAG_CHAR) => scope(ctx.tag(&s[1 ..])).parse_next(i),
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

fn compose<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> + Clone {
    move |i: &mut _| {
        let tokens = tokens(ctx, 1 ..).parse_next(i)?;
        compose_tokens(ctx, i, tokens.into_iter())
    }
}

fn compose_tokens<'a, T, I>(ctx: ParseCtx<'a>, i: &mut &'a str, mut tokens: I) -> ModalResult<T>
where
    T: ParseRepr,
    I: ExactSizeIterator<Item = Token<T>> + DoubleEndedIterator<Item = Token<T>>, {
    let len = tokens.len();
    if len == 0 {
        return fail.parse_next(i);
    }
    if ctx.is_tag {
        return Ok(compose_tag(ctx, tokens, ctx.tag));
    }
    if len == 1 {
        let repr = tokens.next().unwrap().into_repr();
        return Ok(repr);
    }
    if len == 2 {
        let func = tokens.next().unwrap().into_repr();
        let input = tokens.next().unwrap().into_repr();
        return Ok(compose_two(ctx, func, input));
    }
    match ctx.arity {
        Arity::Two => match ctx.direction {
            Direction::Left => Ok(compose_many2(ctx, tokens)),
            Direction::Right => Ok(compose_many2(ctx, tokens.rev())),
        },
        Arity::Three => {
            if len % 2 == 0 {
                return cut_err(fail.context(expect_desc("odd number of tokens"))).parse_next(i);
            }
            match ctx.direction {
                Direction::Left => Ok(compose_many3(ctx, tokens)),
                Direction::Right => Ok(compose_many3(ctx, tokens.rev())),
            }
        }
    }
}

fn compose_tag<T, I>(ctx: ParseCtx, tokens: I, tag: &str) -> T
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>, {
    let list = tokens.map(Token::into_repr).collect::<List<_>>();
    let list = T::from(list);
    let tag = T::from(Symbol::from_str(tag));
    compose_two(ctx, tag, list)
}

fn compose_one<T: ParseRepr>(ctx: ParseCtx, token: Token<T>) -> T {
    if ctx.is_tag { compose_tag(ctx, [token].into_iter(), ctx.tag) } else { token.into_repr() }
}

fn compose_two<T: ParseRepr>(ctx: ParseCtx, left: T, right: T) -> T {
    match ctx.struct1 {
        Struct::Pair => T::from(Pair::new(left, right)),
        Struct::Change => T::from(Change::new(left, right)),
        Struct::Call => T::from(Call::new(left, right)),
        Struct::Optimize => T::from(Optimize::new(left, right)),
        Struct::Solve => T::from(Solve::new(left, right)),
    }
}

fn compose_many2<T, I>(ctx: ParseCtx, mut iter: I) -> T
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
        let (left, right) = match ctx.direction {
            Direction::Left => (first1, second),
            Direction::Right => (second, first1),
        };
        first = Token::Default(compose_two(ctx, left, right));
    }
    first.into_repr()
}

fn compose_many3<T, I>(ctx: ParseCtx, mut iter: I) -> T
where
    T: ParseRepr,
    I: Iterator<Item = Token<T>>, {
    let mut first = iter.next().unwrap();
    loop {
        let Some(middle) = iter.next() else {
            break;
        };
        let first1 = first.into_repr();
        let last = iter.next().unwrap().into_repr();
        let (left, right) = match ctx.direction {
            Direction::Left => (first1, last),
            Direction::Right => (last, first1),
        };
        first = Token::Default(compose_infix(ctx, left, middle, right));
    }
    first.into_repr()
}

fn compose_infix<T: ParseRepr>(ctx: ParseCtx, left: T, middle: Token<T>, right: T) -> T {
    let middle = match middle {
        Token::Unquote(s) => match &*s {
            PAIR => return T::from(Pair::new(left, right)),
            CHANGE => return T::from(Change::new(left, right)),
            CALL => return T::from(Call::new(left, right)),
            OPTIMIZE => return T::from(Optimize::new(left, right)),
            SOLVE => return T::from(Solve::new(left, right)),
            _ => T::from(s),
        },
        Token::Default(middle) => middle,
    };
    let pair = Pair::new(left, right);
    let pair = T::from(pair);
    compose_two(ctx, middle, pair)
}

fn abstract1<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    scope(ctx).map(|t| T::from(Abstract::new(t)))
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

fn list<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    let items = items(compose(ctx), SEPARATOR.context(expect_char(SEPARATOR)))
        .map(|list| T::from(List::from(list)));
    delimited_trim(LIST_LEFT, items, LIST_RIGHT).context(label("list"))
}

fn raw_list<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    let items = tokens(ctx, 0 ..).map(|tokens| {
        let list: List<T> = tokens.into_iter().map(Token::into_repr).collect();
        T::from(list)
    });
    delimited_trim(LIST_LEFT, items, LIST_RIGHT).context(label("raw list"))
}

fn map<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    let items = items(key_value(ctx), SEPARATOR.context(expect_char(SEPARATOR)));
    delimited_trim(MAP_LEFT, items, MAP_RIGHT)
        .map(|pairs| T::from(Map::from_iter(pairs)))
        .context(label("map"))
}

fn key_value<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, (T, T), E> + Clone {
    move |i: &mut _| {
        let key = token(ctx).parse_next(i)?;
        let key = compose_one(ctx, key);
        let pair = opt(preceded(space(1 ..), PAIR.void())).parse_next(i).unwrap();
        if pair.is_none() {
            return Ok((key, T::from(Unit)));
        }
        let value = cut_err(preceded(
            space(1 ..).context(expect_desc("space")),
            compose(ctx).context(expect_desc("value")),
        ))
        .parse_next(i)?;
        Ok((key, value))
    }
}

fn raw_map<T: ParseRepr>(ctx: ParseCtx) -> impl Parser<&str, T, E> {
    let items = move |i: &mut _| {
        let tokens = tokens(ctx, 0 ..).parse_next(i)?;
        if tokens.len() % 2 != 0 {
            return cut_err(fail.context(expect_desc("even number of tokens"))).parse_next(i);
        }
        let mut map = Map::with_capacity(tokens.len() / 2);
        let mut tokens = tokens.into_iter();
        while let Some(key) = tokens.next() {
            let value = tokens.next().unwrap();
            map.insert(key.into_repr(), value.into_repr());
        }
        Ok(T::from(map))
    };
    delimited_trim(MAP_LEFT, items, MAP_RIGHT).context(label("raw map"))
}

fn symbol(i: &mut &str) -> ModalResult<Symbol> {
    let literal = take_while(1 .., |c| is_symbol(c) && c != '\\' && c != SYMBOL_QUOTE);
    let fragment = alt((literal, symbol_escaped, symbol_space));
    let symbol = repeat(0 .., fragment).fold(String::new, |mut string, fragment| {
        string.push_str(fragment);
        string
    });
    delimited_cut(SYMBOL_QUOTE, symbol, SYMBOL_QUOTE)
        .map(Symbol::from_string)
        .context(label("symbol"))
        .parse_next(i)
}

fn symbol_escaped<'a>(i: &mut &'a str) -> ModalResult<&'a str> {
    preceded('\\', move |i: &mut _| match any.parse_next(i)? {
        '\\' => empty.value("\\").parse_next(i),
        '_' => empty.value(" ").parse_next(i),
        SYMBOL_QUOTE => empty.value(concatcp!(SYMBOL_QUOTE)).parse_next(i),
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
        .map(Symbol::from_string)
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
        TEXT_QUOTE => empty.value(StrFragment::Char(TEXT_QUOTE)).parse_next(i),
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

fn int(i: &mut &str) -> ModalResult<Int> {
    let int = (sign, integral).map(|(sign, i)| build_int(sign, i));
    scoped(int).context(label("int")).parse_next(i)
}

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
    radix: u8, f: F, desc: &'static str,
) -> impl Parser<&'a str, Significand, E>
where F: Parser<&'a str, &'a str, E> + Clone {
    let int = trim_num1(f.clone());
    let fraction = opt(preceded('.', cut_err(trim_num0(f).context(expect_desc("fraction")))));
    (int, fraction)
        .map(move |(int, fraction)| build_significand(radix, int, fraction))
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

fn byte(i: &mut &str) -> ModalResult<Byte> {
    let hex = preceded('X', cut_err(hexadecimal_byte));
    let bin = preceded('B', cut_err(binary_byte));
    let byte = alt((hex, bin, hexadecimal_byte));
    scoped(byte).context(label("byte")).parse_next(i)
}

fn hexadecimal_byte(i: &mut &str) -> ModalResult<Byte> {
    let digits = hexadecimal1.verify(|s: &str| s.len() % 2 == 0);
    trim_num0(digits)
        .map(|s| {
            let byte = utils::conversion::hex_str_to_vec_u8(&s).unwrap();
            Byte::from(byte)
        })
        .context(expect_desc("hexadecimal"))
        .parse_next(i)
}

fn binary_byte(i: &mut &str) -> ModalResult<Byte> {
    let digits = binary1.verify(|s: &str| s.len() % 8 == 0);
    trim_num0(digits)
        .map(|s| {
            let byte = utils::conversion::bin_str_to_vec_u8(&s).unwrap();
            Byte::from(byte)
        })
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
