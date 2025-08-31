use std::error::Error;

use airlang::Air;
use airlang::semantics::val::Val;
use airlang::syntax::escape_text;
use airlang::syntax::parse;
use airlang::type_::Int;
use airlang::type_::Text;
use airlang_dev::init_logger;

use crate::cfg::StdCfg;

#[test]
fn test_build_load() -> Result<(), Box<dyn Error>> {
    init_logger();
    let src = generate_load("/src/test/build_load/case_1/main.air");
    let src = parse(&src)?;
    let mut air = Air::new(StdCfg::default().into());
    let output = air.interpret(src);
    assert_eq!(output, Val::Int(Int::from(5).into()));
    Ok(())
}

fn generate_load(path: &str) -> String {
    let mut src = Text::from("_ build.load \"");
    escape_text(&mut src, env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
