use crate::grammar::{
    parse::lexer::{
        units::{
            delimiter::DelimiterLexer,
            letter::LetterLexer,
            num::NumLexer,
            preserve::PreserveLexer,
            string::StringLexer,
            symbol::SymbolLexer,
        },
        LexerConfig,
        UnitLexer,
    },
    ParseError,
    ParseResult,
};

pub(crate) struct AirLexerConfig {
    delimiter_lexer: DelimiterLexer,
    num_lexer: NumLexer,
    preserve_lexer: PreserveLexer,
    symbol_lexer: SymbolLexer,
    letter_lexer: LetterLexer,
    string_lexer: StringLexer,
}

impl AirLexerConfig {
    pub(crate) fn new() -> AirLexerConfig {
        AirLexerConfig {
            delimiter_lexer: DelimiterLexer::new(),
            num_lexer: NumLexer::new(),
            preserve_lexer: PreserveLexer::new(),
            symbol_lexer: SymbolLexer::new(),
            letter_lexer: LetterLexer::new(),
            string_lexer: StringLexer::new(),
        }
    }
}

impl LexerConfig for AirLexerConfig {
    fn dispatch_char(&self, c: char, next: Option<char>) -> ParseResult<&dyn UnitLexer> {
        match c {
            ' ' | '\t' | '\r' | '\n' => Ok(&self.delimiter_lexer),
            'a'..='z' | 'A'..='Z' => Ok(&self.letter_lexer),
            '0'..='9' => Ok(&self.num_lexer),
            '+' | '-' => Ok(match next {
                Some('0'..='9') => &self.num_lexer,
                _ => &self.symbol_lexer,
            }),
            '"' => Ok(&self.string_lexer),
            '\'' => Ok(match next {
                Some('a'..='z' | 'A'..='Z' | '0'..='9') => &self.preserve_lexer,
                _ => &self.symbol_lexer,
            }),
            _ if c.is_ascii_punctuation() => Ok(&self.symbol_lexer),
            _ => ParseError::err(format!("no unit lexer found for {c}")),
        }
    }
}
