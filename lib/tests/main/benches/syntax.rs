use {
    airlang::syntax::{
        generate,
        parse,
    },
    std::error::Error,
};

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/parse.air");
    parse(s)?;
    Ok(())
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    let s = include_str!("../../../benches/main/generate.air");
    let repr = parse(s)?;
    let str = generate(&repr);
    let new_repr = parse(&str)?;
    assert_eq!(repr, new_repr);
    Ok(())
}
