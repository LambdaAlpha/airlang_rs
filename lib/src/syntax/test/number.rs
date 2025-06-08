use crate::syntax::repr::Repr;
use crate::syntax::test::number;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        number(10, "00", 1, "0"),
        number(10, "00", 1, "0"),
        number(10, "-00", 1, "0"),
        number(10, "0", 0, "0"),
        number(10, "0", 0, "0"),
        number(10, "-0", 0, "0"),
        number(10, "1", 0, "0"),
        number(10, "1", 0, "2"),
        number(10, "-123455666", 6, "-123"),
        number(10, "1111111111111111111111111111111", 30, "100"),
        number(16, "1ff1", 2, "-3"),
        number(16, "-1ff1", 2, "-3"),
        number(16, "1f", 0, "-3"),
        number(16, "-1ff1", 2, "0"),
        number(2, "101", 2, "-3"),
        number(2, "-1", 0, "-3"),
        number(2, "101", 2, "0"),
        number(10, "3141592653589793", 15, "0"),
        number(10, "2718281828459045", 15, "0"),
        number(10, "0", 0, "0"),
        number(10, "1", 0, "0"),
        number(10, "1", 0, "0"),
        number(10, "-1", 0, "0"),
        number(10, "123", 0, "0"),
        number(10, "-123", 0, "0"),
        number(16, "1ff1", 2, "-3"),
        number(16, "-1ff1", 2, "0"),
        number(2, "101", 2, "0"),
        number(2, "-101", 2, "-3"),
        number(10, "12345", 2, "0"),
    ]
}
