use super::{
    units::{
        bytes::BytesLexer, delimeter::DelimeterLexer, letter::LetterLexer, num::NumLexer,
        string::StringLexer, symbol::SymbolLexer,
    },
    LexerConfig, LexerError, UnitLexer,
};

pub struct AirLexerConfig {
    delimeter_lexer: DelimeterLexer,
    bytes_lexer: BytesLexer,
    num_lexer: NumLexer,
    symbol_lexer: SymbolLexer,
    letter_lexer: LetterLexer,
    string_lexer: StringLexer,
}

impl AirLexerConfig {
    pub fn new() -> AirLexerConfig {
        AirLexerConfig {
            delimeter_lexer: DelimeterLexer::new(),
            bytes_lexer: BytesLexer::new(),
            num_lexer: NumLexer::new(),
            symbol_lexer: SymbolLexer::new(),
            letter_lexer: LetterLexer::new(),
            string_lexer: StringLexer::new(),
        }
    }
}

impl LexerConfig for AirLexerConfig {
    fn dispatch(&self, c: char) -> Result<&dyn UnitLexer, LexerError> {
        match c {
            ' ' | '\t' | '\r' | '\n' => Ok(&self.delimeter_lexer),
            'a'..='z' | 'A'..='Z' => Ok(&self.letter_lexer),
            '0'..='9' | '+' | '-' => Ok(&self.num_lexer),
            '"' => Ok(&self.string_lexer),
            '\'' => Ok(&self.bytes_lexer),
            _ if c.is_ascii_punctuation() => Ok(&self.symbol_lexer),
            _ => Err(LexerError {
                msg: format!("no unit lexer found for {c}"),
            }),
        }
    }
}
