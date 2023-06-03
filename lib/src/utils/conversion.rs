use std::{
    fmt::Write,
    num::ParseIntError,
};

pub(crate) fn bin_str_to_vec_u8(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(8)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect()
}

pub(crate) fn u8_array_to_bin_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 8);
    u8_array_to_bin_string_mut(bytes, &mut s);
    s
}

pub(crate) fn u8_array_to_bin_string_mut(bytes: &[u8], s: &mut String) {
    for &b in bytes {
        write!(s, "{b:08b}").unwrap();
    }
}

pub(crate) fn hex_str_to_vec_u8(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub(crate) fn u8_array_to_hex_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    u8_array_to_hex_string_mut(bytes, &mut s);
    s
}

pub(crate) fn u8_array_to_hex_string_mut(bytes: &[u8], s: &mut String) {
    for &b in bytes {
        write!(s, "{b:02x}").unwrap();
    }
}
