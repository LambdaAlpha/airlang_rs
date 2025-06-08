use std::error::Error;

use airlang::syntax::generate_compact;
use airlang::syntax::parse;
use airlang::syntax::repr::Repr;

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/parse.air");
    let _: Repr = parse(s)?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/generate.air");
    let repr: Repr = parse(s)?;
    let str = generate_compact((&repr).try_into().unwrap());
    let new_repr = parse(&str)?;
    assert_eq!(repr, new_repr);
    Ok(())
}
