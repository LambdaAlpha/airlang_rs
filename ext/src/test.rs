use std::error::Error;

use airlang::{
    initial_ctx,
    interpret_mut,
    parse,
    Int,
    MutCtx,
    Text,
    Val,
};

use crate::init_ctx;

#[test]
fn test_build_import() -> Result<(), Box<dyn Error>> {
    let src = generate_import("/src/test/build_import/case_1/main.air");
    let src = parse(&src)?;
    let mut ctx = initial_ctx();
    let mut mut_ctx = MutCtx::new(&mut ctx);
    init_ctx(mut_ctx.reborrow());
    let output = interpret_mut(mut_ctx, src);
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
