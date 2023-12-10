use {
    airlang::{
        generate,
        initial_ctx,
        interpret,
        parse,
        types::Int,
        Val,
    },
    std::error::Error,
};

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut ctx = initial_ctx();
    let s = include_str!("../../benches/main/interpret.air");
    let src_val = parse(s)?;
    let output = interpret(&mut ctx, src_val);
    let expected = Val::Int(Int::from(6));
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
