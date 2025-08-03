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
use rand::prelude::IteratorRandom;

use crate::prelude::mode::CompMode;
use crate::prelude::mode::ListMode;
use crate::prelude::mode::MapMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::PairMode;
use crate::prelude::mode::PrimMode;
use crate::prelude::mode::SymbolMode;
use crate::prelude::mode::TaskMode;
use crate::prelude::mode::TaskPrimMode;
use crate::prelude::put_preludes;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::func::ConstCellCompFunc;
use crate::semantics::func::ConstStaticCompFunc;
use crate::semantics::func::DynComposite;
use crate::semantics::func::FreeCellCompFunc;
use crate::semantics::func::FreeComposite;
use crate::semantics::func::FreeStaticCompFunc;
use crate::semantics::func::MutCellCompFunc;
use crate::semantics::func::MutStaticCompFunc;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstCellCompFuncVal;
use crate::semantics::val::ConstStaticCompFuncVal;
use crate::semantics::val::FreeCellCompFuncVal;
use crate::semantics::val::FreeStaticCompFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCellCompFuncVal;
use crate::semantics::val::MutStaticCompFuncVal;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Change;
use crate::type_::CtxInput;
use crate::type_::Either;
use crate::type_::FuncCtx;
use crate::type_::FuncCtxInput;
use crate::type_::FuncInput;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Text;
use crate::type_::Unit;

thread_local!(pub(in crate::prelude) static ARBITRARY: OnceCell<Box<dyn ArbitraryVal>> = OnceCell::new());

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
            1,      // ctx
            1,      // func
            1,      // extension
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;

        match i {
            0 => Val::Unit(Unit::any(rng, new_depth)),
            1 => Val::Bit(Bit::any(rng, new_depth)),
            2 => Val::Symbol(Symbol::any(rng, new_depth)),
            3 => Val::Text(Text::any(rng, new_depth).into()),
            4 => Val::Int(Int::any(rng, new_depth).into()),
            5 => Val::Number(Number::any(rng, new_depth).into()),
            6 => Val::Byte(Byte::any(rng, new_depth).into()),
            7 => Val::Pair(Pair::<Val, Val>::any(rng, new_depth).into()),
            8 => Val::Task(Task::<Val, Val, Val>::any(rng, new_depth).into()),
            9 => Val::List(List::<Val>::any(rng, new_depth).into()),
            10 => Val::Map(Map::<Val, Val>::any(rng, new_depth).into()),
            11 => Val::Ctx(Ctx::any(rng, new_depth).into()),
            12 => Val::Func(FuncVal::any(rng, new_depth)),
            13 => arbitrary_ext(),
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
        Pair::new(First::any(rng, depth), Second::any(rng, depth))
    }
}

impl<This, That> Arbitrary for Either<This, That>
where
    This: Arbitrary,
    That: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        if rng.random() {
            Either::This(This::any(rng, depth))
        } else {
            Either::That(That::any(rng, depth))
        }
    }
}

impl<From, To> Arbitrary for Change<From, To>
where
    From: Arbitrary,
    To: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        Change::new(From::any(rng, depth), To::any(rng, depth))
    }
}

impl<Func, Ctx, Input> Arbitrary for Task<Func, Ctx, Input>
where
    Func: Arbitrary,
    Ctx: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
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
        FuncCtx { func: Func::any(rng, depth), ctx: Ctx::any(rng, depth) }
    }
}

impl<Func, Input> Arbitrary for FuncInput<Func, Input>
where
    Func: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        FuncInput { func: Func::any(rng, depth), input: Input::any(rng, depth) }
    }
}

impl<Ctx, Input> Arbitrary for CtxInput<Ctx, Input>
where
    Ctx: Arbitrary,
    Input: Arbitrary,
{
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
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
        let mut map = Map::with_capacity(len);
        for _ in 0 .. len {
            map.insert(K::any(rng, depth), V::any(rng, depth));
        }
        map
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
        CtxValue::new(Val::any(rng, depth), Contract::any(rng, depth))
    }
}

impl Arbitrary for Ctx {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let variables = CtxMap::new(Map::any(rng, depth));
        Ctx::new(variables)
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        if rng.random() { None } else { Some(T::any(rng, depth)) }
    }
}

impl<T: Arbitrary> Arbitrary for Box<T> {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
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
        let new_depth = depth + 1;
        let some = Arbitrary::any(rng, new_depth);
        let first = Arbitrary::any(rng, new_depth);
        let second = Arbitrary::any(rng, new_depth);
        PairMode { some, first, second }
    }
}

impl Arbitrary for TaskMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let ctx = Arbitrary::any(rng, new_depth);
        let input = Arbitrary::any(rng, new_depth);
        TaskMode { func, ctx, input }
    }
}

impl Arbitrary for ListMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let new_depth = depth + 1;
        let head = Arbitrary::any(rng, new_depth);
        let tail = Arbitrary::any(rng, new_depth);
        ListMode { head, tail }
    }
}

impl Arbitrary for MapMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let new_depth = depth + 1;
        let some = Arbitrary::any(rng, new_depth);
        let else_ = Arbitrary::any(rng, new_depth);
        MapMode { some, else_ }
    }
}

impl Arbitrary for PrimMode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let symbol = Arbitrary::any(rng, depth);
        let task = Arbitrary::any(rng, depth);
        PrimMode { symbol, task }
    }
}

impl Arbitrary for Mode {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let new_depth = depth + 1;
        if rng.random() {
            Mode::Comp(Arbitrary::any(rng, new_depth))
        } else {
            Mode::Func(FuncVal::any(rng, new_depth))
        }
    }
}

impl Arbitrary for FuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        if rng.random() {
            let mut prelude: Map<Symbol, Val> = Map::with_capacity(128);
            put_preludes(&mut prelude);
            let func =
                prelude.into_values().filter(|v| matches!(v, Val::Func(_))).choose(rng).unwrap();
            let Val::Func(func) = func else { unreachable!() };
            func
        } else {
            match rng.random_range(0 .. 6) {
                0 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::FreeStaticComp(func)
                }
                1 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::ConstStaticComp(func)
                }
                2 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::MutStaticComp(func)
                }
                3 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::FreeCellComp(func)
                }
                4 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::ConstCellComp(func)
                }
                5 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::MutCellComp(func)
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Arbitrary for Setup {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let call_input = Arbitrary::any(rng, depth);
        let solve_input = Arbitrary::any(rng, depth);
        Setup { call: call_input, solve: solve_input }
    }
}

impl Arbitrary for FreeComposite {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        FreeComposite { body: Arbitrary::any(rng, depth), input_name: Arbitrary::any(rng, depth) }
    }
}

impl Arbitrary for DynComposite {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        DynComposite { free: Arbitrary::any(rng, depth), ctx_name: Arbitrary::any(rng, depth) }
    }
}

impl Arbitrary for FreeCellCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = FreeCellCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        FreeCellCompFuncVal::from(func)
    }
}

impl Arbitrary for FreeStaticCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = FreeStaticCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        FreeStaticCompFuncVal::from(func)
    }
}

impl Arbitrary for ConstCellCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = ConstCellCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        ConstCellCompFuncVal::from(func)
    }
}

impl Arbitrary for ConstStaticCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = ConstStaticCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        ConstStaticCompFuncVal::from(func)
    }
}

impl Arbitrary for MutCellCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = MutCellCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        MutCellCompFuncVal::from(func)
    }
}

impl Arbitrary for MutStaticCompFuncVal {
    fn any<R: Rng + ?Sized>(rng: &mut R, depth: usize) -> Self {
        let func = MutStaticCompFunc {
            id: Arbitrary::any(rng, depth),
            comp: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            setup: Arbitrary::any(rng, depth),
        };
        MutStaticCompFuncVal::from(func)
    }
}

pub(in crate::prelude) fn arbitrary_ext() -> Val {
    ARBITRARY.with(|ext| {
        let Some(ext) = ext.get() else {
            return Val::default();
        };
        ext.arbitrary()
    })
}

pub(in crate::prelude) fn arbitrary_ext_type(type_: Symbol) -> Val {
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
