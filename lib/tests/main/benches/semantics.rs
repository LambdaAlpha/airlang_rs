use std::error::Error;

use airlang::Air;
use airlang::init_prelude;
use airlang::init_solver;
use airlang::prelude::CorePrelude;
use airlang::semantics::val::Val;
use airlang::solver::core_solver;
use airlang::syntax::generate_pretty;
use airlang::syntax::parse;
use airlang::type_::Int;

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    init_prelude(Box::new(CorePrelude::default()));
    init_solver(core_solver());
    let mut air = Air::default();
    let s = include_str!("../../../benches/main/interpret.air");
    let src_val = parse(s)?;
    let output = air.interpret(src_val);
    let expected = Val::Int(Int::from(3267).into());
    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/parse.air");
    parse::<Val>(s)?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/generate.air");
    let repr: Val = parse(s)?;
    let str = generate_pretty((&repr).try_into()?);
    let new_repr = parse(&str)?;
    assert_eq!(repr, new_repr);
    Ok(())
}
