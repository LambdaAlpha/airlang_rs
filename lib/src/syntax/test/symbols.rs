use crate::syntax::{
    repr::Repr,
    test::{
        call,
        list,
        symbol,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        symbol(""),
        symbol("a"),
        symbol("Abc"),
        symbol("A_BB__CCC_"),
        symbol("A1B2C3"),
        symbol("`"),
        symbol("~"),
        symbol("!"),
        symbol("@"),
        symbol("#"),
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
        symbol(":"),
        symbol("'"),
        symbol("\""),
        symbol("."),
        symbol("<"),
        symbol(">"),
        symbol("/"),
        symbol("?"),
        symbol("("),
        symbol(")"),
        symbol("["),
        symbol("]"),
        symbol("{"),
        symbol("}"),
        symbol(","),
        symbol("\\"),
        symbol("+="),
        symbol("<="),
        symbol("->"),
        symbol("==="),
        symbol("!=="),
        list(vec![symbol("%"), call(list(vec![]), list(vec![]))]),
        symbol("a%"),
        symbol("%a"),
        symbol("%'"),
        symbol("'%"),
        symbol("@a"),
        symbol("a@"),
        symbol(":a"),
        symbol("a:"),
        symbol("$a"),
        symbol("a$"),
        symbol("a?"),
        symbol("?a"),
        symbol("a\""),
        symbol("\"a"),
        symbol("1"),
        symbol("1.0e-10"),
        symbol("1a"),
        symbol("+"),
        symbol("+1"),
        symbol("+1.0e-10"),
        symbol("+1.a"),
        symbol("-1.0"),
        symbol("true"),
        symbol("'$"),
        symbol("'?"),
        symbol("'a"),
        symbol("ab"),
    ]
}
