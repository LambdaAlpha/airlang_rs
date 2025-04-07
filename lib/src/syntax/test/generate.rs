use crate::syntax::{
    Repr,
    test::{
        generate,
        list,
        map,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        generate(symbol("a")),
        generate(generate(symbol("a"))),
        generate(list(vec![symbol("a"), symbol("b")])),
        list(vec![generate(symbol("a"))]),
        map(vec![(generate(symbol("a")), symbol("b"))]),
        map(vec![(symbol("a"), generate(symbol("b")))]),
    ]
}
