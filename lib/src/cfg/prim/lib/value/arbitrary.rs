use std::hash::Hash;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_bigint::Sign;
use rand::Rng;
use rand::RngExt;
use rand::distr::SampleString;
use rand::distr::StandardUniform;
use rand::distr::Uniform;
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

#[derive(Copy, Clone)]
pub(crate) struct Any;

impl Distribution<Val> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Val {
        match rng.random_range(0 ..= 15) {
            0 => Val::Unit(Distribution::<Unit>::sample(self, rng)),
            1 => Val::Bit(Distribution::<Bit>::sample(self, rng)),
            2 => Val::Key(Distribution::<Key>::sample(self, rng)),
            3 => Val::Text(Distribution::<Text>::sample(self, rng).into()),
            4 => Val::Int(Distribution::<Int>::sample(self, rng).into()),
            5 => Val::Decimal(Distribution::<Decimal>::sample(self, rng).into()),
            6 => Val::Byte(Distribution::<Byte>::sample(self, rng).into()),
            7 => Val::Cell(Distribution::<Cell<Val>>::sample(self, rng).into()),
            8 => Val::Pair(Distribution::<Pair<Val, Val>>::sample(self, rng).into()),
            9 => Val::List(Distribution::<List<Val>>::sample(self, rng).into()),
            10 => Val::Map(Distribution::<Map<Key, Val>>::sample(self, rng).into()),
            11 => Val::Quote(Distribution::<Quote<Val>>::sample(self, rng).into()),
            12 => Val::Call(Distribution::<Call<Val, Val>>::sample(self, rng).into()),
            13 => Val::Link(Distribution::<LinkVal>::sample(self, rng)),
            14 => Val::Cfg(Distribution::<Cfg>::sample(self, rng).into()),
            15 => Val::Func(Distribution::<FuncVal>::sample(self, rng)),
            _ => unreachable!(),
        }
    }
}

impl Distribution<Unit> for Any {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Unit {
        Unit
    }
}

impl Distribution<Bit> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bit {
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

impl Distribution<Key> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Key {
        let len = any_len(rng, 8, 2);
        let s = DistKey.sample_string(rng, len);
        Key::from_string_unchecked(s)
    }
}

impl Distribution<Text> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Text {
        let len = any_len(rng, 8, 2);
        let s: String = if rng.random() {
            let uniform = Uniform::new(char::from(0), char::from(128)).unwrap();
            rng.sample_iter::<char, _>(uniform).take(len).collect()
        } else {
            rng.sample_iter::<char, _>(StandardUniform).take(len).collect()
        };
        Text::from(s)
    }
}

impl Distribution<Int> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Int {
        Int::from(any_int(rng))
    }
}

impl Distribution<Decimal> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Decimal {
        let d = BigDecimal::from_bigint(any_int(rng), rng.random());
        Decimal::new(d)
    }
}

impl Distribution<Byte> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Byte {
        Byte::from(any_bytes(rng))
    }
}

impl<Value> Distribution<Cell<Value>> for Any
where Any: Distribution<Value>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell<Value> {
        Cell::new(self.sample(rng))
    }
}

impl<Value> Distribution<Quote<Value>> for Any
where Any: Distribution<Value>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quote<Value> {
        Quote::new(self.sample(rng))
    }
}

impl<Left, Right> Distribution<Pair<Left, Right>> for Any
where Any: Distribution<Left> + Distribution<Right>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pair<Left, Right> {
        Pair::new(self.sample(rng), self.sample(rng))
    }
}

impl<This, That> Distribution<Either<This, That>> for Any
where Any: Distribution<This> + Distribution<That>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Either<This, That> {
        if rng.random() { Either::This(self.sample(rng)) } else { Either::That(self.sample(rng)) }
    }
}

impl<Func, Input> Distribution<Call<Func, Input>> for Any
where Any: Distribution<Func> + Distribution<Input>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Call<Func, Input> {
        Call { func: self.sample(rng), input: self.sample(rng) }
    }
}

impl<T> Distribution<List<T>> for Any
where Any: Distribution<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> List<T> {
        let mut list = Vec::new();
        for _ in 0 .. any_len(rng, 2, 4) {
            list.push(self.sample(rng));
        }
        List::from(list)
    }
}

impl<K, V> Distribution<Map<K, V>> for Any
where
    K: Eq + Hash,
    Any: Distribution<K> + Distribution<V>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Map<K, V> {
        let mut map = Map::default();
        for _ in 0 .. any_len(rng, 2, 4) {
            map.insert(self.sample(rng), self.sample(rng));
        }
        map
    }
}

impl Distribution<LinkVal> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LinkVal {
        let val = self.sample(rng);
        let const_ = rng.random();
        LinkVal::new(val, const_)
    }
}

impl Distribution<Cfg> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cfg {
        let map: Map<Key, Val> = self.sample(rng);
        Cfg::from(map)
    }
}

impl<T> Distribution<Option<T>> for Any
where Any: Distribution<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<T> {
        if rng.random_ratio(1, 4) { None } else { Some(self.sample(rng)) }
    }
}

impl<T> Distribution<Box<T>> for Any
where Any: Distribution<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<T> {
        Box::new(self.sample(rng))
    }
}

impl Distribution<FuncVal> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FuncVal {
        let func = self.sample(rng);
        FuncVal::Comp(func)
    }
}

impl Distribution<CompFuncVal> for Any {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CompFuncVal {
        let ctx = if rng.random() {
            CompCtx::Aware { name: self.sample(rng), const_: rng.random() }
        } else {
            CompCtx::Free
        };
        let input = if rng.random() {
            CompInput::Aware { name: self.sample(rng) }
        } else {
            CompInput::Free
        };
        let func = CompFunc { prelude: self.sample(rng), body: self.sample(rng), input, ctx };
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
