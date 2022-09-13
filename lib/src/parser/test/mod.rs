use crate::{parser::parse, val::Val};

use super::ParserError;

mod comments;
mod infixes;
mod lists;
mod maps;
mod prefixes;

#[test]
fn test_parse_comments() -> Result<(), ParserError> {
    let src = include_str!("./comments.air");
    test_parse(src, comments::expected)
}

#[test]
fn test_parse_lists() -> Result<(), ParserError> {
    let src = include_str!("./lists.air");
    test_parse(src, lists::expected)
}

#[test]
fn test_parse_maps() -> Result<(), ParserError> {
    let src = include_str!("./maps.air");
    test_parse(src, maps::expected)
}

#[test]
fn test_parse_infixes() -> Result<(), ParserError> {
    let src = include_str!("./infixes.air");
    test_parse(src, infixes::expected)
}

#[test]
fn test_parse_prefixes() -> Result<(), ParserError> {
    let src = include_str!("./prefixes.air");
    test_parse(src, prefixes::expected)
}

fn test_parse(src: &str, expected: impl Fn() -> Val) -> Result<(), ParserError> {
    let values = parse(src)?;
    assert_eq!(values, expected());
    Ok(())
}
