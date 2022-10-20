use crate::grammar::{parse, ParseError, repr::Repr};

use super::ParseResult;

mod infixes;
mod lists;
mod maps;
mod postfixes;
mod prefixes;

#[test]
fn test_parse_lists() -> ParseResult<()> {
    let src = include_str!("./lists.air");
    test_parse(src, lists::expected)
}

#[test]
fn test_parse_maps() -> ParseResult<()> {
    let src = include_str!("./maps.air");
    test_parse(src, maps::expected)
}

#[test]
fn test_parse_prefixes() -> ParseResult<()> {
    let src = include_str!("./prefixes.air");
    test_parse(src, prefixes::expected)
}

#[test]
fn test_parse_infixes() -> ParseResult<()> {
    let src = include_str!("./infixes.air");
    test_parse(src, infixes::expected)
}

#[test]
fn test_parse_postfixes() -> ParseResult<()> {
    let src = include_str!("./postfixes.air");
    test_parse(src, postfixes::expected)
}

fn test_parse(src: &str, expected: impl Fn() -> Repr) -> ParseResult<()> {
    let values = parse(src)?;
    assert_eq!(values, expected());
    Ok(())
}

#[test]
fn test_lists_to_string() -> ParseResult<()> {
    let src = include_str!("./lists.air");
    test_repr_to_string(src)
}

#[test]
fn test_maps_to_string() -> ParseResult<()> {
    let src = include_str!("./maps.air");
    test_repr_to_string(src)
}

#[test]
fn test_prefixes_to_string() -> ParseResult<()> {
    let src = include_str!("./prefixes.air");
    test_repr_to_string(src)
}

#[test]
fn test_infixes_to_string() -> ParseResult<()> {
    let src = include_str!("./infixes.air");
    test_repr_to_string(src)
}

#[test]
fn test_postfixes_to_string() -> ParseResult<()> {
    let src = include_str!("./postfixes.air");
    test_repr_to_string(src)
}

fn test_repr_to_string(src: &str) -> ParseResult<()> {
    let values = parse(src)?;
    let string = values.to_string();
    let new_values = parse(&string)?;
    assert_eq!(values, new_values);
    Ok(())
}

#[test]
fn test_parse_error_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ParseError>();
}

#[test]
fn test_parse_error_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<ParseError>();
}
