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
use crate::syntax::test::quote;
use crate::syntax::test::solve;
use crate::syntax::test::text;
use crate::syntax::test::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        unit(),
        bit(true),
        bit(false),
        key("key"),
        key("key"),
        key(">="),
        key("a.b.c"),
        key("[0, 1, 2]"),
        key("\"a\"?'\")( 'a'"),
        key(
            "abcdefghijklmnopqrstuvwxyz()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%& 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        ),
        text("text"),
        text("🜁: Alchemical Symbol For Air"),
        text("'a'🜁'\" \t\r\n\"a\""),
        text("()[]{}<>\\|/'\"`^*+=-~_.,:;!?@#$%&🜁'\" \t\r\n"),
        int("0", 10),
        int("-1", 10),
        int("123", 10),
        int("-123", 10),
        int("-123", 10),
        int("7f", 16),
        int("7f", 16),
        int("-1110", 2),
        decimal(true, 0, "0"),
        decimal(false, 0, "1"),
        decimal(true, 1, "123"),
        decimal(false, 1, "123"),
        decimal(false, 1, "123"),
        decimal(false, -12, "3456"),
        decimal(false, -12, "3456"),
        byte(vec![0x00]),
        byte(vec![0b00001111]),
        byte(vec![0x00, 0xff, 0xff]),
        cell(key("v")),
        cell(key("key")),
        cell(text("text")),
        cell(list(vec![key("l"), key("i"), key("s"), key("t")])),
        cell(map(vec![("a", key("map"))])),
        cell(bit(true)),
        cell(key("cell")),
        cell(cell(list(vec![cell(map(vec![("a", cell(text("")))]))]))),
        pair(key("left"), key("right")),
        pair(key("a"), int("1", 10)),
        pair(key("a"), pair(key("b"), key("c"))),
        list(vec![key("v1"), key("v2"), key("..."), key("vn")]),
        list(vec![key("v1"), key("v2"), key("..."), key("vn")]),
        list(vec![int("0", 10), int("1", 10), int("2", 10)]),
        list(vec![unit(), bit(false), int("0", 10), key("")]),
        list(vec![key("git"), key("commit"), key("--amend"), key("--no-edit")]),
        map(vec![("k1", key("v1")), ("k2", key("v2")), ("...", key("...")), ("kn", key("vn"))]),
        map(vec![("k1", key("v1")), ("k2", key("v2")), ("...", key("...")), ("kn", key("vn"))]),
        map(vec![("a", int("1", 10)), ("b", int("2", 10)), ("c", int("3", 10))]),
        map(vec![("a", int("1", 10)), ("b", bit(true)), ("c", key(" "))]),
        map(vec![("a", unit()), ("b", unit()), ("c", unit())]),
        map(vec![
            ("select", key("*")),
            ("from", key("book")),
            ("where", infix_call(key("price"), key(">"), int("100", 10))),
            ("order_by", key("title")),
        ]),
        quote(key("v")),
        quote(key("key")),
        quote(text("text")),
        quote(list(vec![key("l"), key("i"), key("s"), key("t")])),
        quote(map(vec![("a", key("map"))])),
        quote(bit(true)),
        quote(key("quote")),
        quote(quote(list(vec![quote(map(vec![("a", quote(text("")))]))]))),
        call(key("not"), bit(true)),
        infix_call(int("1", 10), key("+"), int("1", 10)),
        infix_call(key("a"), key("and"), infix_call(key("b"), key("or"), key("c"))),
        solve(key("*"), int("21", 10)),
        solve(key("is_carmichael_number"), bit(true)),
        unit(),
        unit(),
        unit(),
        unit(),
        unit(),
        unit(),
        list(vec![int("1", 10), int("4", 10)]),
        map(vec![("a", int("2", 10))]),
    ]
}
