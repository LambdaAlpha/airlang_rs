use std::env;
use std::error::Error;

use airlang::cfg::error::ABORT_MSG;
use airlang::cfg::error::ABORT_TYPE;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use log::error;
use log::trace;

use crate::cfg::comp::DevCompCfg;
use crate::log::init_logger;

const MAIN_DELIMITER: &str = "\n=====\n";
const SUB_DELIMITER: &str = "\n-----\n";

pub fn parse_file<'a, const N: usize>(input: &'a str, file_name: &str) -> Vec<[&'a str; N]> {
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

pub fn test_eval(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    init_logger();
    let mut cfg = DevCompCfg::generate();
    let ctx = prelude(&mut cfg);
    run_test_eval(cfg, ctx, input, file_name)
}

fn run_test_eval(cfg: Cfg, ctx: Val, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let backup_cfg = cfg;
    let backup_ctx = ctx;
    for [title, i, o] in parse_file::<3>(input, file_name) {
        let src: Val = i.parse().map_err(|e| {
            eprintln!("file {file_name} case ({title}): input ({i}) parse failed\n{e}");
            e
        })?;
        trace!("file {file_name} case ({title})");
        let mut cfg = backup_cfg.clone();
        let mut ctx = backup_ctx.clone();
        let ret = Eval.call(&mut cfg, &mut ctx, src);
        log_abort(&cfg);
        let ret_expected = o.parse().map_err(|e| {
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

pub fn log_abort(cfg: &Cfg) {
    if !cfg.is_aborted() {
        return;
    }
    let type_ = cfg.import(Key::from_str_unchecked(ABORT_TYPE));
    let msg = cfg.import(Key::from_str_unchecked(ABORT_MSG));
    match (type_, msg) {
        (Some(type_), Some(msg)) => error!("aborted by {type_}: {msg}"),
        (None, Some(msg)) => error!("aborted: {msg}"),
        (Some(type_), None) => error!("aborted by {type_}"),
        (None, None) => error!("aborted"),
    }
}
