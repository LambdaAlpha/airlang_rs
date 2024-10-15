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
        text("  \n\r\t\u{1f701}"),
        text(" \\_\\n\\r\\t\\u(1f701)"),
        text(" \\ \\\n\\\r\\\t\\\u{1f701}"),
        text("\u{1f701}"),
        text("multiple lines"),
        text("a\nb\n  cd"),
        text("  \n\r\t\u{1f701}"),
        text("`!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?"),
        text("12345 67890\n12345 67890\n"),
        text("\"\""),
        text(""),
    ]
}
