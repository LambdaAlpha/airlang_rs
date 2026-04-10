use std::error::Error;

use airlang::cfg::prelude;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::syntax::FmtOptions;
use airlang::syntax::FmtRepr;
use airlang::type_::Bit;
use airlang::type_::Cell;
use airlang::type_::Int;
use airlang::type_::Text;
use airlang_dev::init_logger;

use crate::cfg::comp::ExtCompCfg;

#[test]
fn test_build_load_nest() -> Result<(), Box<dyn Error>> {
    let path = "/src/test/build_load/case_nest/main.air";
    let expect = Val::Cell(Cell::new(Val::Int(Int::from(5).into())).into());
    test_build_load(path, expect)
}

#[test]
fn test_build_load_bom() -> Result<(), Box<dyn Error>> {
    let path = "/src/test/build_load/case_bom/test_bom.air";
    let expect = Val::Cell(Cell::new(Val::Bit(Bit::true_())).into());
    test_build_load(path, expect)
}

fn test_build_load(path: &str, expect: Val) -> Result<(), Box<dyn Error>> {
    init_logger();
    let src = generate_load(path);
    let src: Val = src.parse()?;
    let mut cfg = ExtCompCfg::generate();
    let mut ctx = prelude(&mut cfg);
    let output = Eval.call(&mut cfg, &mut ctx, src);
    assert_eq!(output, expect);
    Ok(())
}

// AIR CODE
fn generate_load(path: &str) -> String {
    let path_str = format!("{}{}", env!("CARGO_MANIFEST_DIR"), path);
    let mut path_text = String::new();
    Text::from(path_str).fmt(FmtOptions::default(), &mut path_text).unwrap();
    format!(
        "_ do _[\
            _load set _ import .build.load,\
            _ load {path_text}\
        ]"
    )
}
