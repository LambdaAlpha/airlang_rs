use std::error::Error;

use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    Ctx,
    Invariant,
    MutableCtx,
    Str,
    Symbol,
};
use airlang_ext::init_ctx;

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

const MAIN_NAME: &str = "main";

#[allow(unused)]
fn test_main(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Ok(());
    }
    let mut ctx = generate_ext_ctx_with_main()?;
    let backup = ctx.clone();
    let mut mut_ctx = MutableCtx::new(&mut ctx);

    let tests = input.split(MAIN_DELIMITER);
    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let ret = interpret_mutable(mut_ctx.reborrow(), src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {ctx:#?}",
        );
        ctx = backup.clone();
        mut_ctx = MutableCtx::new(&mut ctx);
    }
    Ok(())
}

fn generate_ext_ctx_with_main() -> Result<Ctx, Box<dyn Error>> {
    let src = generate_import("/main/main.air");
    let src = parse(&src)?;
    let mut ctx = initial_ctx();
    let mut mut_ctx = MutableCtx::new(&mut ctx);
    init_ctx(mut_ctx.reborrow());
    let main = interpret_mutable(mut_ctx.reborrow(), src);
    let main_name = unsafe { Symbol::from_str_unchecked(MAIN_NAME) };
    mut_ctx.put(main_name, Invariant::Const, main)?;
    Ok(ctx)
}

fn generate_import(path: &str) -> String {
    let mut src = Str::from("build.import \"");
    src.push_str_escaped(env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
