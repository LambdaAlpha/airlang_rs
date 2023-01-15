use {
    crate::grammar::{
        parse::{
            lexer::config::AirLexerConfig,
            AtomNode,
        },
        repr::{
            Bytes,
            Float,
            Int,
        },
        ParseError,
        ParseResult,
    },
    regex::{
        Captures,
        Regex,
    },
};

mod config;
#[cfg(test)]
mod test;
mod units;

#[derive(Debug)]
pub(crate) enum FlatNode {
    Symbol(String),
    Atom(AtomNode),
}

pub(crate) type FlatNodes = Vec<FlatNode>;

pub(crate) fn parse(src: &str) -> ParseResult<FlatNodes> {
    let tokens = lexing(src)?;

    let mut nodes = Vec::with_capacity(tokens.len());
    for token in tokens {
        let node = match token {
            Token::Unit => FlatNode::Atom(AtomNode::Unit),
            Token::Bool(b) => FlatNode::Atom(AtomNode::Bool(b)),
            Token::Int(i) => FlatNode::Atom(AtomNode::Int(i)),
            Token::Float(f) => FlatNode::Atom(AtomNode::Float(f)),
            Token::Symbol(symbol) => FlatNode::Symbol(symbol),
            Token::Letter(l) => FlatNode::Atom(AtomNode::Letter(l)),
            Token::String(s) => FlatNode::Atom(AtomNode::String(s)),
            Token::Bytes(b) => FlatNode::Atom(AtomNode::Bytes(b)),
            Token::Delimiter(_) => continue,
        };
        nodes.push(node);
    }
    Ok(nodes)
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Delimiter(String),
    Unit,
    Bool(bool),
    Int(Int),
    Float(Float),
    Symbol(String),
    Letter(String),
    String(String),
    Bytes(Bytes),
}

pub(crate) trait LexerConfig {
    fn dispatch_char(&self, c: char, next: Option<char>) -> ParseResult<&dyn UnitLexer>;
}

pub(crate) trait UnitLexer {
    fn pattern(&self) -> &Regex;
    fn lexing(&self, captures: &Captures) -> ParseResult<Token>;
}

fn lexing(src: &str) -> ParseResult<Vec<Token>> {
    let config = AirLexerConfig::new();
    let mut tokens = Vec::<Token>::new();
    let mut rest = &src[..];
    while !rest.is_empty() {
        let first = rest.chars().next().unwrap();
        let next = rest.chars().nth(1);

        let lexer = config.dispatch_char(first, next)?;

        let captures = lexer.pattern().captures(rest);
        if captures.is_none() {
            return ParseError::err(format!(
                "pattern matching failed on first: {first}, next: {next:?}"
            ));
        }
        let captures = captures.unwrap();

        let m0 = captures.get(0).unwrap();
        rest = &rest[m0.end()..];
        let token = lexer.lexing(&captures)?;
        if !matches!(token, Token::Delimiter(_)) {
            tokens.push(token);
        }
    }
    return Ok(tokens);
}
