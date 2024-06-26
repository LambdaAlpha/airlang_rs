use crate::syntax::{
    repr::Repr,
    test::text,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        text(""),
        text("abc"),
        text("0123"),
        text("`~!@#$%^&*()-_=+[]{};:'\",.<>?\\|/"),
        text("🜁🜂🜃🜄"),
        text("   \n\r\t\u{1F701}"),
        text(" \\ \\s\\n\\r\\t\\u(1F701)"),
        text(" \\ \\ \\\n\\\r\\\t\\\u{1F701}"),
        text("\u{1F701}"),
        text("multiple lines"),
        text("a\nb\n  c"),
    ]
}
