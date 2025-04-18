use std::{
    error::Error,
    str::FromStr,
};

use num_bigint::BigInt;
use num_traits::Num;

use crate::{
    Either,
    abstract1::Abstract,
    bit::Bit,
    int::Int,
    map::Map,
    number::Number,
    symbol::Symbol,
    syntax::{
        ChangeRepr,
        GenerateRepr,
        generate_compact,
        generate_pretty,
        generate_symbol,
        parse,
        repr::{
            Repr,
            call::CallRepr,
            equiv::EquivRepr,
            inverse::InverseRepr,
            pair::PairRepr,
            reify::ReifyRepr,
        },
    },
    test::parse_test_file,
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

mod either;

mod change;

mod call;

mod reify;

mod equiv;

mod inverse;

mod generate;

mod abstract1;

mod list;

mod map;

mod space;

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

fn this(repr: Repr) -> Repr {
    Repr::Either(Box::new(Either::This(repr)))
}

fn that(repr: Repr) -> Repr {
    Repr::Either(Box::new(Either::That(repr)))
}

fn change(from: Repr, to: Repr) -> Repr {
    Repr::Change(Box::new(ChangeRepr::new(from, to)))
}

fn call(func: Repr, input: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(func, input)))
}

fn reify(func: Repr) -> Repr {
    Repr::Reify(Box::new(ReifyRepr::new(func)))
}

fn equiv(func: Repr) -> Repr {
    Repr::Equiv(Box::new(EquivRepr::new(func)))
}

fn inverse(func: Repr) -> Repr {
    Repr::Inverse(Box::new(InverseRepr::new(func)))
}

fn generate(func: Repr) -> Repr {
    Repr::Generate(Box::new(GenerateRepr::new(func)))
}

fn abstract1(value: Repr) -> Repr {
    Repr::Abstract(Box::new(Abstract::new(value)))
}

fn list(v: Vec<Repr>) -> Repr {
    Repr::List(v.into())
}

fn map(v: Vec<(Repr, Repr)>) -> Repr {
    Repr::Map(Map::from_iter(v))
}

fn tag_pair(tag: &str, v: Vec<Repr>) -> Repr {
    let first = Repr::Symbol(Symbol::from_str(tag));
    let second = Repr::List(v.into());
    Repr::Pair(Box::new(PairRepr::new(first, second)))
}

fn tag_change(tag: &str, v: Vec<Repr>) -> Repr {
    let func = Repr::Symbol(Symbol::from_str(tag));
    let output = Repr::List(v.into());
    Repr::Change(Box::new(ChangeRepr::new(func, output)))
}

fn tag_call(tag: &str, v: Vec<Repr>) -> Repr {
    let func = Repr::Symbol(Symbol::from_str(tag));
    let input = Repr::List(v.into());
    Repr::Call(Box::new(CallRepr::new(func, input)))
}

fn infix_pair(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(middle, Repr::Pair(Box::new(PairRepr::new(left, right))))))
}

fn infix_change(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Change(Box::new(ChangeRepr::new(
        middle,
        Repr::Pair(Box::new(PairRepr::new(left, right))),
    )))
}

fn infix_call(left: Repr, middle: Repr, right: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(middle, Repr::Pair(Box::new(PairRepr::new(left, right))))))
}

fn test_parse(
    src: &str, file_name: &str, expected: impl FnOnce() -> Vec<Repr>,
) -> Result<(), Box<dyn Error>> {
    let mut expected = expected().into_iter();
    let cases = parse_test_file::<2>(src, file_name);
    if expected.len() != cases.len() {
        return Err(format!("file {file_name} length not equal").into());
    }
    for [title, s] in cases {
        let expected_repr = expected.next().unwrap();
        let real_repr = parse(s).map_err(|e| {
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
    let gen_fmt_list = [generate_compact, generate_pretty, generate_symbol];
    for [title, s] in parse_test_file::<2>(src, file_name) {
        let repr = parse(s).map_err(|e| {
            eprintln!("file {file_name} case ({title}) src({s}): parse failed\n{e}");
            e
        })?;
        for gen1 in gen_fmt_list {
            let string = gen1(&repr);
            let new_repr = parse(&string).map_err(|e| {
                eprintln!(
                    "file {file_name} case ({title}) src({s}): parse error with generated string ({string})!\n{e}"
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
    for [title, s] in parse_test_file::<2>(src, file_name) {
        assert!(parse(s).is_err(), "file {file_name} case ({title}) src({s}): shouldn't parse");
    }
    Ok(())
}

fn test_parse_bad(src: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    for [title, i1, i2] in parse_test_file::<3>(src, file_name) {
        let i1 = parse(i1).map_err(|e| {
            eprintln!("file {file_name} case ({title}): ({i1}) parse failed\n{e}");
            e
        })?;
        let i2 = parse(i2).map_err(|e| {
            eprintln!("file {file_name} case ({title}): ({i2}) parse failed\n{e}");
            e
        })?;
        assert_eq!(i1, i2, "file {file_name} case ({title}): expect({i2}) != real({i1})");
    }
    Ok(())
}

#[test]
fn test_space() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/space.air"), "test/space.air", space::expected)
}

#[test]
fn test_parse_unit() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/unit.air"), "test/unit.air", unit::expected)
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
    test_parse(include_str!("test/symbol.air"), "test/symbol.air", symbol::expected)
}

#[test]
fn test_generate_symbol() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/symbol.air"), "test/symbol.air")
}

#[test]
fn test_parse_text() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/text.air"), "test/text.air", text::expected)
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
    test_parse(include_str!("test/number.air"), "test/number.air", number::expected)
}

#[test]
fn test_generate_number() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/number.air"), "test/number.air")
}

#[test]
fn test_parse_byte() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/byte.air"), "test/byte.air", byte::expected)
}

#[test]
fn test_generate_byte() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/byte.air"), "test/byte.air")
}

#[test]
fn test_parse_pair() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/pair.air"), "test/pair.air", pair::expected)
}

#[test]
fn test_generate_pair() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/pair.air"), "test/pair.air")
}

#[test]
fn test_parse_either() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/either.air"), "test/either.air", either::expected)
}

#[test]
fn test_generate_either() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/either.air"), "test/either.air")
}

#[test]
fn test_parse_change() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/change.air"), "test/change.air", change::expected)
}

#[test]
fn test_generate_change() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/change.air"), "test/change.air")
}

#[test]
fn test_parse_call() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/call.air"), "test/call.air", call::expected)
}

#[test]
fn test_generate_call() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/call.air"), "test/call.air")
}

#[test]
fn test_parse_reify() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/reify.air"), "test/reify.air", reify::expected)
}

#[test]
fn test_generate_reify() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/reify.air"), "test/reify.air")
}

#[test]
fn test_parse_equiv() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/equiv.air"), "test/equiv.air", equiv::expected)
}

#[test]
fn test_generate_equiv() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/equiv.air"), "test/equiv.air")
}

#[test]
fn test_parse_inverse() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/inverse.air"), "test/inverse.air", inverse::expected)
}

#[test]
fn test_generate_inverse() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/inverse.air"), "test/inverse.air")
}

#[test]
fn test_parse_generate() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/generate.air"), "test/generate.air", generate::expected)
}

#[test]
fn test_generate_generate() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/generate.air"), "test/generate.air")
}

#[test]
fn test_parse_abstract() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/abstract.air"), "test/abstract.air", abstract1::expected)
}

#[test]
fn test_generate_abstract() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/abstract.air"), "test/abstract.air")
}

#[test]
fn test_parse_scope() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/scope.air"), "test/scope.air", scope::expected)
}

#[test]
fn test_generate_scope() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/scope.air"), "test/scope.air")
}

#[test]
fn test_parse_list() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/list.air"), "test/list.air", list::expected)
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
