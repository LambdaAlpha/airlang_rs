use crate::syntax::{
    repr::Repr,
    test::string,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        string(""),
        string("abc"),
        string("0123"),
        string("`~!@#$%^&*()-_=+[]{};:'\",.<>?\\|/"),
        string("🜁🜂🜃🜄"),
        string("   \n\r\t\u{1F701}"),
        string(" \\ \\s\\n\\r\\t\\u{1F701}"),
        string(" \\ \\ \\\n\\\r\\\t\\\u{1F701}"),
        string("multiple lines"),
        string("a\nb\n  c"),
    ]
}
