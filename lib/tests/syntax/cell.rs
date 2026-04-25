use airlang::syntax::repr::Repr;

use crate::cell;
use crate::key;
use crate::list;
use crate::map;
use crate::pair;
use crate::text;
use crate::unit;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        cell(key("")),
        cell(text("")),
        cell(list(vec![])),
        cell(map(vec![])),
        cell(unit()),
        cell(cell(key("a"))),
        cell(pair(key("a"), key("b"))),
        cell(list(vec![key("a"), key("b")])),
        cell(map(vec![("a", key("b"))])),
        pair(key("a"), cell(key("b"))),
        list(vec![cell(key("a"))]),
        map(vec![("a", cell(key("b")))]),
    ]
}
