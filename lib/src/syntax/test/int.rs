use crate::syntax::repr::Repr;
use crate::syntax::test::int;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        int("0", 10),
        int("00", 10),
        int("0", 10),
        int("-0", 10),
        int("123", 10),
        int("123", 10),
        int("-123", 10),
        int("11111111222222223333333344444444", 10),
        int("11111111222222223333333344444444", 10),
        int("123", 10),
        int("-123", 10),
        int("0", 16),
        int("00", 16),
        int("a0b1c2d3", 16),
        int("-a0b1c2d3", 16),
        int("9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        int("9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        int("0", 2),
        int("11010010", 2),
        int("-11010010", 2),
        int("0", 10),
        int("1", 10),
        int("1", 10),
        int("-1", 10),
        int("123", 10),
        int("-123", 10),
        int("a0b1c2d3", 16),
        int("-a0b1c2d3", 16),
        int("11010010", 2),
        int("-11010010", 2),
    ]
}
