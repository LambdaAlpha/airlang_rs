use std::error::Error;

use airlang_dev::init_logger;
use log::trace;

use crate::Air;
use crate::init_prelude;
use crate::init_solver;
use crate::prelude::CorePrelude;
use crate::semantics::val::Val;
use crate::solver::core_solver;
use crate::syntax::parse;

const MAIN_DELIMITER: &str = "\n=====\n";
const SUB_DELIMITER: &str = "\n-----\n";

pub(crate) fn parse_test_file<'a, const N: usize>(
    input: &'a str, file_name: &str,
) -> Vec<[&'a str; N]> {
    let mut cases = Vec::with_capacity(100);
    if input.is_empty() {
        return cases;
    }
    let cases_str = input.split(MAIN_DELIMITER);
    for case_str in cases_str {
        let split_err = format!("file {file_name} case ({case_str}): invalid test case format");
        let case: Vec<&str> = case_str.split(SUB_DELIMITER).collect();
        let case: [&str; N] = case.try_into().expect(&split_err);
        cases.push(case);
    }
    cases
}

fn test(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    init_logger();
    init_prelude(Box::new(CorePrelude::default()));
    init_solver(core_solver());
    test_interpret(Air::default(), input, file_name)
}

fn test_interpret(air: Air, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let backup = air;
    for [title, i, o] in parse_test_file::<3>(input, file_name) {
        let mut air = backup.clone();
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name} case ({title}): input ({i}) parse failed\n{e}");
            e
        })?;
        trace!("file {file_name} case ({title})");
        let ret = air.interpret(src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name} case ({title}): output ({o}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "file {file_name} case({title}) input({i}): expect({o}) != real({ret:#?})\n\
            current context: {air:#?}",
        );
    }
    Ok(())
}

#[test]
fn test_unit() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/unit.air"), "test/unit.air")
}

#[test]
fn test_bit() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/bit.air"), "test/bit.air")
}

#[test]
fn test_symbol() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/symbol.air"), "test/symbol.air")
}

#[test]
fn test_text() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/text.air"), "test/text.air")
}

#[test]
fn test_int() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/int.air"), "test/int.air")
}

#[test]
fn test_number() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/number.air"), "test/number.air")
}

#[test]
fn test_byte() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/byte.air"), "test/byte.air")
}

#[test]
fn test_pair() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/pair.air"), "test/pair.air")
}

#[test]
fn test_call() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/call.air"), "test/call.air")
}

#[test]
fn test_list() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/list.air"), "test/list.air")
}

#[test]
fn test_map() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/map.air"), "test/map.air")
}

#[test]
fn test_ctx() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/ctx.air"), "test/ctx.air")
}

#[test]
fn test_func() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/func.air"), "test/func.air")
}

#[test]
fn test_debug() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/debug.air"), "test/debug.air")
}

#[test]
fn test_doc() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/doc.air"), "test/doc.air")
}

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert_eq!(size, 24);
}

#[test]
fn test_core() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/core.air"), "test/core.air")
}

#[test]
fn test_solve() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/solve.air"), "test/solve.air")
}

#[test]
fn test_syntax() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/syntax.air"), "test/syntax.air")
}

#[test]
fn test_value() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/value.air"), "test/value.air")
}

#[test]
fn test_ctrl() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/ctrl.air"), "test/ctrl.air")
}

#[test]
fn test_mode() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/mode.air"), "test/mode.air")
}
