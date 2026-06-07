use std::error::Error;

use airlang_dev::test::test_eval;

#[test]
fn test_wasm() -> Result<(), Box<dyn Error>> {
    test_eval(include_str!("wasm.air"), "ext/wasm.air")
}

#[test]
fn test_build() -> Result<(), Box<dyn Error>> {
    test_eval(include_str!("build.air"), "ext/build.air")
}
