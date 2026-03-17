use crate::syntax::repr::Repr;
use crate::syntax::test::text;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        text(""),
        text("()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%&"),
        text("abc ABC 0123"),
        text("🜁🜂🜃🜄"),
        text(
            "\u{00}\u{01}\u{02}\u{03}\u{04}\u{05}\u{06}\u{07}\
            \u{08}\u{09}\u{0A}\u{0B}\u{0C}\u{0D}\u{0E}\u{0F}\
            \u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\
            \u{18}\u{19}\u{1A}\u{1B}\u{1C}\u{1D}\u{1E}\u{1F}\
            \u{20}\u{7F}",
        ),
        text("a?🜁"),
        text("ab"),
        text("ab "),
        text("ab "),
        text("a\"^"),
        text("ab\"^ "),
        text("a"),
        text("ab\" end"),
        text("ab\" end"),
    ]
}
