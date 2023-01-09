use crate::grammar::parse::lexer::{
    Token,
    Token::String,
};

pub(crate) fn expected() -> Vec<Token> {
    vec![
        String("".to_owned()),
        String("abc".to_owned()),
        String("0123".to_owned()),
        String("`~!@#$%^&*()-_=+[]{};:'\",.<>?\\|/".to_owned()),
        String("🜁🜂🜃🜄".to_owned()),
        String("   \n\r\t\u{1F701}".to_owned()),
        String(" \\ \\s\\n\\r\\t\\u{1F701}".to_owned()),
        String(" \\ \\ \\\n\\\r\\\t\\\u{1F701}".to_owned()),
        String("multiple lines".to_owned()),
        String("leading\nspaces\n  indent".to_owned()),
    ]
}
