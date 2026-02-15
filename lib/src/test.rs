use std::env;
use std::error::Error;

use airlang_dev::init_logger;
use log::error;
use log::trace;

use crate::cfg::CoreCfg;
use crate::cfg2::CoreCfg2;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::DynFunc;
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
    let mut cfg = CoreCfg2::generate();
    let ctx = CoreCfg::prelude(&mut cfg, "test").unwrap();
    test_interpret(cfg, ctx, input, file_name)
}

fn test_interpret(cfg: Cfg, ctx: Val, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let backup_cfg = cfg;
    let backup_ctx = ctx;
    for [title, i, o] in parse_test_file::<3>(input, file_name) {
        let src: Val = parse(i).map_err(|e| {
            eprintln!("file {file_name} case ({title}): input ({i}) parse failed\n{e}");
            e
        })?;
        trace!("file {file_name} case ({title})");
        let mut cfg = backup_cfg.clone();
        let mut ctx = backup_ctx.clone();
        let ret = Eval.call(&mut cfg, &mut ctx, src);
        log_abort(&cfg);
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
                ret, ret_expected,
                "file {file_name} case({title}) input({i}): expect({o}) != real({ret:#})\n\
                current ctx:\n{:#}\ncurrent cfg:\n{:#}",
                ctx, cfg
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

fn log_abort(cfg: &Cfg) {
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
fn test_integer() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/integer.air"), "test/integer.air")
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
fn test_config() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/config.air"), "test/config.air")
}

#[test]
fn test_function() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/function.air"), "test/function.air")
}

#[test]
fn test_context() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/context.air"), "test/context.air")
}

#[test]
fn test_control() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/control.air"), "test/control.air")
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
fn test_language() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/language.air"), "test/language.air")
}

#[test]
fn test_core() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/core.air"), "test/core.air")
}

#[test]
fn test_doc() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/doc.air"), "test/doc.air")
}

#[test]
fn test_debug() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/debug.air"), "test/debug.air")
}

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert_eq!(size, 24);
}
