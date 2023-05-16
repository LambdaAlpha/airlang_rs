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
        let src = parse(i)?;
        let ret = interpreter.interpret(src);
        let ret_expected = parse(o)?;
        assert_eq!(
            ret, ret_expected,
            "input: {:?}, real output: {:?} != expected output: {:?}",
            i, ret, o
        );
        interpreter.reset();
    }
    Ok(())
}

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert!(size <= 40)
}

#[test]
fn test_core() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/core.air"))
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
fn test_bool() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/bool.air"))
}

#[test]
fn test_eval() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/eval.air"))
}

#[test]
fn test_int() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/int.air"))
}

#[test]
fn test_pair() -> Result<(), Box<dyn Error>> {
    test_interpret(include_str!("test/pair.air"))
}
