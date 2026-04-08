use std::hash::Hash;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_bigint::Sign;
use rand::Rng;
use rand::RngExt;
use rand::distr::SampleString;
use rand::distr::StandardUniform;
use rand::prelude::Distribution;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::CompCtx;
use crate::semantics::func::CompFunc;
use crate::semantics::func::CompInput;
use crate::semantics::val::CompFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Decimal;
use crate::type_::Either;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Quote;
use crate::type_::Text;
use crate::type_::Unit;

pub trait Arbitrary {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

impl Arbitrary for Val {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.random_range(0 ..= 15) {
            0 => Val::Unit(Unit::any(rng)),
            1 => Val::Bit(Bit::any(rng)),
            2 => Val::Key(Key::any(rng)),
            3 => Val::Text(Text::any(rng).into()),
            4 => Val::Int(Int::any(rng).into()),
            5 => Val::Decimal(Decimal::any(rng).into()),
            6 => Val::Byte(Byte::any(rng).into()),
            7 => Val::Cell(Cell::<Val>::any(rng).into()),
            8 => Val::Pair(Pair::<Val, Val>::any(rng).into()),
            9 => Val::List(List::<Val>::any(rng).into()),
            10 => Val::Map(Map::<Key, Val>::any(rng).into()),
            11 => Val::Quote(Quote::<Val>::any(rng).into()),
            12 => Val::Call(Call::<Val, Val>::any(rng).into()),
            13 => Val::Link(LinkVal::any(rng)),
            14 => Val::Cfg(Cfg::any(rng).into()),
            15 => Val::Func(FuncVal::any(rng)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Unit {
    fn any<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        Unit
    }
}

impl Arbitrary for Bit {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Bit::from(rng.random::<bool>())
    }
}

struct DistKey;

impl Distribution<u8> for DistKey {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        rng.random_range(Key::MIN ..= Key::MAX) as u8
    }
}

impl SampleString for DistKey {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // safety: keys are valid utf-8
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}

impl Arbitrary for Key {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let len = any_len(rng, 8, 2);
        let s = DistKey.sample_string(rng, len);
        Key::from_string_unchecked(s)
    }
}

impl Arbitrary for Text {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let len = any_len(rng, 8, 2);
        let s: String = rng.sample_iter::<char, _>(StandardUniform).take(len).collect();
        Text::from(s)
    }
}

impl Arbitrary for Int {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Int::from(any_int(rng))
    }
}

impl Arbitrary for Decimal {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let d = BigDecimal::from_bigint(any_int(rng), rng.random());
        Decimal::new(d)
    }
}

impl Arbitrary for Byte {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Byte::from(any_bytes(rng))
    }
}

impl<Value> Arbitrary for Cell<Value>
where Value: Arbitrary
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Cell::new(Value::any(rng))
    }
}

impl<Value> Arbitrary for Quote<Value>
where Value: Arbitrary
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Quote::new(Value::any(rng))
    }
}

impl<Left, Right> Arbitrary for Pair<Left, Right>
where
    Left: Arbitrary,
    Right: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Pair::new(Left::any(rng), Right::any(rng))
    }
}

impl<This, That> Arbitrary for Either<This, That>
where
    This: Arbitrary,
    That: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        if rng.random() { Either::This(This::any(rng)) } else { Either::That(That::any(rng)) }
    }
}

impl<Func, Input> Arbitrary for Call<Func, Input>
where
    Func: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Call { func: Func::any(rng), input: Input::any(rng) }
    }
}

impl<T> Arbitrary for List<T>
where T: Arbitrary
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut list = Vec::new();
        for _ in 0 .. any_len(rng, 2, 4) {
            list.push(T::any(rng));
        }
        List::from(list)
    }
}

impl<K, V> Arbitrary for Map<K, V>
where
    K: Eq + Hash + Arbitrary,
    V: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut map = Map::default();
        for _ in 0 .. any_len(rng, 2, 4) {
            map.insert(K::any(rng), V::any(rng));
        }
        map
    }
}

impl Arbitrary for LinkVal {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let val = Val::any(rng);
        let const_ = rng.random();
        LinkVal::new(val, const_)
    }
}

impl Arbitrary for Cfg {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Cfg::from(Map::any(rng))
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        if rng.random_ratio(1, 4) { None } else { Some(T::any(rng)) }
    }
}

impl<T: Arbitrary> Arbitrary for Box<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Box::new(T::any(rng))
    }
}

impl Arbitrary for FuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let func = Arbitrary::any(rng);
        FuncVal::Comp(func)
    }
}

impl Arbitrary for CompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let ctx = if rng.random() {
            CompCtx::Aware { name: Arbitrary::any(rng), const_: rng.random() }
        } else {
            CompCtx::Free
        };
        let input = if rng.random() {
            CompInput::Aware { name: Arbitrary::any(rng) }
        } else {
            CompInput::Free
        };
        let func = CompFunc { prelude: Arbitrary::any(rng), body: Arbitrary::any(rng), input, ctx };
        CompFuncVal::from(func)
    }
}

fn any_int<R: Rng + ?Sized>(rng: &mut R) -> BigInt {
    let sign = if rng.random() { Sign::Plus } else { Sign::Minus };
    let bytes = any_bytes(rng);
    BigInt::from_bytes_le(sign, &bytes)
}

fn any_bytes<R: Rng + ?Sized>(rng: &mut R) -> Vec<u8> {
    let len = any_len(rng, 8, 2);
    let mut bytes = vec![0u8; len];
    rng.fill(&mut bytes);
    bytes
}

fn any_len<R: Rng + ?Sized>(rng: &mut R, unit: usize, ratio: u32) -> usize {
    let mut start = 0;
    loop {
        if !rng.random_ratio(1, ratio) {
            return start + rng.random_range(0 .. unit);
        }
        start += unit;
    }
}
