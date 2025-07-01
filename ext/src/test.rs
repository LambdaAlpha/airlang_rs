use std::error::Error;

use airlang::Air;
use airlang::init_prelude;
use airlang::init_solver;
use airlang::semantics::val::Val;
use airlang::syntax::escape_text;
use airlang::syntax::parse;
use airlang::type_::Int;
use airlang::type_::Text;

use crate::prelude::StdPrelude;
use crate::solver::std_solver;

#[test]
fn test_build_import() -> Result<(), Box<dyn Error>> {
    let src = generate_import("/src/test/build_import/case_1/main.air");
    let src = parse(&src)?;
    init_prelude(Box::new(StdPrelude::default()));
    init_solver(std_solver());
    let mut air = Air::default();
    let output = air.interpret(src);
    assert_eq!(output, Val::Int(Int::from(5).into()));
    Ok(())
}

fn generate_import(path: &str) -> String {
    let mut src = Text::from("; build.import \"");
    escape_text(&mut src, env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
