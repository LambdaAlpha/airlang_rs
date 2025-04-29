use std::{
    cmp::min,
    hash::Hash,
};

use num_bigint::BigInt;
use rand::{
    Rng,
    distr::{
        SampleString,
        StandardUniform,
        weighted::WeightedIndex,
    },
    prelude::{
        Distribution,
        IndexedRandom,
        IteratorRandom,
        SmallRng,
    },
};

use crate::{
    Call,
    CallMode,
    ConstStaticCompFunc,
    ConstStaticCompFuncVal,
    FreeCellCompFunc,
    FreeCellCompFuncVal,
    FreeStaticCompFunc,
    FreeStaticCompFuncVal,
    Mode,
    ModeFunc,
    ModeFuncVal,
    MutStaticCompFunc,
    MutStaticCompFuncVal,
    SymbolMode,
    Val,
    ValExt,
    bit::Bit,
    byte::Byte,
    change::Change,
    ctx::{
        Ctx,
        map::{
            CtxMap,
            CtxValue,
            VarAccess,
        },
    },
    either::Either,
    extension::UnitExt,
    func::{
        comp::Composite,
        const_cell_comp::ConstCellCompFunc,
        func_mode::FuncMode,
        mut_cell_comp::MutCellCompFunc,
    },
    int::Int,
    list::List,
    map::Map,
    mode::{
        comp::CompMode,
        list::ListMode,
        map::MapMode,
        pair::PairMode,
        prim::{
            CodeMode,
            DataMode,
            PrimMode,
        },
        united::UniMode,
    },
    number::Number,
    pair::Pair,
    prelude::{
        PRELUDE,
        Prelude,
    },
    symbol::Symbol,
    text::Text,
    unit::Unit,
    val::func::{
        FuncVal,
        const_cell_comp::ConstCellCompFuncVal,
        mut_cell_comp::MutCellCompFuncVal,
    },
};

pub(crate) trait Arbitrary {
    fn any(rng: &mut SmallRng, depth: usize) -> Self;
}

impl Arbitrary for Val {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
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
            8 => Val::Call(Call::<Val, Val>::any(rng, new_depth).into()),
            9 => Val::List(List::<Val>::any(rng, new_depth).into()),
            10 => Val::Map(Map::<Val, Val>::any(rng, new_depth).into()),
            11 => Val::Ctx(Ctx::any(rng, new_depth).into()),
            12 => Val::Func(FuncVal::any(rng, new_depth)),
            13 => Val::Ext(Arbitrary::any(rng, new_depth)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for Unit {
    fn any(_rng: &mut SmallRng, _depth: usize) -> Self {
        Unit
    }
}

impl Arbitrary for Bit {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        Bit::new(rng.random())
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
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        let len = any_len(rng);
        let s = DistSymbol.sample_string(rng, len);
        Symbol::from_string(s)
    }
}

impl Arbitrary for Text {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        let len = any_len(rng);
        let s: String = rng.sample_iter::<char, _>(StandardUniform).take(len).collect();
        Text::from(s)
    }
}

impl Arbitrary for Int {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        Int::from(rng.random::<i128>())
    }
}

impl Arbitrary for Number {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        let int: i64 = rng.random();
        let int = BigInt::from(int);
        let exp: i8 = rng.random();
        let exp = BigInt::from(exp);
        Number::new(int, 10, exp)
    }
}

impl Arbitrary for Byte {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
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
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        Pair::new(First::any(rng, depth), Second::any(rng, depth))
    }
}

impl<This, That> Arbitrary for Either<This, That>
where
    This: Arbitrary,
    That: Arbitrary,
{
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
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
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        Change::new(From::any(rng, depth), To::any(rng, depth))
    }
}

impl<Func, Input> Arbitrary for Call<Func, Input>
where
    Func: Arbitrary,
    Input: Arbitrary,
{
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        Call::new(Func::any(rng, depth), Input::any(rng, depth))
    }
}

impl<T> Arbitrary for List<T>
where T: Arbitrary
{
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
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
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let len = any_len_weighted(rng, depth);
        let mut map = Map::with_capacity(len);
        for _ in 0 .. len {
            map.insert(K::any(rng, depth), V::any(rng, depth));
        }
        map
    }
}

impl Arbitrary for VarAccess {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        const ACCESSES: [VarAccess; 3] = [VarAccess::Assign, VarAccess::Mut, VarAccess::Const];
        *(ACCESSES.choose(rng).unwrap())
    }
}

impl Arbitrary for CtxValue {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        CtxValue {
            val: Val::any(rng, depth),
            access: VarAccess::any(rng, depth),
            static1: rng.random(),
        }
    }
}

impl Arbitrary for Ctx {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let variables = Map::any(rng, depth);
        let variables = CtxMap::new(variables, rng.random());
        let solver = if rng.random() { Some(Arbitrary::any(rng, depth)) } else { None };
        Ctx::new(variables, solver)
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        if rng.random() { None } else { Some(T::any(rng, depth)) }
    }
}

impl Arbitrary for DataMode {
    fn any(_rng: &mut SmallRng, _depth: usize) -> Self {
        DataMode
    }
}

impl Arbitrary for CodeMode {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        const MODES: [CodeMode; 2] = [CodeMode::Form, CodeMode::Eval];
        *(MODES.choose(rng).unwrap())
    }
}

impl Arbitrary for SymbolMode {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        const MODES: [SymbolMode; 3] = [SymbolMode::Literal, SymbolMode::Ref, SymbolMode::Move];
        *(MODES.choose(rng).unwrap())
    }
}

impl Arbitrary for UniMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let code = CodeMode::any(rng, depth);
        let symbol = SymbolMode::any(rng, depth);
        UniMode::new(code, symbol)
    }
}

impl Arbitrary for CompMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let symbol = Arbitrary::any(rng, depth);
        let pair = Arbitrary::any(rng, depth);
        let call = Arbitrary::any(rng, depth);
        let list = Arbitrary::any(rng, depth);
        let map = Arbitrary::any(rng, depth);
        CompMode { symbol, pair, call, list, map }
    }
}

impl Arbitrary for PairMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let first = Arbitrary::any(rng, new_depth);
        let second = Arbitrary::any(rng, new_depth);
        PairMode { first, second }
    }
}

impl Arbitrary for CallMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let code = CodeMode::any(rng, depth);
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let input = Arbitrary::any(rng, new_depth);
        CallMode { code, func, input }
    }
}

impl Arbitrary for ListMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let head = Arbitrary::any(rng, new_depth);
        let tail = Arbitrary::any(rng, new_depth);
        ListMode { head, tail }
    }
}

impl Arbitrary for MapMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let some = Arbitrary::any(rng, new_depth);
        let key = Arbitrary::any(rng, new_depth);
        let value = Arbitrary::any(rng, new_depth);
        let else1 = Pair::new(key, value);
        MapMode { some, else1 }
    }
}

impl Arbitrary for PrimMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let symbol = Arbitrary::any(rng, depth);
        let pair = Arbitrary::any(rng, depth);
        let call = Arbitrary::any(rng, depth);
        let list = Arbitrary::any(rng, depth);
        let map = Arbitrary::any(rng, depth);
        PrimMode { symbol, pair, call, list, map }
    }
}

impl Arbitrary for Mode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // united
            weight, // primitive
            1,      // composite
            1,      // function
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;
        match i {
            0 => Mode::Uni(Arbitrary::any(rng, depth)),
            1 => Mode::Prim(Arbitrary::any(rng, depth)),
            2 => Mode::Comp(Box::new(Arbitrary::any(rng, depth))),
            3 => Mode::Func(FuncVal::any(rng, new_depth)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for FuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        if rng.random() {
            let prelude = PRELUDE.with(|prelude| {
                let mut m = Map::default();
                prelude.put(&mut m);
                m
            });
            let func = prelude
                .into_values()
                .filter(|v| matches!(v.val, Val::Func(_)))
                .choose(rng)
                .unwrap();
            let Val::Func(func) = func.val else { unreachable!() };
            func
        } else {
            match rng.random_range(0 .. 7) {
                0 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::Mode(func)
                }
                1 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::FreeStaticComp(func)
                }
                2 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::ConstStaticComp(func)
                }
                3 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::MutStaticComp(func)
                }
                4 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::FreeCellComp(func)
                }
                5 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::ConstCellComp(func)
                }
                6 => {
                    let func = Arbitrary::any(rng, depth);
                    FuncVal::MutCellComp(func)
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Arbitrary for FuncMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let call = Arbitrary::any(rng, depth);
        FuncMode { call }
    }
}

impl Arbitrary for Composite {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        Composite {
            body: Arbitrary::any(rng, depth),
            ctx: Arbitrary::any(rng, depth),
            input_name: Arbitrary::any(rng, depth),
        }
    }
}

impl Arbitrary for FreeCellCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = FreeCellCompFunc::new(composite, mode);
        FreeCellCompFuncVal::from(func)
    }
}

impl Arbitrary for FreeStaticCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = FreeStaticCompFunc::new(composite, mode);
        FreeStaticCompFuncVal::from(func)
    }
}

impl Arbitrary for ConstCellCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let ctx_name = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = ConstCellCompFunc::new(composite, ctx_name, mode);
        ConstCellCompFuncVal::from(func)
    }
}

impl Arbitrary for ConstStaticCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let ctx_name = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = ConstStaticCompFunc::new(composite, ctx_name, mode);
        ConstStaticCompFuncVal::from(func)
    }
}

impl Arbitrary for MutCellCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let ctx_name = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = MutCellCompFunc::new(composite, ctx_name, mode);
        MutCellCompFuncVal::from(func)
    }
}

impl Arbitrary for MutStaticCompFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let composite = Arbitrary::any(rng, depth);
        let ctx_name = Arbitrary::any(rng, depth);
        let mode = Arbitrary::any(rng, depth);
        let func = MutStaticCompFunc::new(composite, ctx_name, mode);
        MutStaticCompFuncVal::from(func)
    }
}

impl Arbitrary for ModeFuncVal {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let mode = Arbitrary::any(rng, depth);
        ModeFuncVal::from(ModeFunc::new(mode))
    }
}

impl Arbitrary for Box<dyn ValExt> {
    fn any(_rng: &mut SmallRng, _depth: usize) -> Self {
        Box::new(UnitExt)
    }
}

fn sample<const N: usize>(rng: &mut SmallRng, weights: [usize; N]) -> usize {
    let dist = WeightedIndex::new(weights).unwrap();
    dist.sample(rng)
}

fn any_len_weighted(rng: &mut SmallRng, depth: usize) -> usize {
    const WEIGHTS: [usize; 16] = [16, 16, 16, 16, 4, 4, 4, 4, 1, 1, 1, 1, 1, 1, 1, 1];
    let dist = WeightedIndex::new(WEIGHTS).unwrap();
    let limit = 16usize.saturating_sub(depth);
    let len = dist.sample(rng);
    min(len, limit)
}

fn any_len(rng: &mut SmallRng) -> usize {
    let len: u8 = rng.random();
    len as usize
}
