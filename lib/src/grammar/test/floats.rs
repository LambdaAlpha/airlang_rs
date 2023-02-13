use crate::{
    grammar::test::float,
    repr::Repr,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        float(true, "0", "0", true, "0"),
        float(true, "0", "0", true, "0"),
        float(false, "0", "0", true, "0"),
        float(true, "0", "", true, "0"),
        float(true, "0", "", true, "0"),
        float(false, "0", "", true, "0"),
        float(true, "1", "", true, "0"),
        float(true, "1", "", true, "2"),
        float(true, "3", "141592653589793", true, ""),
        float(true, "2", "718281828459045", true, ""),
        float(false, "123", "455666", false, "123"),
        float(true, "1", "111111111111111111111111111111", true, "100"),
    ]
}
