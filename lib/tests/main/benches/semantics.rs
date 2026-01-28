use std::error::Error;

use airlang::Air;
use airlang::cfg2::CoreCfg2;
use airlang::semantics::val::Val;
use airlang::type_::Int;

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut air = Air::new(CoreCfg2::generate()).unwrap();
    let s = include_str!("../../../benches/main/interpret.air");
    let src_val = s.parse()?;
    let output = air.interpret(src_val);
    let expected = Val::Int(Int::from(3267).into());
    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/parse.air");
    s.parse::<Val>()?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/generate.air");
    let repr: Val = s.parse()?;
    let str = format!("{repr:#}");
    let new_repr: Val = str.parse()?;
    assert_eq!(repr, new_repr);
    Ok(())
}
