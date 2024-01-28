use crate::syntax::{
    repr::Repr,
    test::bytes,
};

pub(crate) fn expected() -> Vec<Repr> {
    vec![
        bytes(vec![]),
        bytes(vec![]),
        bytes(vec![]),
        bytes(vec![0b00000000]),
        bytes(vec![0x00, 0x00, 0x00, 0x00]),
        bytes(vec![0xff]),
        bytes(vec![0x0f, 0x0f, 0x0f]),
        bytes(vec![0x00, 0x11, 0xff, 0xee]),
    ]
}
