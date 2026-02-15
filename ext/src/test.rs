use std::error::Error;
use std::fmt::Write;

use airlang::cfg::CoreCfg;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Bit;
use airlang::type_::Cell;
use airlang::type_::Int;
use airlang::type_::Text;
use airlang_dev::init_logger;

use crate::cfg2::StdCfg2;

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
    let mut cfg = StdCfg2::generate();
    let mut ctx = CoreCfg::prelude(&mut cfg, "test_build_load").unwrap();
    let output = Eval.call(&mut cfg, &mut ctx, src);
    assert_eq!(output, expect);
    Ok(())
}

// AIR CODE
fn generate_load(path: &str) -> String {
    let mut path_prefix = String::new();
    write!(&mut path_prefix, "{:-}", Text::from(env!("CARGO_MANIFEST_DIR"))).unwrap();
    format!(
        "_ do [\
            .load set _ import _build.load,\
            _ load \"{path_prefix}{path}\"\
        ]"
    )
}
