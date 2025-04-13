use std::{
    error::Error,
    rc::Rc,
};

use crate::{
    AirCell,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    FuncMode,
    FuncVal,
    Symbol,
    ctx::map::VarAccess,
    func::free_static_prim::FreeStaticFn,
    parse,
    val::Val,
};

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
    test_interpret(AirCell::default(), input, file_name)
}

fn test_interpret(air: AirCell, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let backup = air;
    for [title, i, o] in parse_test_file::<3>(input, file_name) {
        let mut air = backup.clone();
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name} case ({title}): input ({i}) parse failed\n{e}");
            e
        })?;
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
fn test_change() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/change.air"), "test/change.air")
}

#[test]
fn test_call() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/call.air"), "test/call.air")
}

#[test]
fn test_reify() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/reify.air"), "test/reify.air")
}

#[test]
fn test_equiv() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/equiv.air"), "test/equiv.air")
}

#[test]
fn test_inverse() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/inverse.air"), "test/inverse.air")
}

#[test]
fn test_generate() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/generate.air"), "test/generate.air")
}

#[test]
fn test_abstract() -> Result<(), Box<dyn Error>> {
    test(include_str!("test/abstract.air"), "test/abstract.air")
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
fn test_extension() -> Result<(), Box<dyn Error>> {
    let mut air = AirCell::default();
    let func_ext_name = Symbol::from_str("func_ext");
    let func = FreeStaticPrimFunc::new_extension(
        func_ext_name.clone(),
        Rc::new(FuncExt),
        FuncMode::default(),
    );
    let func = Val::Func(FuncVal::FreeStaticPrim(FreeStaticPrimFuncVal::from(func)));
    air.ctx_mut().put(func_ext_name, VarAccess::Const, func)?;
    test_interpret(air, include_str!("test/extension.air"), "test/extension.air")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FuncExt;

impl FreeStaticFn for FuncExt {
    fn call(&self, input: Val) -> Val {
        input
    }
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
