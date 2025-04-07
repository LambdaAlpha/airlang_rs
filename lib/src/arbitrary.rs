use std::cmp::min;

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
    Abstract,
    AbstractMode,
    Call,
    CallMode,
    Change,
    ConstStaticCompFunc,
    ConstStaticCompFuncVal,
    Equiv,
    EquivMode,
    FreeCellCompFunc,
    FreeCellCompFuncVal,
    FreeStaticCompFunc,
    FreeStaticCompFuncVal,
    Generate,
    GenerateMode,
    Inverse,
    InverseMode,
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
    ctx::{
        Ctx,
        map::{
            CtxMap,
            CtxValue,
            VarAccess,
        },
    },
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
        change::ChangeMode,
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

pub(crate) fn any_val(rng: &mut SmallRng, depth: usize) -> Val {
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
        1,      // change
        1,      // call
        1,      // equiv
        1,      // inverse
        1,      // generate
        1,      // abstract
        1,      // list
        1,      // map
        1,      // ctx
        1,      // func
        1,      // extension
    ];
    let i = sample(rng, weights);
    let new_depth = depth + 1;

    match i {
        0 => Val::Unit(any_unit(rng)),
        1 => Val::Bit(any_bit(rng)),
        2 => Val::Symbol(any_symbol(rng)),
        3 => Val::Text(any_text(rng).into()),
        4 => Val::Int(any_int(rng).into()),
        5 => Val::Number(any_number(rng).into()),
        6 => Val::Byte(any_byte(rng).into()),
        7 => Val::Pair(any_pair(rng, new_depth).into()),
        8 => Val::Change(any_change(rng, new_depth).into()),
        9 => Val::Call(any_call(rng, new_depth).into()),
        10 => Val::Equiv(any_equiv(rng, new_depth).into()),
        11 => Val::Inverse(any_inverse(rng, new_depth).into()),
        12 => Val::Generate(any_generate(rng, new_depth).into()),
        13 => Val::Abstract(any_abstract(rng, new_depth).into()),
        14 => Val::List(any_list(rng, new_depth).into()),
        15 => Val::Map(any_map(rng, new_depth).into()),
        16 => Val::Ctx(any_ctx(rng, new_depth).into()),
        17 => Val::Func(any_func(rng, new_depth)),
        18 => Val::Ext(any_extension(rng, new_depth)),
        _ => unreachable!(),
    }
}

pub(crate) fn any_unit(_rng: &mut SmallRng) -> Unit {
    Unit
}

pub(crate) fn any_bit(rng: &mut SmallRng) -> Bit {
    Bit::new(rng.random())
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

pub(crate) fn any_symbol(rng: &mut SmallRng) -> Symbol {
    let len = any_len(rng);
    let s = DistSymbol.sample_string(rng, len);
    Symbol::from_string(s)
}

pub(crate) fn any_text(rng: &mut SmallRng) -> Text {
    let len = any_len(rng);
    let s: String = rng.sample_iter::<char, _>(StandardUniform).take(len).collect();
    Text::from(s)
}

pub(crate) fn any_int(rng: &mut SmallRng) -> Int {
    Int::from(rng.random::<i128>())
}

pub(crate) fn any_number(rng: &mut SmallRng) -> Number {
    let int: i64 = rng.random();
    let int = BigInt::from(int);
    let exp: i8 = rng.random();
    let exp = BigInt::from(exp);
    Number::new(int, 10, exp)
}

pub(crate) fn any_byte(rng: &mut SmallRng) -> Byte {
    let len = any_len(rng);
    let mut byte = vec![0u8; len];
    rng.fill(&mut *byte);
    Byte::from(byte)
}

pub(crate) fn any_pair(rng: &mut SmallRng, depth: usize) -> Pair<Val, Val> {
    Pair::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_change(rng: &mut SmallRng, depth: usize) -> Change<Val, Val> {
    Change::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_call(rng: &mut SmallRng, depth: usize) -> Call<Val, Val> {
    Call::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_equiv(rng: &mut SmallRng, depth: usize) -> Equiv<Val> {
    Equiv::new(any_val(rng, depth))
}

pub(crate) fn any_inverse(rng: &mut SmallRng, depth: usize) -> Inverse<Val> {
    Inverse::new(any_val(rng, depth))
}

pub(crate) fn any_generate(rng: &mut SmallRng, depth: usize) -> Generate<Val> {
    Generate::new(any_val(rng, depth))
}

pub(crate) fn any_abstract(rng: &mut SmallRng, depth: usize) -> Abstract<Val> {
    Abstract::new(any_val(rng, depth))
}

pub(crate) fn any_list(rng: &mut SmallRng, depth: usize) -> List<Val> {
    let len = any_len_weighted(rng, depth);
    let mut list = Vec::with_capacity(len);
    for _ in 0 .. len {
        list.push(any_val(rng, depth));
    }
    List::from(list)
}

pub(crate) fn any_map(rng: &mut SmallRng, depth: usize) -> Map<Val, Val> {
    let len = any_len_weighted(rng, depth);
    let mut map = Map::with_capacity(len);
    for _ in 0 .. len {
        map.insert(any_val(rng, depth), any_val(rng, depth));
    }
    map
}

pub(crate) fn any_var_access(rng: &mut SmallRng) -> VarAccess {
    const ACCESSES: [VarAccess; 3] = [VarAccess::Assign, VarAccess::Mut, VarAccess::Const];
    *(ACCESSES.choose(rng).unwrap())
}

pub(crate) fn any_ctx_map(rng: &mut SmallRng, depth: usize) -> Map<Symbol, CtxValue> {
    let len = any_len_weighted(rng, depth);
    let mut ctx_map = Map::with_capacity(len);
    for _ in 0 .. len {
        let ctx_value = CtxValue {
            val: any_val(rng, depth),
            access: any_var_access(rng),
            static1: rng.random(),
        };
        ctx_map.insert(any_symbol(rng), ctx_value);
    }
    ctx_map
}

pub(crate) fn any_ctx(rng: &mut SmallRng, depth: usize) -> Ctx {
    let variables = any_ctx_map(rng, depth);
    let variables = CtxMap::new(variables, rng.random());
    let solver = if rng.random() { Some(any_func(rng, depth)) } else { None };
    Ctx::new(variables, solver)
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
        let change = Arbitrary::any(rng, depth);
        let call = Arbitrary::any(rng, depth);
        let equiv = Arbitrary::any(rng, depth);
        let inverse = Arbitrary::any(rng, depth);
        let generate = Arbitrary::any(rng, depth);
        let abstract1 = Arbitrary::any(rng, depth);
        let list = Arbitrary::any(rng, depth);
        let map = Arbitrary::any(rng, depth);
        CompMode { symbol, pair, change, call, equiv, inverse, generate, abstract1, list, map }
    }
}

impl Arbitrary for PairMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let first = Arbitrary::any(rng, new_depth);
        let second = Arbitrary::any(rng, new_depth);
        let pair = Pair::new(first, second);
        PairMode { pair }
    }
}

impl Arbitrary for ChangeMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let from = Arbitrary::any(rng, new_depth);
        let to = Arbitrary::any(rng, new_depth);
        let change = Change::new(from, to);
        ChangeMode { change }
    }
}

impl Arbitrary for CallMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let code = CodeMode::any(rng, depth);
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let input = Arbitrary::any(rng, new_depth);
        let call = Call::new(func, input);
        CallMode { code, call }
    }
}

impl Arbitrary for EquivMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let equiv = Equiv::new(func);
        EquivMode { equiv }
    }
}

impl Arbitrary for InverseMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let inverse = Inverse::new(func);
        InverseMode { inverse }
    }
}

impl Arbitrary for GenerateMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let func = Arbitrary::any(rng, new_depth);
        let generate = Generate::new(func);
        GenerateMode { generate }
    }
}

impl Arbitrary for AbstractMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let value = Arbitrary::any(rng, new_depth);
        let abstract1 = Abstract::new(value);
        AbstractMode { abstract1 }
    }
}

impl Arbitrary for ListMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let head_size = any_len_weighted(rng, depth);
        let mut head = Vec::with_capacity(head_size);
        for _ in 0 .. head_size {
            head.push(Arbitrary::any(rng, new_depth));
        }
        let head = List::from(head);
        let tail = Arbitrary::any(rng, new_depth);
        ListMode { head, tail }
    }
}

impl Arbitrary for MapMode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let new_depth = depth + 1;
        let len = any_len_weighted(rng, new_depth);
        let mut some = Map::with_capacity(len);
        for _ in 0 .. len {
            some.insert(any_val(rng, new_depth), Arbitrary::any(rng, new_depth));
        }
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
        let change = Arbitrary::any(rng, depth);
        let call = Arbitrary::any(rng, depth);
        let equiv = Arbitrary::any(rng, depth);
        let inverse = Arbitrary::any(rng, depth);
        let generate = Arbitrary::any(rng, depth);
        let abstract1 = Arbitrary::any(rng, depth);
        let list = Arbitrary::any(rng, depth);
        let map = Arbitrary::any(rng, depth);
        PrimMode { symbol, pair, change, call, equiv, inverse, generate, abstract1, list, map }
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
            3 => Mode::Func(any_func(rng, new_depth)),
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.random() {
        let prelude = PRELUDE.with(|prelude| {
            let mut m = Map::default();
            prelude.put(&mut m);
            m
        });
        let func =
            prelude.into_values().filter(|v| matches!(v.val, Val::Func(_))).choose(rng).unwrap();
        let Val::Func(func) = func.val else { unreachable!() };
        func
    } else {
        match rng.random_range(0 .. 7) {
            0 => {
                let func = any_mode_func(rng, depth);
                FuncVal::Mode(func)
            }
            1 => {
                let func = any_free_static_comp_func(rng, depth);
                FuncVal::FreeStaticComp(func)
            }
            2 => {
                let func = any_const_static_comp_func(rng, depth);
                FuncVal::ConstStaticComp(func)
            }
            3 => {
                let func = any_mut_static_comp_func(rng, depth);
                FuncVal::MutStaticComp(func)
            }
            4 => {
                let func = any_free_cell_comp_func(rng, depth);
                FuncVal::FreeCellComp(func)
            }
            5 => {
                let func = any_const_cell_comp_func(rng, depth);
                FuncVal::ConstCellComp(func)
            }
            6 => {
                let func = any_mut_cell_comp_func(rng, depth);
                FuncVal::MutCellComp(func)
            }
            _ => unreachable!(),
        }
    }
}

fn any_func_mode(rng: &mut SmallRng, depth: usize) -> FuncMode {
    let call = Arbitrary::any(rng, depth);
    let equiv = Arbitrary::any(rng, depth);
    let inverse = Arbitrary::any(rng, depth);
    FuncMode { call, equiv, inverse }
}

fn any_composite(rng: &mut SmallRng, depth: usize) -> Composite {
    Composite { body: any_val(rng, depth), ctx: any_ctx(rng, depth), input_name: any_symbol(rng) }
}

pub(crate) fn any_free_cell_comp_func(rng: &mut SmallRng, depth: usize) -> FreeCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let mode = any_func_mode(rng, depth);
    let func = FreeCellCompFunc::new(composite, mode);
    FreeCellCompFuncVal::from(func)
}

pub(crate) fn any_free_static_comp_func(rng: &mut SmallRng, depth: usize) -> FreeStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let mode = any_func_mode(rng, depth);
    let func = FreeStaticCompFunc::new(composite, mode);
    FreeStaticCompFuncVal::from(func)
}

pub(crate) fn any_const_cell_comp_func(rng: &mut SmallRng, depth: usize) -> ConstCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let func = ConstCellCompFunc::new(composite, ctx_name, mode);
    ConstCellCompFuncVal::from(func)
}

pub(crate) fn any_const_static_comp_func(
    rng: &mut SmallRng, depth: usize,
) -> ConstStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let func = ConstStaticCompFunc::new(composite, ctx_name, mode);
    ConstStaticCompFuncVal::from(func)
}

pub(crate) fn any_mut_cell_comp_func(rng: &mut SmallRng, depth: usize) -> MutCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let func = MutCellCompFunc::new(composite, ctx_name, mode);
    MutCellCompFuncVal::from(func)
}

pub(crate) fn any_mut_static_comp_func(rng: &mut SmallRng, depth: usize) -> MutStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let func = MutStaticCompFunc::new(composite, ctx_name, mode);
    MutStaticCompFuncVal::from(func)
}

pub(crate) fn any_mode_func(rng: &mut SmallRng, depth: usize) -> ModeFuncVal {
    let mode = Arbitrary::any(rng, depth);
    ModeFuncVal::from(ModeFunc::new(mode))
}

pub(crate) fn any_extension(_rng: &mut SmallRng, _depth: usize) -> Box<dyn ValExt> {
    Box::new(UnitExt)
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
