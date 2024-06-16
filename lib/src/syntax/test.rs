use std::{
    error::Error,
    str::FromStr,
};

use num_bigint::BigInt;
use num_traits::Num;

use crate::{
    bool::Bool,
    int::Int,
    map::Map,
    number::Number,
    string::Str,
    symbol::Symbol,
    syntax::{
        parse,
        repr::{
            annotate::AnnotateRepr,
            ask::AskRepr,
            call::CallRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
            Repr,
        },
    },
    unit::Unit,
};

mod booleans;
mod bytes;
mod calls;
mod annotates;
mod numbers;
mod infixes;
mod ints;
mod lists;
mod maps;
mod pairs;
mod asks;
mod strings;
mod symbols;
mod units;
mod wraps;

fn unit() -> Repr {
    Repr::Unit(Unit)
}

fn bool(b: bool) -> Repr {
    Repr::Bool(Bool::new(b))
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

fn bytes(b: Vec<u8>) -> Repr {
    Repr::Bytes(b.into())
}

fn symbol(s: &str) -> Repr {
    Repr::Symbol(Symbol::from_str(s))
}

fn string(s: &str) -> Repr {
    Repr::String(Str::from(s))
}

fn pair(first: Repr, second: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(first, second)))
}

fn call(func: Repr, input: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(func, input)))
}

fn ask(func: Repr, output: Repr) -> Repr {
    Repr::Ask(Box::new(AskRepr::new(func, output)))
}

fn annotate(note: Repr, value: Repr) -> Repr {
    Repr::Annotate(Box::new(AnnotateRepr::new(note, value)))
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

fn infix(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(
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

    for (s, expected_repr) in sources.zip(expected()) {
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
fn test_parse_units() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/units.air"),
        "test/units.air",
        units::expected,
    )
}

#[test]
fn test_generate_units() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/units.air"), "test/units.air")
}

#[test]
fn test_parse_booleans() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/booleans.air"),
        "test/booleans.air",
        booleans::expected,
    )
}

#[test]
fn test_generate_booleans() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/booleans.air"), "test/booleans.air")
}

#[test]
fn test_parse_ints() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/ints.air"),
        "test/ints.air",
        ints::expected,
    )
}

#[test]
fn test_generate_ints() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/ints.air"), "test/ints.air")
}

#[test]
fn test_parse_numbers() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/numbers.air"),
        "test/numbers.air",
        numbers::expected,
    )
}

#[test]
fn test_generate_numbers() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/numbers.air"), "test/numbers.air")
}

#[test]
fn test_parse_bytes() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/bytes.air"),
        "test/bytes.air",
        bytes::expected,
    )
}

#[test]
fn test_generate_bytes() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/bytes.air"), "test/bytes.air")
}

#[test]
fn test_parse_symbols() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/symbols.air"),
        "test/symbols.air",
        symbols::expected,
    )
}

#[test]
fn test_generate_symbols() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/symbols.air"), "test/symbols.air")
}

#[test]
fn test_parse_strings() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/strings.air"),
        "test/strings.air",
        strings::expected,
    )
}

#[test]
fn test_generate_strings() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/strings.air"), "test/strings.air")
}
#[test]
fn test_parse_pairs() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/pairs.air"),
        "test/pairs.air",
        pairs::expected,
    )
}

#[test]
fn test_generate_pairs() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/pairs.air"), "test/pairs.air")
}

#[test]
fn test_parse_calls() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/calls.air"),
        "test/calls.air",
        calls::expected,
    )
}

#[test]
fn test_generate_calls() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/calls.air"), "test/calls.air")
}

#[test]
fn test_parse_asks() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/asks.air"),
        "test/asks.air",
        asks::expected,
    )
}

#[test]
fn test_generate_asks() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/asks.air"), "test/asks.air")
}

#[test]
fn test_parse_wraps() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/wraps.air"),
        "test/wraps.air",
        wraps::expected,
    )
}

#[test]
fn test_generate_wraps() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/wraps.air"), "test/wraps.air")
}

#[test]
fn test_parse_lists() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/lists.air"),
        "test/lists.air",
        lists::expected,
    )
}

#[test]
fn test_generate_lists() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/lists.air"), "test/lists.air")
}

#[test]
fn test_parse_maps() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/maps.air"),
        "test/maps.air",
        maps::expected,
    )
}

#[test]
fn test_generate_maps() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/maps.air"), "test/maps.air")
}

#[test]
fn test_parse_annotates() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/annotates.air"),
        "test/annotates.air",
        annotates::expected,
    )
}

#[test]
fn test_generate_annotates() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/annotates.air"), "test/annotates.air")
}

#[test]
fn test_parse_infixes() -> Result<(), Box<dyn Error>> {
    test_parse(
        include_str!("test/infixes.air"),
        "test/infixes.air",
        infixes::expected,
    )
}

#[test]
fn test_generate_infixes() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/infixes.air"), "test/infixes.air")
}

#[test]
fn test_parse_illegal_examples() -> Result<(), Box<dyn Error>> {
    test_parse_illegal(include_str!("test/illegals.air"), "test/illegals.air")
}

#[test]
fn test_parse_bad_examples() -> Result<(), Box<dyn Error>> {
    test_parse_bad(include_str!("test/bad.air"), "test/bad.air")
}
