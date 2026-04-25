use std::error::Error;

use airlang::semantics::val::Val;
use airlang::syntax::repr::Repr;

#[test]
fn test_parse_repr() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/syntax/parse.air");
    let _: Repr = s.parse()?;
    Ok(())
}

#[test]
fn test_parse_val() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/syntax/parse.air");
    let _: Val = s.parse()?;
    Ok(())
}

#[test]
fn test_generate_repr() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/syntax/generate.air");
    let repr: Repr = s.parse()?;
    let str = format!("{repr}");
    let new_repr = str.parse()?;
    assert_eq!(repr, new_repr);
    Ok(())
}

#[test]
fn test_generate_val() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../benches/syntax/generate.air");
    let repr: Val = s.parse()?;
    let str = format!("{repr:#}");
    let new_repr: Val = str.parse()?;
    assert_eq!(repr, new_repr);
    Ok(())
}
