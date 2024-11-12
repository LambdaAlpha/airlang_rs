use std::error::Error;

use airlang::{
    AirCell,
    Int,
    Text,
    Val,
    parse,
};

use crate::init_ctx;

#[test]
fn test_build_import() -> Result<(), Box<dyn Error>> {
    let src = generate_import("/src/test/build_import/case_1/main.air");
    let src = parse(&src)?;
    let mut air = AirCell::default();
    init_ctx(air.ctx_mut());
    let output = air.interpret(src);
    assert_eq!(output, Val::Int(Int::from(5).into()));
    Ok(())
}

fn generate_import(path: &str) -> String {
    let mut src = Text::from("build.import \"");
    src.push_str_escaped(env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
