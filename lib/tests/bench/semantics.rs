use std::error::Error;

use airlang::cfg::comp::BaseCompCfg;
use airlang::cfg::prelude;
use airlang::semantics::core::Eval;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Int;

#[test]
fn test_interpret() -> Result<(), Box<dyn Error>> {
    let mut cfg = BaseCompCfg::generate();
    let mut ctx = prelude(&mut cfg);
    let s = include_str!("../../benches/semantics/interpret.air");
    let src_val: Val = s.parse()?;
    let output = Eval.call(&mut cfg, Ctx::new_mut(&mut ctx), src_val);
    let expected = Val::Int(Int::from(3267).into());
    assert_eq!(output, expected);
    Ok(())
}
