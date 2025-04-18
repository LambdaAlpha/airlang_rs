use crate::syntax::{
    Repr,
    test::{
        symbol,
        that,
        this,
    },
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![this(symbol("a")), that(symbol("a")), this(that(symbol("a"))), that(this(symbol("a")))]
}
