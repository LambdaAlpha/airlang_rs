use std::error::Error;

use airlang::Air;
use airlang::init_solver;
use airlang::prelude::CorePrelude;
use airlang::semantics::ctx::Contract;
use airlang::solve::core_solver;
use airlang::syntax::escape_text;
use airlang::syntax::parse;
use airlang::type_::Symbol;
use airlang::type_::Text;
use airlang_dev::init_logger;

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

const MAIN_NAME: &str = "main";

#[expect(dead_code)]
fn test_main(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    init_logger();
    if input.is_empty() {
        return Ok(());
    }
    let mut air = generate_air_with_main()?;
    let backup = air.clone();

    let tests = input.split(MAIN_DELIMITER);
    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let ret = air.interpret(src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {air:#?}",
        );
        air = backup.clone();
    }
    Ok(())
}

fn generate_air_with_main() -> Result<Air, Box<dyn Error>> {
    let src = generate_import("/main/main.air");
    let src = parse(&src)?;
    init_solver(core_solver());
    let mut air = Air::new(CorePrelude::default().into());
    let main = air.interpret(src);
    let main_name = Symbol::from_str_unchecked(MAIN_NAME);
    air.ctx_mut().put(main_name, main, Contract::Final)?;
    Ok(air)
}

fn generate_import(path: &str) -> String {
    let mut src = Text::from("build.import \"");
    escape_text(&mut src, env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
