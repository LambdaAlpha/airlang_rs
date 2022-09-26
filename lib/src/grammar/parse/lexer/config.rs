use super::{
    units::{
        delimeter::DelimeterLexer, letter::LetterLexer, num::NumLexer, string::StringLexer,
        symbol::SymbolLexer,
    },
    LexerConfig, ParseError, ParseResult, UnitLexer,
};

pub(crate) struct AirLexerConfig {
    delimeter_lexer: DelimeterLexer,
    num_lexer: NumLexer,
    symbol_lexer: SymbolLexer,
    letter_lexer: LetterLexer,
    string_lexer: StringLexer,
}

impl AirLexerConfig {
    pub(crate) fn new() -> AirLexerConfig {
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
    fn dispatch_char(&self, c: char) -> ParseResult<&dyn UnitLexer> {
        match c {
            ' ' | '\t' | '\r' | '\n' => Ok(&self.delimeter_lexer),
            'a'..='z' | 'A'..='Z' => Ok(&self.letter_lexer),
            '0'..='9' | '+' | '-' => Ok(&self.num_lexer),
            '"' => Ok(&self.string_lexer),
            _ if c.is_ascii_punctuation() => Ok(&self.symbol_lexer),
            _ => ParseError::err(format!("no unit lexer found for {c}")),
        }
    }
}
