use {
    crate::grammar::{
        lexer::ParseResult,
        parse::lexer::{
            Token,
            UnitLexer,
        },
        ParseError,
    },
    regex::Regex,
};

#[cfg(test)]
mod test;

pub(crate) struct StringLexer {
    pattern: Regex,
    delimiter_pattern: Regex,
}

impl StringLexer {
    pub(crate) fn new() -> StringLexer {
        StringLexer {
            pattern: Regex::new(
                "(?x)
        \"
        (?P<body>(?:
            [^\"\\\\] # everything other than \" or \\
            |
            \\\\[\\ strn\\\\\"] # escape space, tab, newline and \"
            |
            \\\\[uU]\\{[0-9a-fA-F]{1,6}\\} # unicode
        )*)
        \"
        ",
            )
            .unwrap(),
            delimiter_pattern: Regex::new("[ \\t\\r\\n]*[\\t\\r\\n]+[ \\t\\r\\n]*").unwrap(),
        }
    }
}

impl UnitLexer for StringLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        let m = captures.name("body").unwrap();
        let s = self.delimiter_pattern.replace_all(m.as_str(), "");
        let mut ns = String::new();
        let mut escape = false;
        let mut iter = s.chars().into_iter();
        while let Some(c) = iter.next() {
            if escape {
                let escaped = match c {
                    ' ' | 's' => ' ',
                    't' => '\t',
                    'r' => '\r',
                    'n' => '\n',
                    'u' | 'U' => {
                        let mut codepoint = String::new();
                        while let Some(hex) = iter.next() {
                            if hex == '{' {
                                continue;
                            }
                            if hex == '}' {
                                break;
                            }
                            codepoint.push(hex);
                        }
                        let i = u32::from_str_radix(codepoint.as_str(), 16).unwrap();
                        let i = char::from_u32(i);
                        if i.is_none() {
                            return ParseError::err(format!("invalid unicode {codepoint}"));
                        }
                        i.unwrap()
                    }
                    _ => c,
                };
                ns.push(escaped);
                escape = false
            } else {
                if c == '\\' {
                    escape = true
                } else {
                    ns.push(c)
                }
            }
        }
        Ok(Token::String(ns))
    }
}
