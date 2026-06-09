use airlang::syntax::repr::Repr;

use crate::key;
use crate::list;
use crate::map;
use crate::pair;
use crate::quote;
use crate::text;
use crate::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        quote(text("")),
        quote(text("")),
        quote(list(vec![])),
        quote(map(vec![])),
        quote(unit()),
        quote(quote(key("a"))),
        quote(pair(key("a"), key("b"))),
        quote(list(vec![key("a"), key("b")])),
        quote(map(vec![("a", key("b"))])),
        pair(key("a"), quote(key("b"))),
        list(vec![quote(key("a"))]),
        map(vec![("a", quote(key("b")))]),
    ]
}
