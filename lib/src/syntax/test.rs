use {
    crate::{
        repr::{
            CallRepr,
            ListRepr,
            MapRepr,
            PairRepr,
            Repr,
            ReverseRepr,
        },
        syntax::parse,
        types::{
            Bool,
            Float,
            Int,
            Map,
            Str,
            Symbol,
            Unit,
        },
    },
    std::error::Error,
};

mod booleans;
mod bytes;
mod calls;
mod comments;
mod floats;
mod infixes;
mod ints;
mod lists;
mod maps;
mod pairs;
mod reverses;
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
    Repr::Int(Int::from_sign_string_radix(sign, s, radix))
}

fn positive_decimal_int(s: &str) -> Repr {
    Repr::Int(Int::from_sign_string_radix(true, s, 10))
}

fn float(sign: bool, integral: &str, fractional: &str, exp_sign: bool, exp_digits: &str) -> Repr {
    Repr::Float(Float::from_parts(
        sign, integral, fractional, exp_sign, exp_digits,
    ))
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

fn reverse(func: Repr, output: Repr) -> Repr {
    Repr::Reverse(Box::new(ReverseRepr::new(func, output)))
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

    for (s, expected_repr) in sources.zip(expected()) {
        let real_repr = parse(s).map_err(|e| {
            eprintln!("{e}");
            e
        })?;
        assert_eq!(
            real_repr, expected_repr,
            "src: {}, real: {} != expected: {}",
            s, real_repr, expected_repr
        );
    }
    Ok(())
}

fn test_generate(src: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split("# ===");
    for s in sources {
        let repr = parse(s)?;
        let string = repr.to_string();
        let new_repr = parse(&string)?;
        assert_eq!(
            repr, new_repr,
            "src: {}, repr: {} != new_repr: {}",
            s, repr, new_repr
        );
    }
    Ok(())
}

fn test_parse_illegal(src: &str) -> Result<(), Box<dyn Error>> {
    let sources = src.split("# ===");
    for s in sources {
        assert!(matches!(parse(s), Err(_)), "src: {} shouldn't parse", s);
    }
    Ok(())
}

fn test_parse_bad(src: &str) -> Result<(), Box<dyn Error>> {
    let tests = src.split("# ===");

    for test in tests {
        let (i1, i2) = test.split_once("# ---").unwrap();
        let i1 = parse(i1)?;
        let i2 = parse(i2)?;
        assert_eq!(i1, i2, "src1: {} != scr2: {}", i1, i2);
    }
    Ok(())
}

#[test]
fn test_parse_units() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/units.air"), units::expected)
}

#[test]
fn test_generate_units() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/units.air"))
}

#[test]
fn test_parse_booleans() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/booleans.air"), booleans::expected)
}

#[test]
fn test_generate_booleans() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/booleans.air"))
}

#[test]
fn test_parse_ints() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/ints.air"), ints::expected)
}

#[test]
fn test_generate_ints() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/ints.air"))
}

#[test]
fn test_parse_floats() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/floats.air"), floats::expected)
}

#[test]
fn test_generate_floats() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/floats.air"))
}

#[test]
fn test_parse_bytes() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/bytes.air"), bytes::expected)
}

#[test]
fn test_generate_bytes() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/bytes.air"))
}

#[test]
fn test_parse_symbols() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/symbols.air"), symbols::expected)
}

#[test]
fn test_generate_symbols() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/symbols.air"))
}

#[test]
fn test_parse_strings() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/strings.air"), strings::expected)
}

#[test]
fn test_generate_strings() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/strings.air"))
}
#[test]
fn test_parse_pairs() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/pairs.air"), pairs::expected)
}

#[test]
fn test_generate_pairs() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/pairs.air"))
}

#[test]
fn test_parse_calls() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/calls.air"), calls::expected)
}

#[test]
fn test_generate_calls() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/calls.air"))
}

#[test]
fn test_parse_reverses() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/reverses.air"), reverses::expected)
}

#[test]
fn test_generate_reverses() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/reverses.air"))
}

#[test]
fn test_parse_wraps() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/wraps.air"), wraps::expected)
}

#[test]
fn test_generate_wraps() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/wraps.air"))
}

#[test]
fn test_parse_lists() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/lists.air"), lists::expected)
}

#[test]
fn test_generate_lists() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/lists.air"))
}

#[test]
fn test_parse_maps() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/maps.air"), maps::expected)
}

#[test]
fn test_generate_maps() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/maps.air"))
}

#[test]
fn test_parse_comments() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/comments.air"), comments::expected)
}

#[test]
fn test_generate_comments() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/comments.air"))
}

#[test]
fn test_parse_infixes() -> Result<(), Box<dyn Error>> {
    test_parse(include_str!("test/infixes.air"), infixes::expected)
}

#[test]
fn test_generate_infixes() -> Result<(), Box<dyn Error>> {
    test_generate(include_str!("test/infixes.air"))
}

#[test]
fn test_parse_illegal_examples() -> Result<(), Box<dyn Error>> {
    test_parse_illegal(include_str!("test/illegals.air"))
}

#[test]
fn test_parse_bad_examples() -> Result<(), Box<dyn Error>> {
    test_parse_bad(include_str!("test/bad.air"))
}
