use {
    crate::semantics::{
        parse,
        val::Val,
    },
    std::{
        error::Error,
        mem::size_of,
    },
};

fn test_interpret(input: &str) -> Result<(), Box<dyn Error>> {
    let mut interpreter = crate::semantics::Interpreter::new();
    let tests = input.split("# ===");

    for test in tests {
        let (i, o) = test.split_once("# ---").unwrap();
        let src = parse(i).map_err(|e| {
            eprintln!("{e}");
            e
        })?;
        let ret = interpreter.interpret(src);
        let ret_expected = parse(o).map_err(|e| {
            eprintln!("{e}");
            e
        })?;
        assert_eq!(
            ret, ret_expected,
            "input: {:}, real output: {:?} != expected output: {:}",
            i, ret, o
        );
        interpreter.reset();
    }
    Ok(())
}

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert!(size <= 40);
}

#[test]
fn test_core() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/core.air"))
}

#[test]
fn test_syntax() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/syntax.air"))
}

#[test]
fn test_types() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/type.air"))
}

#[test]
fn test_ctx() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/ctx.air"))
}

#[test]
fn test_ctrl() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/ctrl.air"))
}

#[test]
fn test_eval() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/eval.air"))
}

#[test]
fn test_func() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/func.air"))
}

#[test]
fn test_call() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/call.air"))
}

#[test]
fn test_logic() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/logic.air"))
}

#[test]
fn test_bool() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/bool.air"))
}

#[test]
fn test_int() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/int.air"))
}

#[test]
fn test_str() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/str.air"))
}

#[test]
fn test_pair() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/pair.air"))
}

#[test]
fn test_list() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/list.air"))
}

#[test]
fn test_map() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/map.air"))
}
