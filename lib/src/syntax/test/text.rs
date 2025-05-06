use crate::syntax::repr::Repr;
use crate::syntax::test::text;

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        text(""),
        text(" ()[]{}<>\\|/'\":;!?,.`~@#$%^&*-+_="),
        text("abc ABC 0123"),
        text("🜁🜂🜃🜄"),
        text("  \n\r\t\u{1f701}"),
        text(" \\_\\n\\r\\t\\u(1f701)"),
        text(" \\ \\\n\\\r\\\t\\\u{1f701}"),
        text("\u{1f701}"),
        text("multiple lines"),
        text("a\nb\n  cd"),
        text("`!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?"),
        text("12345 67890\n12345 67890\n"),
        text("12345 67890\n12345 67890\n"),
        text("\"\""),
        text(""),
    ]
}
