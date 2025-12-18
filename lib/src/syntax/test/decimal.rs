use crate::syntax::repr::Repr;
use crate::syntax::test::decimal;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        decimal(true, 0, "00"),
        decimal(true, 0, "10"),
        decimal(true, 0, "0"),
        decimal(true, 0, "1"),
        decimal(false, -123, "123455666"),
        decimal(true, 1000, "1111111111111111111111111111111"),
        decimal(true, 0, "3141592653589793"),
        decimal(true, 0, "2718281828459045"),
        decimal(true, 0, "0"),
        decimal(true, 0, "1"),
        decimal(true, 0, "1"),
        decimal(false, 0, "1"),
        decimal(true, 2, "123"),
        decimal(false, 2, "123"),
        decimal(true, 2, "12345"),
    ]
}
