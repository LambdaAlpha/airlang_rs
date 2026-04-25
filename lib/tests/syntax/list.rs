use airlang::syntax::repr::Repr;

use crate::key;
use crate::list;
use crate::pair;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        list(vec![]),
        list(vec![]),
        list(vec![key("a")]),
        list(vec![key("a")]),
        list(vec![key("a")]),
        list(vec![key("a"), key("b")]),
        list(vec![key("a"), key("b")]),
        list(vec![pair(key("a"), key("b")), key("c")]),
        list(vec![key("a"), key("b")]),
        list(vec![list(vec![])]),
        list(vec![list(vec![]), list(vec![])]),
    ]
}
