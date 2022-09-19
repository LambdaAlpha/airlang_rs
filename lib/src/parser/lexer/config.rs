use super::{
    units::{
        delimeter::DelimeterLexer, letter::LetterLexer, num::NumLexer, string::StringLexer,
        symbol::SymbolLexer,
    },
    LexError, LexResult, LexerConfig, UnitLexer,
};

pub struct AirLexerConfig {
    delimeter_lexer: DelimeterLexer,
    num_lexer: NumLexer,
    symbol_lexer: SymbolLexer,
    letter_lexer: LetterLexer,
    string_lexer: StringLexer,
}

impl AirLexerConfig {
    pub fn new() -> AirLexerConfig {
        AirLexerConfig {
            delimeter_lexer: DelimeterLexer::new(),
            num_lexer: NumLexer::new(),
            symbol_lexer: SymbolLexer::new(),
            letter_lexer: LetterLexer::new(),
            string_lexer: StringLexer::new(),
        }
    }
}

impl LexerConfig for AirLexerConfig {
    fn dispatch_char(&self, c: char) -> LexResult<&dyn UnitLexer> {
        match c {
            ' ' | '\t' | '\r' | '\n' => Ok(&self.delimeter_lexer),
            'a'..='z' | 'A'..='Z' => Ok(&self.letter_lexer),
            '0'..='9' | '+' | '-' => Ok(&self.num_lexer),
            '"' => Ok(&self.string_lexer),
            _ if c.is_ascii_punctuation() => Ok(&self.symbol_lexer),
            _ => LexError::err(format!("no unit lexer found for {c}")),
        }
    }
    fn dispatch_token(&self, token: &super::Token) -> &dyn UnitLexer {
        match token {
            super::Token::Delimeter(_) => &self.delimeter_lexer,
            super::Token::Bool(_) => &self.symbol_lexer,
            super::Token::Int(_) => &self.num_lexer,
            super::Token::Float(_) => &self.num_lexer,
            super::Token::Symbol(s) => match s.as_str() {
                "+" | "-" => &self.num_lexer,
                _ => &self.symbol_lexer,
            },
            super::Token::Letter(_) => &self.letter_lexer,
            super::Token::String(_) => &self.string_lexer,
            super::Token::Bytes(_) => &self.symbol_lexer,
        }
    }
}
