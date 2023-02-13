use {
    crate::{
        grammar::parse,
        repr::{
            CallRepr,
            ListRepr,
            MapRepr,
            PairRepr,
            Repr,
        },
        types::{
            Bool,
            Float,
            Int,
            Letter,
            Map,
            Symbol,
            Unit,
        },
    },
    std::{
        error::Error,
        rc::Rc,
    },
};

mod booleans;
mod bytes;
mod calls;
mod comments;
mod floats;
mod infixes;
mod ints;
mod letters;
mod lists;
mod maps;
mod pairs;
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

fn int(sign: bool, s: &str, radix: u8) -> Repr {
    Repr::Int(Rc::new(Int::from_sign_string_radix(sign, s, radix)))
}

fn positive_decimal_int(s: &str) -> Repr {
    Repr::Int(Rc::new(Int::from_sign_string_radix(true, s, 10)))
}

fn float(sign: bool, integral: &str, fractional: &str, exp_sign: bool, exp_digits: &str) -> Repr {
    Repr::Float(Rc::new(Float::from_parts(
        sign, integral, fractional, exp_sign, exp_digits,
    )))
}

fn bytes(b: Vec<u8>) -> Repr {
    Repr::Bytes(Rc::new(b.into()))
}

fn letter(s: &str) -> Repr {
    Repr::Letter(Rc::new(Letter::new(s.to_owned())))
}

fn symbol(s: &str) -> Repr {
    Repr::Symbol(Rc::new(Symbol::new(s.to_owned())))
}

fn string(s: &str) -> Repr {
    Repr::String(Rc::new(s.into()))
}

fn pair(first: Repr, second: Repr) -> Repr {
    Repr::Pair(Box::new(PairRepr::new(first, second)))
}

fn call(func: Repr, arg: Repr) -> Repr {
    Repr::Call(Box::new(CallRepr::new(func, arg)))
}

fn list(v: Vec<Repr>) -> Repr {
    Repr::List(v.into())
}

fn map(v: Vec<(Repr, Repr)>) -> Repr {
    Repr::Map(Map::from_iter(v))
}

fn ltree(root: Repr, leaves: Vec<Repr>) -> Repr {
    Repr::Call(Box::new(CallRepr::new(
        root,
        Repr::List(ListRepr::from(leaves)),
    )))
}

fn mtree(root: Repr, leaves: Vec<(Repr, Repr)>) -> Repr {
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

fn test_parse(src: &str, expected: impl FnOnce() -> Vec<Repr>) -> Result<(), Box<dyn Error>> {
    let sources = src.split("# ===");
    let mut repr_list = Vec::new();
    for s in sources {
        repr_list.push(parse(dbg!(s)).map_err(|e| {
            println!("{e}");
            e
        })?);
    }
    assert_eq!(repr_list, expected());
    Ok(())
}

fn test_stringify(src: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split("# ===");
    for s in sources {
        let repr = parse(dbg!(s))?;
        let string = dbg!(repr.to_string());
        let new_repr = parse(&string)?;
        assert_eq!(repr, new_repr);
    }
    Ok(())
}

fn test_parse_illegal(src: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split("# ===");
    for s in sources {
        assert!(matches!(parse(dbg!(s)), Err(_)));
    }
    Ok(())
}

fn test_parse_bad(src: &str) -> Result<(), Box<dyn Error>> {
    let tests = src.split("# ===");

    for test in tests {
        let (i1, i2) = test.split_once("# ---").unwrap();
        let i1 = parse(dbg!(i1))?;
        let i2 = parse(dbg!(i2))?;
        assert_eq!(i1, i2);
    }
    Ok(())
}

#[test]
fn test_parse_units() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/units.air"), units::expected)
}

#[test]
fn test_stringify_units() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/units.air"))
}

#[test]
fn test_parse_booleans() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/booleans.air"), booleans::expected)
}

#[test]
fn test_stringify_booleans() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/booleans.air"))
}

#[test]
fn test_parse_ints() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/ints.air"), ints::expected)
}

#[test]
fn test_stringify_ints() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/ints.air"))
}

#[test]
fn test_parse_floats() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/floats.air"), floats::expected)
}

#[test]
fn test_stringify_floats() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/floats.air"))
}

#[test]
fn test_parse_bytes() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/bytes.air"), bytes::expected)
}

#[test]
fn test_stringify_bytes() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/bytes.air"))
}

#[test]
fn test_parse_letters() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/letters.air"), letters::expected)
}

#[test]
fn test_stringify_letters() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/letters.air"))
}

#[test]
fn test_parse_symbols() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/symbols.air"), symbols::expected)
}

#[test]
fn test_stringify_symbols() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/symbols.air"))
}

#[test]
fn test_parse_strings() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/strings.air"), strings::expected)
}

#[test]
fn test_stringify_strings() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/strings.air"))
}
#[test]
fn test_parse_pairs() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/pairs.air"), pairs::expected)
}

#[test]
fn test_stringify_pairs() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/pairs.air"))
}

#[test]
fn test_parse_calls() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/calls.air"), calls::expected)
}

#[test]
fn test_stringify_calls() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/calls.air"))
}

#[test]
fn test_parse_wraps() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/wraps.air"), wraps::expected)
}

#[test]
fn test_stringify_wraps() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/wraps.air"))
}

#[test]
fn test_parse_lists() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/lists.air"), lists::expected)
}

#[test]
fn test_stringify_lists() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/lists.air"))
}

#[test]
fn test_parse_maps() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/maps.air"), maps::expected)
}

#[test]
fn test_stringify_maps() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/maps.air"))
}

#[test]
fn test_parse_comments() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/comments.air"), comments::expected)
}

#[test]
fn test_stringify_comments() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/comments.air"))
}

#[test]
fn test_parse_infixes() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/infixes.air"), infixes::expected)
}

#[test]
fn test_stringify_infixes() -> Result<(), Box<dyn Error>> {
    test_stringify(include_str!("test/infixes.air"))
}

#[test]
fn test_parse_illegal_examples() -> Result<(), Box<dyn Error>> {
    test_parse_illegal(include_str!("test/illegals.air"))
}

#[test]
fn test_parse_bad_examples() -> Result<(), Box<dyn Error>> {
    test_parse_bad(include_str!("test/bad.air"))
}
