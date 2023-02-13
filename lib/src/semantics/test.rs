use {
    crate::{
        interpret,
        parse,
    },
    std::error::Error,
};

#[allow(dead_code)]
fn test_interpret(input: &str) -> Result<(), Box<dyn Error>> {
    let tests = input.split("# ===");

    for test in tests {
        let (i, o) = test.split_once("# ---").unwrap();
        let src = parse(dbg!(i))?;
        let ret = interpret(&src)?;
        let ret_expected = parse(dbg!(o))?;
        assert_eq!(ret, ret_expected);
    }
    Ok(())
}
