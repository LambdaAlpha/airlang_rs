use std::error::Error;

use airlang::{
    AirCell,
    Symbol,
    Text,
    VarAccess,
    parse,
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

fn generate_air_with_main() -> Result<AirCell, Box<dyn Error>> {
    let src = generate_import("/main/main.air");
    let src = parse(&src)?;
    let mut air = AirCell::default();
    init_ctx(air.ctx_mut());
    let main = air.interpret(src);
    let main_name = unsafe { Symbol::from_str_unchecked(MAIN_NAME) };
    air.ctx_mut().put(main_name, VarAccess::Const, main)?;
    Ok(air)
}

fn generate_import(path: &str) -> String {
    let mut src = Text::from("build.import \"");
    src.push_str_escaped(env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}
