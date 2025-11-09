use std::error::Error;

use airlang::Air;
use airlang::semantics::val::Val;
use airlang::syntax::escape_text;
use airlang::syntax::parse;
use airlang::type_::Bit;
use airlang::type_::Int;
use airlang_dev::init_logger;

use crate::cfg2::StdCfg2;

#[test]
fn test_build_load_nest() -> Result<(), Box<dyn Error>> {
    let path = "/src/test/build_load/case_nest/main.air";
    let expect = Val::Int(Int::from(5).into());
    test_build_load(path, expect)
}

#[test]
fn test_build_load_bom() -> Result<(), Box<dyn Error>> {
    let path = "/src/test/build_load/case_bom/test_bom.air";
    let expect = Val::Bit(Bit::true_());
    test_build_load(path, expect)
}

fn test_build_load(path: &str, expect: Val) -> Result<(), Box<dyn Error>> {
    init_logger();
    let src = generate_load(path);
    let src = parse(&src)?;
    let mut air = Air::new(StdCfg2::generate()).unwrap();
    let output = air.interpret(src);
    assert_eq!(output, expect);
    Ok(())
}

fn generate_load(path: &str) -> String {
    let mut path_prefix = String::new();
    escape_text(&mut path_prefix, env!("CARGO_MANIFEST_DIR"));
    format!(
        "_ do [\
            load = _ import _build.load,\
            _ load \"{path_prefix}{path}\"\
        ]"
    )
}
