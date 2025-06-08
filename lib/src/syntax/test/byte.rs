use crate::syntax::repr::Repr;
use crate::syntax::test::byte;

pub(super) fn expected() -> Vec<Repr> {
    vec![
        byte(vec![]),
        byte(vec![]),
        byte(vec![]),
        byte(vec![0b00000000]),
        byte(vec![0x00, 0x00, 0x00, 0x00]),
        byte(vec![0xff]),
        byte(vec![0x0f, 0x0f, 0x0f]),
        byte(vec![0x00, 0x11, 0xff, 0xee]),
        byte(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
    ]
}
