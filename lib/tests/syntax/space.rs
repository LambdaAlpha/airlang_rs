use airlang::syntax::repr::Repr;

use crate::infix_call;
use crate::key;
use crate::list;
use crate::map;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        key("a"),
        key("a"),
        key("a"),
        infix_call(key("a"), key("b"), key("c")),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        key("a"),
        infix_call(key("a"), key("b"), key("c")),
        key("a"),
        key("a"),
        list(vec![key("a"), key("d")]),
        map(vec![("a", key("b"))]),
        key("c"),
    ]
}
