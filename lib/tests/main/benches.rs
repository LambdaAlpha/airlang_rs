use std::error::Error;

use airlang::AirCell;
use airlang::Int;
use airlang::Val;
use airlang::generate;
use airlang::parse;

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut air = AirCell::default();
    let s = include_str!("../../benches/main/interpret.air");
    let src_val = parse(s)?;
    let output = air.interpret(src_val);
    let expected = Val::Int(Int::from(3267).into());
    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/main/parse.air");
    parse(s)?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/main/generate.air");
    let repr = parse(s)?;
    let str = generate(&repr)?;
    let new_repr = parse(&str)?;
    assert_eq!(repr, new_repr);
    Ok(())
}

mod syntax;
