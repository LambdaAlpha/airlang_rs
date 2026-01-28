use std::env;
use std::error::Error;

use airlang_dev::init_logger;
use log::error;
use log::trace;

use crate::Air;
use crate::cfg2::CoreCfg2;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::syntax::parse;
use crate::type_::Key;

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
    let air = Air::new(CoreCfg2::generate()).unwrap();
    test_interpret(air, input, file_name)
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
        log_abort(air.cfg_mut());
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name} case ({title}): output ({o}) parse failed\n{e}");
            e
        })?;
        let show_env = if let Ok(show) = env::var("AIR_TEST_SHOW_CFG_CTX") {
            show == "1" || show == "on" || show == "true"
        } else {
            false
        };
        if show_env {
            assert_eq!(
                ret,
                ret_expected,
                "file {file_name} case({title}) input({i}): expect({o}) != real({ret:#})\n\
                current ctx:\n{:#}\ncurrent cfg:\n{:#}",
                air.ctx(),
                air.cfg()
            );
        } else {
            assert_eq!(
                ret, ret_expected,
                "file {file_name} case({title}) input({i}): expect({o}) != real({ret:#})",
            );
        }
    }
    Ok(())
}

fn log_abort(cfg: &mut Cfg) {
    if !cfg.is_aborted() {
        return;
    }
    let type_ = cfg.import(Key::from_str_unchecked(Cfg::ABORT_TYPE));
    let msg = cfg.import(Key::from_str_unchecked(Cfg::ABORT_MSG));
    match (type_, msg) {
        (Some(type_), Some(msg)) => error!("aborted by {type_}: {msg}"),
        (None, Some(msg)) => error!("aborted: {msg}"),
        (Some(type_), None) => error!("aborted by {type_}"),
        (None, None) => error!("aborted"),
    }
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
fn test_key() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/key.air"), "test/key.air")
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
fn test_decimal() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/decimal.air"), "test/decimal.air")
}

#[test]
fn test_byte() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/byte.air"), "test/byte.air")
}

#[test]
fn test_cell() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/cell.air"), "test/cell.air")
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
fn test_link() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/link.air"), "test/link.air")
}

#[test]
fn test_cfg() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/cfg.air"), "test/cfg.air")
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
fn test_ctx() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/ctx.air"), "test/ctx.air")
}

#[test]
fn test_ctrl() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/ctrl.air"), "test/ctrl.air")
}

#[test]
fn test_value() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/value.air"), "test/value.air")
}

#[test]
fn test_resource() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/resource.air"), "test/resource.air")
}

#[test]
fn test_error() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/error.air"), "test/error.air")
}

#[test]
fn test_lang() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/lang.air"), "test/lang.air")
}
