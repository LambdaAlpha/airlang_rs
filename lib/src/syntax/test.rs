use std::{
    error::Error,
    str::FromStr,
};

use const_format::concatcp;
use num_bigint::BigInt;
use num_traits::Num;

use crate::{
    bit::Bit,
    int::Int,
    map::Map,
    number::Number,
    symbol::Symbol,
    syntax::{
        SEPARATOR,
        parse,
        repr::{
            Repr,
            abstract1::AbstractRepr,
            ask::AskRepr,
            call::CallRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
        },
    },
    text::Text,
    unit::Unit,
};

mod unit;

mod bit;

mod symbol;

mod text;

mod int;

mod number;

mod byte;

mod pair;

mod call;

mod abstract1;

mod ask;

mod list;

mod map;

mod scope;

fn unit() -> Repr {
    Repr::Unit(Unit)
}

fn bit(b: bool) -> Repr {
    Repr::Bit(Bit::new(b))
}

fn symbol(s: &str) -> Repr {
    Repr::Symbol(Symbol::from_str(s))
}

fn text(s: &str) -> Repr {
    Repr::Text(Text::from(s))
}

fn int(s: &str, radix: u8) -> Repr {
    let i = Int::new(BigInt::from_str_radix(s, radix as u32).unwrap());
    Repr::Int(i)
}

fn positive_decimal_int(s: &str) -> Repr {
    let i = Int::new(BigInt::from_str(s).unwrap());
    Repr::Int(i)
}

fn number(radix: u8, significand: &str, shift: usize, exp: &str) -> Repr {
    let i = BigInt::from_str_radix(significand, radix as u32).unwrap();
    let exp = BigInt::from_str(exp).unwrap();
    let num = Number::new(i, radix, exp - shift);
    Repr::Number(num)
}

fn byte(b: Vec<u8>) -> Repr {
    Repr::Byte(b.into())
}

fn pair(first: Repr, second: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(first, second)))
}

fn call(func: Repr, input: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(func, input)))
}

fn abstract1(func: Repr, value: Repr) -> Repr {
    Repr::Abstract(Box::new(AbstractRepr::new(func, value)))
}

fn ask(func: Repr, output: Repr) -> Repr {
    Repr::Ask(Box::new(AskRepr::new(func, output)))
}

fn no_compose(v: Vec<Repr>) -> Repr {
    let func = Repr::Symbol(Symbol::from_str(concatcp!(SEPARATOR)));
    let input = Repr::List(v.into());
    Repr::Abstract(Box::new(AbstractRepr::new(func, input)))
}

fn list(v: Vec<Repr>) -> Repr {
    Repr::List(v.into())
}

fn map(v: Vec<(Repr, Repr)>) -> Repr {
    Repr::Map(Map::from_iter(v))
}

fn call_list(root: Repr, leaves: Vec<Repr>) -> Repr {
    Repr::Call(Box::new(CallRepr::new(
        root,
        Repr::List(ListRepr::from(leaves)),
    )))
}

fn call_map(root: Repr, leaves: Vec<(Repr, Repr)>) -> Repr {
    Repr::Call(Box::new(CallRepr::new(
        root,
        Repr::Map(MapRepr::from_iter(leaves)),
    )))
}

fn infix_pair(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(
        middle,
        Repr::Pair(Box::new(PairRepr::new(left, right))),
    )))
}

fn infix(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(
        middle,
        Repr::Pair(Box::new(PairRepr::new(left, right))),
    )))
}

fn infix_abstract(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Abstract(Box::new(AbstractRepr::new(
        middle,
        Repr::Pair(Box::new(PairRepr::new(left, right))),
    )))
}

fn infix_ask(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Ask(Box::new(AskRepr::new(
        middle,
        Repr::Pair(Box::new(PairRepr::new(left, right))),
    )))
}

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

fn test_parse(
    src: &str,
    file_name: &str,
    expected: impl FnOnce() -> Vec<Repr>,
) -> Result<(), Box<dyn Error>> {
    let sources = src.split(MAIN_DELIMITER);
    let mut expected = expected().into_iter();

    for s in sources {
        let expected_repr = expected.next().expect("expected result should exist");
        let real_repr = parse(s).map_err(|e| {
            eprintln!("file {file_name}, case ({s}): parse failed\n{e}");
            e
        })?;
        assert_eq!(
            real_repr, expected_repr,
            "file {file_name}, case ({s}): expected: ({expected_repr}) != real: ({real_repr})"
        );
    }
    Ok(())
}

fn test_generate(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split(MAIN_DELIMITER);
    for s in sources {
        let repr = parse(s).map_err(|e| {
            eprintln!("file {file_name}, case ({s}): parse failed\n{e}");
            e
        })?;
        let string = repr.to_string();
        let new_repr = parse(&string).map_err(|e| {
            eprintln!(
                "file {file_name}, case ({s}): parse error with generated string ({string})!\n{e}"
            );
            e
        })?;
        assert_eq!(
            repr, new_repr,
            "file {file_name}, case ({s}): original: ({repr}) != re-parsed: ({new_repr})"
        );
    }
    Ok(())
}

fn test_parse_illegal(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split(MAIN_DELIMITER);
    for s in sources {
        assert!(
            parse(s).is_err(),
            "file {file_name}, case ({s}): shouldn't parse"
        );
    }
    Ok(())
}

fn test_parse_bad(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let tests = src.split(MAIN_DELIMITER);

    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i1, i2) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let i1 = parse(i1).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): ({i1}) parse failed\n{e}");
            e
        })?;
        let i2 = parse(i2).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): ({i2}) parse failed\n{e}");
            e
        })?;
        assert_eq!(
            i1, i2,
            "file {file_name}, case ({test}): expected: ({i2}) != real: ({i1})"
        );
    }
    Ok(())
}

#[test]
fn test_parse_unit() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/unit.air"),
        "test/unit.air",
        unit::expected,
    )
}

#[test]
fn test_generate_unit() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/unit.air"), "test/unit.air")
}

#[test]
fn test_parse_bit() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/bit.air"), "test/bit.air", bit::expected)
}

#[test]
fn test_generate_bit() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/bit.air"), "test/bit.air")
}

#[test]
fn test_parse_symbol() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/symbol.air"),
        "test/symbol.air",
        symbol::expected,
    )
}

#[test]
fn test_generate_symbol() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/symbol.air"), "test/symbol.air")
}

#[test]
fn test_parse_text() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/text.air"),
        "test/text.air",
        text::expected,
    )
}

#[test]
fn test_generate_text() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/text.air"), "test/text.air")
}

#[test]
fn test_parse_int() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/int.air"), "test/int.air", int::expected)
}

#[test]
fn test_generate_int() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/int.air"), "test/int.air")
}

#[test]
fn test_parse_number() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/number.air"),
        "test/number.air",
        number::expected,
    )
}

#[test]
fn test_generate_number() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/number.air"), "test/number.air")
}

#[test]
fn test_parse_byte() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/byte.air"),
        "test/byte.air",
        byte::expected,
    )
}

#[test]
fn test_generate_byte() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/byte.air"), "test/byte.air")
}

#[test]
fn test_parse_pair() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/pair.air"),
        "test/pair.air",
        pair::expected,
    )
}

#[test]
fn test_generate_pair() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/pair.air"), "test/pair.air")
}

#[test]
fn test_parse_call() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/call.air"),
        "test/call.air",
        call::expected,
    )
}

#[test]
fn test_generate_call() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/call.air"), "test/call.air")
}

#[test]
fn test_parse_abstract() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/abstract.air"),
        "test/abstract.air",
        abstract1::expected,
    )
}

#[test]
fn test_generate_abstract() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/abstract.air"), "test/abstract.air")
}

#[test]
fn test_parse_ask() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/ask.air"), "test/ask.air", ask::expected)
}

#[test]
fn test_generate_ask() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/ask.air"), "test/ask.air")
}

#[test]
fn test_parse_scope() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/scope.air"),
        "test/scope.air",
        scope::expected,
    )
}

#[test]
fn test_generate_scope() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/scope.air"), "test/scope.air")
}

#[test]
fn test_parse_list() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/list.air"),
        "test/list.air",
        list::expected,
    )
}

#[test]
fn test_generate_list() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/list.air"), "test/list.air")
}

#[test]
fn test_parse_map() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/map.air"), "test/map.air", map::expected)
}

#[test]
fn test_generate_map() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/map.air"), "test/map.air")
}

#[test]
fn test_parse_illegal_example() -> Result<(), Box<dyn Error>> {
    test_parse_illegal(include_str!("test/illegal.air"), "test/illegal.air")
}

#[test]
fn test_parse_bad_example() -> Result<(), Box<dyn Error>> {
    test_parse_bad(include_str!("test/bad.air"), "test/bad.air")
}
