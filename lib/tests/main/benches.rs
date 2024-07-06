use std::error::Error;

use airlang::{
    generate,
    initial_ctx,
    interpret_mut,
    parse,
    Int,
    MutCtx,
    Val,
};

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut ctx = initial_ctx();
    let mut_ctx = MutCtx::new(&mut ctx);
    let s = include_str!("../../benches/main/interpret.air");
    let src_val = parse(s)?;
    let output = interpret_mut(mut_ctx, src_val);
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
