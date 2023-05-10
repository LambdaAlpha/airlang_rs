use crate::syntax::{
    repr::Repr,
    test::int,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        int(true, "0", 10),
        int(true, "00", 10),
        int(true, "0", 10),
        int(false, "0", 10),
        int(true, "123", 10),
        int(true, "123", 10),
        int(false, "123", 10),
        int(true, "11111111222222223333333344444444", 10),
        int(true, "11111111222222223333333344444444", 10),
        int(true, "0", 16),
        int(true, "00", 16),
        int(true, "a0b1c2d3", 16),
        int(false, "a0b1c2d3", 16),
        int(true, "9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        int(true, "9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        int(true, "0", 2),
        int(true, "11010010", 2),
        int(false, "11010010", 2),
    ]
}
