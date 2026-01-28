use std::error::Error;

use airlang::syntax::repr::Repr;

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/parse.air");
    let _: Repr = s.parse()?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/generate.air");
    let repr: Repr = s.parse()?;
    let str = format!("{repr}");
    let new_repr = str.parse()?;
    assert_eq!(repr, new_repr);
    Ok(())
}
