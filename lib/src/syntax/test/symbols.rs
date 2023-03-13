use crate::{
    repr::Repr,
    syntax::test::{
        list,
        ltree,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        symbol("`"),
        symbol("~"),
        symbol("!"),
        symbol("@"),
        symbol("$"),
        symbol("%"),
        symbol("^"),
        symbol("&"),
        symbol("*"),
        symbol("-"),
        symbol("+"),
        symbol("_"),
        symbol("="),
        symbol("\\"),
        symbol("|"),
        symbol(";"),
        symbol("'"),
        symbol("."),
        symbol("<"),
        symbol(">"),
        symbol("/"),
        symbol("+="),
        symbol("<="),
        symbol("->"),
        symbol("==="),
        symbol("!=="),
        symbol("@\""),
        symbol("@'"),
        symbol("@#"),
        symbol("'@"),
        list(vec![symbol("@"), ltree(symbol("$"), vec![])]),
    ]
}
