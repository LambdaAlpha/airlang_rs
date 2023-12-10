use {
    crate::{
        eval::{
            output::OutputBuilder,
            ValBuilder,
        },
        parse,
        set_call_extension,
        set_reverse_extension,
        val::Val,
    },
    std::{
        error::Error,
        mem::size_of,
    },
};

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

fn test_interpret(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Ok(());
    }
    let mut interpreter = crate::Interpreter::new();
    let tests = input.split(MAIN_DELIMITER);

    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let ret = interpreter.interpret(src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {interpreter:#?}",
        );
        interpreter.reset();
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
    assert!(size <= 40);
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
fn test_eval() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/eval.air"), "test/eval.air")
}

#[test]
fn test_logic() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/logic.air"), "test/logic.air")
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
fn test_reverse() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/reverse.air"), "test/reverse.air")
}

#[test]
fn test_prop() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/prop.air"), "test/prop.air")
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
fn test_float() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/float.air"), "test/float.air")
}

#[test]
fn test_bytes() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/bytes.air"), "test/bytes.air")
}

#[test]
fn test_str() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/str.air"), "test/str.air")
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
    set_call_extension(Box::new(|_ctx, func, input| {
        ValBuilder.from_call(func, input)
    }));
    set_reverse_extension(Box::new(|_ctx, func, output| {
        ValBuilder.from_reverse(func, output)
    }));
    test_interpret(include_str!("test/extension.air"), "test/extension.air")
}
