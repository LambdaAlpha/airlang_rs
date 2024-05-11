use std::error::Error;

use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    Int,
    MutableCtx,
    Val,
};

use crate::init_ctx;

#[test]
fn test_build_import() -> Result<(), Box<dyn Error>> {
    // pwd is ext/
    let src = "build.import \"src/test/build_import/case_1/main.air\"";
    let src = parse(src)?;
    let mut ctx = initial_ctx();
    let mut mut_ctx = MutableCtx::new(&mut ctx);
    init_ctx(mut_ctx.reborrow());
    let output = interpret_mutable(mut_ctx, src);
    assert_eq!(output, Val::Int(Int::from(5).into()));
    Ok(())
}
