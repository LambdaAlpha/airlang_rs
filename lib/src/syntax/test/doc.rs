use crate::syntax::repr::Repr;
use crate::syntax::test::bit;
use crate::syntax::test::byte;
use crate::syntax::test::call;
use crate::syntax::test::cell;
use crate::syntax::test::decimal;
use crate::syntax::test::infix_call;
use crate::syntax::test::int;
use crate::syntax::test::key;
use crate::syntax::test::list;
use crate::syntax::test::map;
use crate::syntax::test::pair;
use crate::syntax::test::text;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        list(vec![int("1", 10), int("4", 10)]),
        map(vec![("a", int("2", 10))]),
        key("key"),
        key(">="),
        key("a.b.c"),
        key("[0, 1, 2]"),
        key(
            " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%&0123456789",
        ),
        text("🜁🜁"),
        text("- a\r\n\t- a.1\r\n\t- a.2"),
        text("- a\n  - a.1\n  - a.2"),
        int("123", 10),
        int("-123", 10),
        int("-123", 10),
        int("7f", 16),
        int("-1110", 2),
        decimal(true, -1, "1"),
        decimal(false, -1, "1"),
        decimal(false, -12, "3456"),
        decimal(false, -1, "1"),
        byte(vec![0b00001111]),
        byte(vec![0x00, 0xff, 0xff]),
        cell(bit(true)),
        cell(key("value")),
        cell(cell(text("data"))),
        pair(key("a"), int("1", 10)),
        pair(key("a"), pair(key("b"), key("c"))),
        list(vec![int("0", 10), int("1", 10), int("2", 10)]),
        list(vec![unit(), bit(false), int("0", 10), key("")]),
        map(vec![("a", int("1", 10)), ("b", int("2", 10)), ("c", int("3", 10))]),
        map(vec![("a", int("1", 10)), ("b", bit(true)), ("c", key(" "))]),
        map(vec![("a", unit()), ("b", unit()), ("c", unit())]),
        call(key("not"), bit(true)),
        infix_call(int("1", 10), key("+"), int("1", 10)),
        infix_call(key("a"), key("and"), infix_call(key("b"), key("or"), key("c"))),
    ]
}
