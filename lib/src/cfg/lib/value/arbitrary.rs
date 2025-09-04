use std::cell::OnceCell;
use std::cmp::min;
use std::hash::Hash;

use num_bigint::BigInt;
use rand::Rng;
use rand::distr::SampleString;
use rand::distr::StandardUniform;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::Distribution;
use rand::prelude::IndexedRandom;

use crate::cfg::lib::mode::CompMode;
use crate::cfg::lib::mode::ListMode;
use crate::cfg::lib::mode::MapMode;
use crate::cfg::lib::mode::Mode;
use crate::cfg::lib::mode::PairMode;
use crate::cfg::lib::mode::PrimMode;
use crate::cfg::lib::mode::SymbolMode;
use crate::cfg::lib::mode::TaskMode;
use crate::cfg::lib::mode::TaskPrimMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::func::ConstCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::MutCompFunc;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstCompFuncVal;
use crate::semantics::val::FreeCompFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCompFuncVal;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::CtxInput;
use crate::type_::Either;
use crate::type_::FuncCtx;
use crate::type_::FuncCtxInput;
use crate::type_::FuncInput;
use crate::type_::Int;
use crate::type_::Link;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Text;
use crate::type_::Unit;

thread_local!(pub(in crate::cfg) static ARBITRARY: OnceCell<Box<dyn ArbitraryVal>> = OnceCell::new());

pub trait ArbitraryVal {
    fn arbitrary(&self) -> Val;

    fn arbitrary_type(&self, type_: Symbol) -> Val;
}

pub(crate) fn set_arbitrary_val(arbitrary: Box<dyn ArbitraryVal>) {
    ARBITRARY.with(|arb| {
        let _ = arb.set(arbitrary);
    });
}

pub trait Arbitrary {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self;
}

impl Arbitrary for Val {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // unit
            weight, // bit
            weight, // symbol
            weight, // text
            weight, // int
            weight, // number
            weight, // byte
            1,      // pair
            1,      // call
            1,      // list
            1,      // map
            1,      // link
            1,      // cfg
            1,      // ctx
            1,      // func
            1,      // extension
        ];
        let i = sample(rng, weights);
        let depth = depth + 1;

        match i {
            0 => Val::Unit(Unit::any(rng, depth)),
            1 => Val::Bit(Bit::any(rng, depth)),
            2 => Val::Symbol(Symbol::any(rng, depth)),
            3 => Val::Text(Text::any(rng, depth).into()),
            4 => Val::Int(Int::any(rng, depth).into()),
            5 => Val::Number(Number::any(rng, depth).into()),
            6 => Val::Byte(Byte::any(rng, depth).into()),
            7 => Val::Pair(Pair::<Val, Val>::any(rng, depth).into()),
            8 => Val::Task(Task::<Val, Val, Val>::any(rng, depth).into()),
            9 => Val::List(List::<Val>::any(rng, depth).into()),
            10 => Val::Map(Map::<Val, Val>::any(rng, depth).into()),
            11 => Val::Link(Link::any(rng, depth)),
            12 => Val::Cfg(Cfg::any(rng, depth).into()),
            13 => Val::Ctx(Ctx::any(rng, depth).into()),
            14 => Val::Func(FuncVal::any(rng, depth)),
            15 => arbitrary_ext(),
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

struct DistSymbol;

impl Distribution<u8> for DistSymbol {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        rng.random_range(Symbol::MIN ..= Symbol::MAX) as u8
    }
}

impl SampleString for DistSymbol {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // safety: symbols are valid utf-8
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}

impl Arbitrary for Symbol {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let len = any_len(rng);
        let s = DistSymbol.sample_string(rng, len);
        Symbol::from_string_unchecked(s)
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

impl Arbitrary for Number {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let int: i64 = rng.random();
        let int = BigInt::from(int);
        let exp: i8 = rng.random();
        let exp = BigInt::from(exp);
        Number::new(int, 10, exp)
    }
}

impl Arbitrary for Byte {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        let len = any_len(rng);
        let mut byte = vec![0u8; len];
        rng.fill(&mut *byte);
        Byte::from(byte)
    }
}

impl<First, Second> Arbitrary for Pair<First, Second>
where
    First: Arbitrary,
    Second: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Pair::new(First::any(rng, depth), Second::any(rng, depth))
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

impl<Func, Ctx, Input> Arbitrary for Task<Func, Ctx, Input>
where
    Func: Arbitrary,
    Ctx: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Task {
            action: Action::any(rng, depth),
            func: Func::any(rng, depth),
            ctx: Ctx::any(rng, depth),
            input: Input::any(rng, depth),
        }
    }
}

impl Arbitrary for Action {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        if rng.random() { Action::Call } else { Action::Solve }
    }
}

impl<Func, Ctx> Arbitrary for FuncCtx<Func, Ctx>
where
    Func: Arbitrary,
    Ctx: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        FuncCtx { func: Func::any(rng, depth), ctx: Ctx::any(rng, depth) }
    }
}

impl<Func, Input> Arbitrary for FuncInput<Func, Input>
where
    Func: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        FuncInput { func: Func::any(rng, depth), input: Input::any(rng, depth) }
    }
}

impl<Ctx, Input> Arbitrary for CtxInput<Ctx, Input>
where
    Ctx: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        CtxInput { ctx: Ctx::any(rng, depth), input: Input::any(rng, depth) }
    }
}

impl<Func, Ctx, Input> Arbitrary for FuncCtxInput<Func, Ctx, Input>
where
    Func: Arbitrary,
    Ctx: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        FuncCtxInput {
            func: Func::any(rng, depth),
            ctx: Ctx::any(rng, depth),
            input: Input::any(rng, depth),
        }
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

impl<T: Arbitrary> Arbitrary for Link<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        Link::new(T::any(rng, depth))
    }
}

impl Arbitrary for Cfg {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let len = any_len_weighted(rng, depth);
        let depth = depth + 1;
        let cfg = Cfg::default();
        for _ in 0 .. len {
            cfg.extend_scope(Symbol::any(rng, depth), Val::any(rng, depth));
        }
        cfg
    }
}

impl Arbitrary for Contract {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        const CONTRACT: [Contract; 5] =
            [Contract::None, Contract::Static, Contract::Still, Contract::Final, Contract::Const];
        *(CONTRACT.choose(rng).unwrap())
    }
}

impl Arbitrary for CtxValue {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        CtxValue::new(Val::any(rng, depth), Contract::any(rng, depth))
    }
}

impl Arbitrary for Ctx {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let variables = CtxMap::new(Map::any(rng, depth));
        Ctx::new(variables)
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

impl Arbitrary for TaskPrimMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        const MODES: [TaskPrimMode; 2] = [TaskPrimMode::Form, TaskPrimMode::Eval];
        *(MODES.choose(rng).unwrap())
    }
}

impl Arbitrary for SymbolMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, _depth: usize) -> Self {
        const MODES: [SymbolMode; 3] = [SymbolMode::Literal, SymbolMode::Ref, SymbolMode::Eval];
        *(MODES.choose(rng).unwrap())
    }
}

impl Arbitrary for CompMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let default = Arbitrary::any(rng, depth);
        let pair = Arbitrary::any(rng, depth);
        let task = Arbitrary::any(rng, depth);
        let list = Arbitrary::any(rng, depth);
        let map = Arbitrary::any(rng, depth);
        CompMode { default, pair, task, list, map }
    }
}

impl Arbitrary for PairMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let some = Arbitrary::any(rng, depth);
        let first = Arbitrary::any(rng, depth);
        let second = Arbitrary::any(rng, depth);
        PairMode { some, first, second }
    }
}

impl Arbitrary for TaskMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let func = Arbitrary::any(rng, depth);
        let ctx = Arbitrary::any(rng, depth);
        let input = Arbitrary::any(rng, depth);
        TaskMode { func, ctx, input }
    }
}

impl Arbitrary for ListMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let head = Arbitrary::any(rng, depth);
        let tail = Arbitrary::any(rng, depth);
        ListMode { head, tail }
    }
}

impl Arbitrary for MapMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let some = Arbitrary::any(rng, depth);
        let else_ = Arbitrary::any(rng, depth);
        MapMode { some, else_ }
    }
}

impl Arbitrary for PrimMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let symbol = Arbitrary::any(rng, depth);
        let task = Arbitrary::any(rng, depth);
        PrimMode { symbol, task }
    }
}

impl Arbitrary for Mode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        if rng.random() {
            Mode::Comp(Arbitrary::any(rng, depth))
        } else {
            Mode::Func(FuncVal::any(rng, depth))
        }
    }
}

impl Arbitrary for FuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        if rng.random() {
            return FuncVal::default();
        }
        match rng.random_range(0 .. 3) {
            0 => {
                let func = Arbitrary::any(rng, depth);
                FuncVal::FreeComp(func)
            }
            1 => {
                let func = Arbitrary::any(rng, depth);
                FuncVal::ConstComp(func)
            }
            2 => {
                let func = Arbitrary::any(rng, depth);
                FuncVal::MutComp(func)
            }
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Setup {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let call_input = Arbitrary::any(rng, depth);
        let solve_input = Arbitrary::any(rng, depth);
        Setup { call: call_input, solve: solve_input }
    }
}

impl Arbitrary for FreeComposite {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        FreeComposite { body: Arbitrary::any(rng, depth), input_name: Arbitrary::any(rng, depth) }
    }
}

impl Arbitrary for DynComposite {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        DynComposite { free: Arbitrary::any(rng, depth), ctx_name: Arbitrary::any(rng, depth) }
    }
}

impl Arbitrary for FreeCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let func = FreeCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        FreeCompFuncVal::from(func)
    }
}

impl Arbitrary for ConstCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let func = ConstCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        ConstCompFuncVal::from(func)
    }
}

impl Arbitrary for MutCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let depth = depth + 1;
        let func = MutCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        MutCompFuncVal::from(func)
    }
}

pub(in crate::cfg) fn arbitrary_ext() -> Val {
    ARBITRARY.with(|ext| {
        let Some(ext) = ext.get() else {
            return Val::default();
        };
        ext.arbitrary()
    })
}

pub(in crate::cfg) fn arbitrary_ext_type(type_: Symbol) -> Val {
    ARBITRARY.with(|ext| {
        let Some(ext) = ext.get() else {
            return Val::default();
        };
        ext.arbitrary_type(type_)
    })
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
