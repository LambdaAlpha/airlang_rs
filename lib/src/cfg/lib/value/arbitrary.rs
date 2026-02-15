use std::cmp::min;
use std::hash::Hash;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use rand::Rng;
use rand::RngExt;
use rand::distr::SampleString;
use rand::distr::StandardUniform;
use rand::distr::weighted::WeightedIndex;
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
use crate::type_::Text;
use crate::type_::Unit;

pub trait Arbitrary {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self;
}

impl Arbitrary for Val {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // unit
            weight, // bit
            weight, // key
            weight, // text
            weight, // int
            weight, // decimal
            weight, // byte
            1,      // cell
            1,      // pair
            1,      // call
            1,      // list
            1,      // map
            1,      // link
            1,      // cfg
            1,      // func
        ];
        let i = sample(rng, weights);
        let depth = depth + 1;

        match i {
            0 => Val::Unit(Unit::any(rng, depth)),
            1 => Val::Bit(Bit::any(rng, depth)),
            2 => Val::Key(Key::any(rng, depth)),
            3 => Val::Text(Text::any(rng, depth).into()),
            4 => Val::Int(Int::any(rng, depth).into()),
            5 => Val::Decimal(Decimal::any(rng, depth).into()),
            6 => Val::Byte(Byte::any(rng, depth).into()),
            7 => Val::Cell(Cell::<Val>::any(rng, depth).into()),
            8 => Val::Pair(Pair::<Val, Val>::any(rng, depth).into()),
            9 => Val::Call(Call::<Val, Val>::any(rng, depth).into()),
            10 => Val::List(List::<Val>::any(rng, depth).into()),
            11 => Val::Map(Map::<Key, Val>::any(rng, depth).into()),
            12 => Val::Link(LinkVal::any(rng, depth)),
            13 => Val::Cfg(Cfg::any(rng, depth).into()),
            14 => Val::Func(FuncVal::any(rng, depth)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Unit {
    fn any<R: Rng + ?Sized>(_rng: &mut R, _depth: usize) -> Self {
        Unit
    }
}

impl Arbitrary for Bit {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
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
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let len = any_len(rng);
        let s = DistKey.sample_string(rng, len);
        Key::from_string_unchecked(s)
    }
}

impl Arbitrary for Text {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let len = any_len(rng);
        let s: String = rng.sample_iter::<char, _>(StandardUniform).take(len).collect();
        Text::from(s)
    }
}

impl Arbitrary for Int {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        Int::from(rng.random::<i128>())
    }
}

impl Arbitrary for Decimal {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let int: i64 = rng.random();
        let int = BigInt::from(int);
        let exp: i8 = rng.random();
        Decimal::new(BigDecimal::from_bigint(int, exp as i64))
    }
}

impl Arbitrary for Byte {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let len = any_len(rng);
        let mut byte = vec![0u8; len];
        rng.fill(&mut byte);
        Byte::from(byte)
    }
}

impl<Value> Arbitrary for Cell<Value>
where Value: Arbitrary
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Cell::new(Value::any(rng, depth))
    }
}

impl<Left, Right> Arbitrary for Pair<Left, Right>
where
    Left: Arbitrary,
    Right: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Pair::new(Left::any(rng, depth), Right::any(rng, depth))
    }
}

impl<This, That> Arbitrary for Either<This, That>
where
    This: Arbitrary,
    That: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        if rng.random() {
            Either::This(This::any(rng, depth))
        } else {
            Either::That(That::any(rng, depth))
        }
    }
}

impl<Func, Input> Arbitrary for Call<Func, Input>
where
    Func: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Call { func: Func::any(rng, depth), input: Input::any(rng, depth) }
    }
}

impl<T> Arbitrary for List<T>
where T: Arbitrary
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let len = any_len_weighted(rng, depth);
        let depth = depth + 1;
        let mut list = Vec::with_capacity(len);
        for _ in 0 .. len {
            list.push(T::any(rng, depth));
        }
        List::from(list)
    }
}

impl<K, V> Arbitrary for Map<K, V>
where
    K: Eq + Hash + Arbitrary,
    V: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let len = any_len_weighted(rng, depth);
        let depth = depth + 1;
        let mut map = Map::with_capacity(len);
        for _ in 0 .. len {
            map.insert(K::any(rng, depth), V::any(rng, depth));
        }
        map
    }
}

impl Arbitrary for LinkVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let val = Val::any(rng, depth);
        let const_ = rng.random();
        LinkVal::new(val, const_)
    }
}

impl Arbitrary for Cfg {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Cfg::from(Map::any(rng, depth))
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        if rng.random() { None } else { Some(T::any(rng, depth)) }
    }
}

impl<T: Arbitrary> Arbitrary for Box<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Box::new(T::any(rng, depth))
    }
}

impl Arbitrary for FuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let func = Arbitrary::any(rng, depth);
        FuncVal::Comp(func)
    }
}

impl Arbitrary for CompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let ctx = if rng.random() {
            CompCtx::Default { name: Arbitrary::any(rng, depth), const_: rng.random() }
        } else {
            CompCtx::Free
        };
        let input = if rng.random() {
            CompInput::Default { name: Arbitrary::any(rng, depth), raw: rng.random() }
        } else {
            CompInput::Free
        };
        let func = CompFunc {
            prelude: Arbitrary::any(rng, depth),
            body: Arbitrary::any(rng, depth),
            input,
            ctx,
        };
        CompFuncVal::from(func)
    }
}

fn sample<const N: usize, R: Rng + ?Sized>(rng: &mut R, weights: [usize; N]) -> usize {
    let dist = WeightedIndex::new(weights).unwrap();
    dist.sample(rng)
}

fn any_len_weighted<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> usize {
    const WEIGHTS: [usize; 16] = [16, 16, 16, 16, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 1];
    let dist = WeightedIndex::new(WEIGHTS).unwrap();
    let limit = 16usize.saturating_sub(depth);
    let len = dist.sample(rng);
    min(len, limit)
}

fn any_len<R: Rng + ?Sized>(rng: &mut R) -> usize {
    let len: u8 = rng.random();
    len as usize
}
