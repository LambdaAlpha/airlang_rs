use crate::syntax::repr::Repr;
use crate::syntax::test::text;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        text(""),
        text("()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%&"),
        text("abc ABC 0123"),
        text("🜁🜂🜃🜄"),
        text("  \n\r\t\u{1f701}"),
        text(" ^_^n^r^t^u(1f701)"),
        text(" ^ ^\n^\r^\t^\u{1f701}"),
        text("\u{1f701}"),
        text(" \t\r\n \t\r\n"),
        text(" \t\r\n\n \t\r\n"),
        text("^_^t^r^n^_^t^r^n()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%&\n"),
        text(" \t\r\n \t\r\n^_^t^r^n^_^t^r^n \t\r\n \t\r\n"),
        text(" \t\r\n \t\r\n^_^t^r^n^_^t^r^n \t\r\n \t\r\n"),
        text(""),
        text(""),
    ]
}
