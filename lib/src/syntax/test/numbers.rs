use crate::syntax::{
    repr::Repr,
    test::number,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        number(true, "0", "0", true, ""),
        number(true, "0", "0", true, ""),
        number(false, "0", "0", true, ""),
        number(true, "0", "", true, ""),
        number(true, "0", "", true, ""),
        number(false, "0", "", true, ""),
        number(true, "1", "", true, ""),
        number(true, "1", "", true, "2"),
        number(true, "3", "141592653589793", true, ""),
        number(true, "2", "718281828459045", true, ""),
        number(false, "123", "455666", false, "123"),
        number(true, "1", "111111111111111111111111111111", true, "100"),
    ]
}
