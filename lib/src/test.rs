use std::{
    error::Error,
    mem::size_of,
    rc::Rc,
};

use crate::{
    Ctx,
    FuncVal,
    Mode,
    StaticFunc,
    StaticFuncVal,
    Symbol,
    ctx::{
        Invariant,
        mut1::MutCtx,
    },
    func::static1::StaticFn,
    initial_ctx,
    interpret_mut,
    parse,
    val::Val,
};

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

fn test_interpret(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let ctx = initial_ctx();
    test_interpret_with_ctx(ctx, input, file_name)
}

fn test_interpret_with_ctx(ctx: Ctx, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Ok(());
    }
    let backup = ctx;

    let tests = input.split(MAIN_DELIMITER);
    for test in tests {
        let mut ctx = backup.clone();
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let ret = interpret_mut(MutCtx::new(&mut ctx), src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {ctx:#?}",
        );
    }
    Ok(())
}

#[test]
fn test_debug() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/debug.air"), "test/debug.air")
}

#[test]
fn test_doc() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/doc.air"), "test/doc.air")
}

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert_eq!(size, 24);
}

#[test]
fn test_core() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/core.air"), "test/core.air")
}

#[test]
fn test_syntax() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/syntax.air"), "test/syntax.air")
}

#[test]
fn test_value() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/value.air"), "test/value.air")
}

#[test]
fn test_ctx() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/ctx.air"), "test/ctx.air")
}

#[test]
fn test_ctrl() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/ctrl.air"), "test/ctx.air")
}

#[test]
fn test_mode() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/mode.air"), "test/mode.air")
}

#[test]
fn test_func() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/func.air"), "test/func.air")
}

#[test]
fn test_call() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/call.air"), "test/call.air")
}

#[test]
fn test_ask() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/ask.air"), "test/ask.air")
}

#[test]
fn test_case() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/case.air"), "test/case.air")
}

#[test]
fn test_answer() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/answer.air"), "test/answer.air")
}

#[test]
fn test_symbol() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/symbol.air"), "test/symbol.air")
}

#[test]
fn test_unit() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/unit.air"), "test/unit.air")
}

#[test]
fn test_bool() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/bool.air"), "test/bool.air")
}

#[test]
fn test_int() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/int.air"), "test/int.air")
}

#[test]
fn test_number() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/number.air"), "test/number.air")
}

#[test]
fn test_byte() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/byte.air"), "test/byte.air")
}

#[test]
fn test_text() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/text.air"), "test/text.air")
}

#[test]
fn test_pair() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/pair.air"), "test/pair.air")
}

#[test]
fn test_list() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/list.air"), "test/list.air")
}

#[test]
fn test_map() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/map.air"), "test/map.air")
}

#[test]
fn test_extension() -> Result<(), Box<dyn Error>> {
    let mut ctx = initial_ctx();
    let mut_ctx = MutCtx::new(&mut ctx);
    let func_ext_name = Symbol::from_str("func_ext");
    mut_ctx.put(
        func_ext_name.clone(),
        Invariant::Const,
        Val::Func(FuncVal::Static(StaticFuncVal::from(StaticFunc::new(
            Mode::default(),
            Mode::default(),
            false,
            func_ext_name,
            Rc::new(FuncExt),
        )))),
    )?;
    test_interpret_with_ctx(
        ctx,
        include_str!("test/extension.air"),
        "test/extension.air",
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FuncExt;

impl StaticFn for FuncExt {
    fn call(&self, input: Val) -> Val {
        input
    }
}

#[test]
fn test_comment() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/comment.air"), "test/comment.air")
}
