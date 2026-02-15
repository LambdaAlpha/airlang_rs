use std::error::Error;

use airlang::cfg::CoreCfg;
use airlang::cfg2::CoreCfg2;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Int;

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut cfg = CoreCfg2::generate();
    let mut ctx = CoreCfg::prelude(&mut cfg, "bench_interpret").unwrap();
    let s = include_str!("../../../benches/main/interpret.air");
    let src_val: Val = s.parse()?;
    let output = Eval.call(&mut cfg, &mut ctx, src_val);
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
