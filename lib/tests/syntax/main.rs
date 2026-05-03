use std::error::Error;

use airlang::syntax::repr::CellRepr;
use airlang::syntax::repr::PairRepr;
use airlang::syntax::repr::QuoteRepr;
use airlang::syntax::repr::Repr;
use airlang::type_::Bit;
use airlang::type_::Call;
use airlang::type_::Decimal;
use airlang::type_::Int;
use airlang::type_::Key;
use airlang::type_::Map;
use airlang::type_::Pair;
use airlang::type_::Solve;
use airlang::type_::Text;
use airlang::type_::Unit;
use airlang_dev::test::parse_file;

fn unit() -> Repr {
    Repr::Unit(Unit)
}

fn bit(b: bool) -> Repr {
    Repr::Bit(Bit::from(b))
}

fn key(s: &str) -> Repr {
    Repr::Key(Key::from_str_unchecked(s))
}

fn text(s: &str) -> Repr {
    Repr::Text(Text::from(s))
}

fn int(s: &str, radix: u8) -> Repr {
    let i = Int::from_str_radix(s, radix as u32).unwrap();
    Repr::Int(i)
}

fn decimal(sign: bool, exp: i64, significand: &str) -> Repr {
    let i = Int::from_str_radix(significand, 10).unwrap();
    let i = if sign { i } else { -i };
    let scale = significand.len() as i64 - 1 - exp;
    let decimal = Decimal::from_int_scale(i, scale);
    Repr::Decimal(decimal)
}

fn byte(b: Vec<u8>) -> Repr {
    Repr::Byte(b.into())
}

fn cell(value: Repr) -> Repr {
    Repr::Cell(Box::new(CellRepr::new(value)))
}

fn pair(left: Repr, right: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(left, right)))
}

fn list(v: Vec<Repr>) -> Repr {
    Repr::List(v.into())
}

fn map(v: Vec<(&str, Repr)>) -> Repr {
    Repr::Map(Map::from_iter(v.into_iter().map(|(k, v)| (Key::from_str_unchecked(k), v))))
}

fn quote(value: Repr) -> Repr {
    Repr::Quote(Box::new(QuoteRepr::new(value)))
}

fn call(func: Repr, input: Repr) -> Repr {
    let call = Call { func, input };
    Repr::Call(Box::new(call))
}

fn infix_call(left: Repr, middle: Repr, right: Repr) -> Repr {
    let call = Call { func: middle, input: Repr::Pair(Box::new(Pair::new(left, right))) };
    Repr::Call(Box::new(call))
}

fn solve(func: Repr, output: Repr) -> Repr {
    let solve = Solve { func, output };
    Repr::Solve(Box::new(solve))
}

fn test_parse(
    src: &str, file_name: &str, expected: impl FnOnce() -> Vec<Repr>,
) -> Result<(), Box<dyn Error>> {
    let mut expected = expected().into_iter();
    let cases = parse_file::<2>(src, file_name);
    if expected.len() != cases.len() {
        return Err(format!("file {file_name} length not equal").into());
    }
    for [title, s] in cases {
        let expected_repr = expected.next().unwrap();
        let real_repr: Repr = s.parse().map_err(|e| {
            eprintln!("file {file_name} case ({title}) src({s}): parse failed\n{e}");
            e
        })?;
        assert_eq!(
            real_repr, expected_repr,
            "file {file_name} case ({title}) src({s}): expect({expected_repr}) != real({real_repr})"
        );
    }
    Ok(())
}

fn test_generate(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    for [title, s] in parse_file::<2>(src, file_name) {
        let repr: Repr = s.parse().map_err(|e| {
            eprintln!("file {file_name} case ({title}) src({s}): parse failed\n{e}");
            e
        })?;
        let fmt_list =
            [format!("{repr}"), format!("{repr:#}"), format!("{repr:?}"), format!("{repr:#?}")];
        for repr_str in fmt_list {
            let new_repr = repr_str.parse().map_err(|e| {
                eprintln!(
                    "file {file_name} case ({title}) src({s}): parse error with generated string ({repr_str})!\n{e}"
                );
                e
            })?;
            assert_eq!(
                repr, new_repr,
                "file {file_name} case ({title}) src({s}): original({repr}) != re-parsed({new_repr})"
            );
        }
    }
    Ok(())
}

fn test_parse_illegal(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    for [title, s] in parse_file::<2>(src, file_name) {
        assert!(
            s.parse::<Repr>().is_err(),
            "file {file_name} case ({title}) src({s}): shouldn't parse"
        );
    }
    Ok(())
}

fn test_parse_bad(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    for [title, i1, i2] in parse_file::<3>(src, file_name) {
        let i1 = i1.parse::<Repr>().map_err(|e| {
            eprintln!("file {file_name} case ({title}): ({i1}) parse failed\n{e}");
            e
        })?;
        let i2 = i2.parse::<Repr>().map_err(|e| {
            eprintln!("file {file_name} case ({title}): ({i2}) parse failed\n{e}");
            e
        })?;
        assert_eq!(i1, i2, "file {file_name} case ({title}): expect({i2}) != real({i1})");
    }
    Ok(())
}

macro_rules! test_syntax {
    ($($name:ident)*) => {
        airlang_dev::macro_::paste!{
            $(#[test]
            fn [< test_parse_ $name >] () -> Result<(), Box<dyn Error>> {
                test_parse(include_str!(concat!(stringify!($name), ".air")), concat!("syntax/", stringify!($name), ".air"), $name::expected)
            }

            #[test]
            fn [< test_generate_ $name >] () -> Result<(), Box<dyn Error>> {
                test_generate(include_str!(concat!(stringify!($name), ".air")), concat!("syntax/", stringify!($name), ".air"))
            })*
        }
    };
}

test_syntax!(unit bit key text integer decimal byte cell pair list map quote call solve);
test_syntax!(scope);

#[test]
fn test_space() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("space.air"), "syntax/space.air", space::expected)
}

#[test]
fn test_parse_illegal_example() -> Result<(), Box<dyn Error>> {
    test_parse_illegal(include_str!("illegal.air"), "syntax/illegal.air")
}

#[test]
fn test_parse_bad_example() -> Result<(), Box<dyn Error>> {
    test_parse_bad(include_str!("bad.air"), "syntax/bad.air")
}

#[test]
fn test_doc() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("doc.air"), "syntax/doc.air", doc::expected)
}

mod unit;

mod bit;

mod key;

mod text;

mod integer;

mod decimal;

mod byte;

mod cell;

mod quote;

mod pair;

mod list;

mod map;

mod call;

mod solve;

mod space;

mod scope;

mod doc;
