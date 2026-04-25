use airlang::semantics::val::Val;
use airlang_dev::macro_::paste;
use airlang_dev::test::test_eval;

macro_rules! test {
    ($($name:ident)*) => {
        $(paste!{
            #[test]
            fn [< test_ $name >] () -> Result<(), Box<dyn ::std::error::Error>> {
                test_eval(include_str!(concat!(stringify!($name), ".air")), concat!("semantics/", stringify!($name), ".air"))
            }
        })*
    };
}

test!(unit bit key text integer decimal byte cell pair list map);
test!(quote call solve link config function);
test!(context control value error fact language);
test!(core doc debug);

#[test]
fn test_val_size() {
    let size = size_of::<Val>();
    assert_eq!(size, 24);
}
