use std::cmp::min;

use num_bigint::BigInt;
use rand::{
    Rng,
    distributions::{
        DistString,
        Standard,
        WeightedIndex,
    },
    prelude::{
        Distribution,
        IteratorRandom,
        SliceRandom,
        SmallRng,
    },
};

use crate::{
    Abstract,
    AbstractMode,
    Ask,
    AskMode,
    Cache,
    Call,
    CallMode,
    Case,
    CaseVal,
    ConstStaticCompFunc,
    ConstStaticCompFuncVal,
    FreeCellCompFunc,
    FreeCellCompFuncVal,
    FreeCtx,
    FreeStaticCompFunc,
    FreeStaticCompFuncVal,
    ModeFunc,
    ModeFuncVal,
    MutStaticCompFunc,
    MutStaticCompFuncVal,
    SelfMode,
    SymbolMode,
    Val,
    ValExt,
    answer::Answer,
    bit::Bit,
    byte::Byte,
    ctx::{
        Ctx,
        CtxValue,
        Invariant,
        map::CtxMap,
    },
    extension::UnitExt,
    func::{
        FuncMode,
        comp::Composite,
        const_cell_comp::ConstCellCompFunc,
        mut_cell_comp::MutCellCompFunc,
    },
    int::Int,
    list::List,
    map::Map,
    mode::{
        Mode,
        composite::CompositeMode,
        list::ListMode,
        map::MapMode,
        pair::PairMode,
        primitive::PrimitiveMode,
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
        1,      // call
        1,      // abstract
        1,      // ask
        1,      // list
        1,      // map
        1,      // ctx
        1,      // func
        1,      // case
        1,      // answer
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
        8 => Val::Call(any_call(rng, new_depth).into()),
        9 => Val::Abstract(any_abstract(rng, new_depth).into()),
        10 => Val::Ask(any_ask(rng, new_depth).into()),
        11 => Val::List(any_list(rng, new_depth).into()),
        12 => Val::Map(any_map(rng, new_depth).into()),
        13 => Val::Ctx(any_ctx(rng, new_depth).into()),
        14 => Val::Func(any_func(rng, new_depth)),
        15 => Val::Case(any_case_val(rng, new_depth)),
        16 => Val::Answer(any_answer(rng, new_depth).into()),
        17 => Val::Ext(any_extension(rng, new_depth)),
        _ => unreachable!(),
    }
}

pub(crate) fn any_unit(_rng: &mut SmallRng) -> Unit {
    Unit
}

pub(crate) fn any_bit(rng: &mut SmallRng) -> Bit {
    Bit::new(rng.gen())
}

struct DistSymbol;

impl Distribution<u8> for DistSymbol {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        rng.gen_range(Symbol::MIN..=Symbol::MAX) as u8
    }
}

impl DistString for DistSymbol {
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
    let s: String = rng.sample_iter::<char, _>(Standard).take(len).collect();
    Text::from(s)
}

pub(crate) fn any_int(rng: &mut SmallRng) -> Int {
    Int::from(rng.gen::<i128>())
}

pub(crate) fn any_number(rng: &mut SmallRng) -> Number {
    let int: i64 = rng.gen();
    let int = BigInt::from(int);
    let exp: i8 = rng.gen();
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

pub(crate) fn any_call(rng: &mut SmallRng, depth: usize) -> Call<Val, Val> {
    Call::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_abstract(rng: &mut SmallRng, depth: usize) -> Abstract<Val, Val> {
    Abstract::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_ask(rng: &mut SmallRng, depth: usize) -> Ask<Val, Val> {
    Ask::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_list(rng: &mut SmallRng, depth: usize) -> List<Val> {
    let len = any_len_weighted(rng, depth);
    let mut list = Vec::with_capacity(len);
    for _ in 0..len {
        list.push(any_val(rng, depth));
    }
    List::from(list)
}

pub(crate) fn any_map(rng: &mut SmallRng, depth: usize) -> Map<Val, Val> {
    let len = any_len_weighted(rng, depth);
    let mut map = Map::with_capacity(len);
    for _ in 0..len {
        map.insert(any_val(rng, depth), any_val(rng, depth));
    }
    map
}

pub(crate) fn any_invariant(rng: &mut SmallRng) -> Invariant {
    const INVARIANTS: [Invariant; 3] = [Invariant::None, Invariant::Final, Invariant::Const];
    *(INVARIANTS.choose(rng).unwrap())
}

pub(crate) fn any_ctx_map(rng: &mut SmallRng, depth: usize) -> Map<Symbol, CtxValue> {
    let len = any_len_weighted(rng, depth);
    let mut ctx_map = Map::with_capacity(len);
    for _ in 0..len {
        let ctx_value = CtxValue {
            val: any_val(rng, depth),
            invariant: any_invariant(rng),
        };
        ctx_map.insert(any_symbol(rng), ctx_value);
    }
    ctx_map
}

pub(crate) fn any_ctx(rng: &mut SmallRng, depth: usize) -> Ctx {
    let variables = any_ctx_map(rng, depth);
    let variables = CtxMap::new(variables, rng.gen());
    let solver = if rng.gen() {
        Some(any_func(rng, depth))
    } else {
        None
    };
    Ctx::new(variables, solver)
}

pub(crate) fn any_primitive_mode(rng: &mut SmallRng) -> PrimitiveMode {
    const MODES: [PrimitiveMode; 3] = [PrimitiveMode::Eval, PrimitiveMode::Id, PrimitiveMode::Form];
    *(MODES.choose(rng).unwrap())
}

pub(crate) fn any_composite_mode<M: Arbitrary>(
    rng: &mut SmallRng,
    depth: usize,
) -> CompositeMode<M> {
    let symbol = any_symbol_mode(rng);
    let pair = any_pair_mode(rng, depth);
    let call = any_call_mode(rng, depth);
    let abstract1 = any_abstract_mode(rng, depth);
    let ask = any_ask_mode(rng, depth);
    let list = any_list_mode(rng, depth);
    let map = any_map_mode(rng, depth);
    CompositeMode {
        symbol,
        pair,
        call,
        abstract1,
        ask,
        list,
        map,
    }
}

pub(crate) fn any_symbol_mode(rng: &mut SmallRng) -> SymbolMode {
    any_primitive_mode(rng).into()
}

pub(crate) fn any_pair_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> PairMode<M> {
    PairMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for PairMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
        ];
        let i = sample(rng, weights);
        match i {
            0 => PairMode::Id,
            1 => {
                let new_depth = depth + 1;
                let pair = Pair::new(M::any(rng, new_depth), M::any(rng, new_depth));
                PairMode::Form(pair)
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_call_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> CallMode<M> {
    CallMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for CallMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
            1,      // eval
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;
        match i {
            0 => CallMode::Id,
            1 => {
                let call = Call::new(M::any(rng, new_depth), M::any(rng, new_depth));
                CallMode::Form(call)
            }
            2 => {
                let call = Call::new(M::any(rng, new_depth), M::any(rng, new_depth));
                CallMode::Eval(call)
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_abstract_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> AbstractMode<M> {
    AbstractMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for AbstractMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
            1,      // eval
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;
        match i {
            0 => AbstractMode::Id,
            1 => {
                let abstract1 = Abstract::new(M::any(rng, new_depth), M::any(rng, new_depth));
                AbstractMode::Form(abstract1)
            }
            2 => {
                let abstract1 = Abstract::new(M::any(rng, new_depth), M::any(rng, new_depth));
                AbstractMode::Eval(abstract1)
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_ask_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> AskMode<M> {
    AskMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for AskMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
            1,      // eval
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;
        match i {
            0 => AskMode::Id,
            1 => {
                let ask = Ask::new(M::any(rng, new_depth), M::any(rng, new_depth));
                AskMode::Form(ask)
            }
            2 => {
                let ask = Ask::new(M::any(rng, new_depth), M::any(rng, new_depth));
                AskMode::Eval(ask)
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_list_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> ListMode<M> {
    ListMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for ListMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
        ];
        let i = sample(rng, weights);
        match i {
            0 => ListMode::Id,
            1 => {
                let new_depth = depth + 1;
                let head_size = any_len_weighted(rng, depth);
                let mut head = Vec::with_capacity(head_size);
                for _ in 0..head_size {
                    head.push(M::any(rng, new_depth));
                }
                let head = List::from(head);
                let tail = M::any(rng, new_depth);
                ListMode::Form { head, tail }
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_map_mode<M: Arbitrary>(rng: &mut SmallRng, depth: usize) -> MapMode<M> {
    MapMode::any(rng, depth)
}

impl<M: Arbitrary> Arbitrary for MapMode<M> {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // id
            1,      // form
        ];
        let i = sample(rng, weights);
        match i {
            0 => MapMode::Id,
            1 => {
                let new_depth = depth + 1;
                let len = any_len_weighted(rng, new_depth);
                let mut some = Map::with_capacity(len);
                for _ in 0..len {
                    some.insert(any_val(rng, new_depth), M::any(rng, new_depth));
                }
                let else1 = Pair::new(M::any(rng, new_depth), M::any(rng, new_depth));
                MapMode::Form { some, else1 }
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn any_mode(rng: &mut SmallRng, depth: usize) -> Mode {
    Mode::any(rng, depth)
}

impl Arbitrary for Mode {
    fn any(rng: &mut SmallRng, depth: usize) -> Self {
        let weight: usize = 1 << min(depth, 32);
        let weights = [
            weight, // primitive
            weight, // recursive
            1,      // composite
        ];
        let i = sample(rng, weights);
        let new_depth = depth + 1;
        match i {
            0 => Mode::Primitive(any_primitive_mode(rng)),
            1 => Mode::Recursive(any_composite_mode::<SelfMode>(rng, new_depth)),
            2 => Mode::Composite(Box::new(any_composite_mode::<Mode>(rng, new_depth))),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for SelfMode {
    fn any(rng: &mut SmallRng, _depth: usize) -> Self {
        if rng.gen() {
            Self::Self1
        } else {
            Self::Primitive(any_primitive_mode(rng))
        }
    }
}

pub(crate) fn any_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.gen() {
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
        let Val::Func(func) = func.val else {
            unreachable!()
        };
        func
    } else {
        match rng.gen_range(0..7) {
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
    let call = any_mode(rng, depth);
    let abstract1 = any_mode(rng, depth);
    let ask = any_mode(rng, depth);
    FuncMode {
        call,
        abstract1,
        ask,
    }
}

fn any_composite(rng: &mut SmallRng, depth: usize) -> Composite {
    Composite {
        body_mode: any_mode(rng, depth),
        body: any_val(rng, depth),
        ctx: any_ctx(rng, depth),
        input_name: any_symbol(rng),
    }
}

pub(crate) fn any_free_cell_comp_func(rng: &mut SmallRng, depth: usize) -> FreeCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = FreeCellCompFunc::new(composite, mode, cacheable);
    FreeCellCompFuncVal::from(func)
}

pub(crate) fn any_free_static_comp_func(rng: &mut SmallRng, depth: usize) -> FreeStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = FreeStaticCompFunc::new(composite, mode, cacheable);
    FreeStaticCompFuncVal::from(func)
}

pub(crate) fn any_const_cell_comp_func(rng: &mut SmallRng, depth: usize) -> ConstCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = ConstCellCompFunc::new(composite, ctx_name, mode, cacheable);
    ConstCellCompFuncVal::from(func)
}

pub(crate) fn any_const_static_comp_func(
    rng: &mut SmallRng,
    depth: usize,
) -> ConstStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = ConstStaticCompFunc::new(composite, ctx_name, mode, cacheable);
    ConstStaticCompFuncVal::from(func)
}

pub(crate) fn any_mut_cell_comp_func(rng: &mut SmallRng, depth: usize) -> MutCellCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = MutCellCompFunc::new(composite, ctx_name, mode, cacheable);
    MutCellCompFuncVal::from(func)
}

pub(crate) fn any_mut_static_comp_func(rng: &mut SmallRng, depth: usize) -> MutStaticCompFuncVal {
    let composite = any_composite(rng, depth);
    let ctx_name = any_symbol(rng);
    let mode = any_func_mode(rng, depth);
    let cacheable = rng.gen();
    let func = MutStaticCompFunc::new(composite, ctx_name, mode, cacheable);
    MutStaticCompFuncVal::from(func)
}

pub(crate) fn any_mode_func(rng: &mut SmallRng, depth: usize) -> ModeFuncVal {
    let mode = any_mode(rng, depth);
    ModeFuncVal::from(ModeFunc::new(mode))
}

pub(crate) fn any_case_val(rng: &mut SmallRng, depth: usize) -> CaseVal {
    if rng.gen() {
        CaseVal::Trivial(any_case(rng, depth).into())
    } else {
        CaseVal::Cache(any_cache(rng, depth).into())
    }
}

pub(crate) fn any_case(rng: &mut SmallRng, depth: usize) -> Case<Val, Val, Val> {
    let func = any_val(rng, depth);
    let input = any_val(rng, depth);
    let output = any_val(rng, depth);
    Case::new(func, input, output)
}

pub(crate) fn any_cache(rng: &mut SmallRng, depth: usize) -> Cache<Val, Val, Val> {
    let func = any_func(rng, depth);
    let input = any_val(rng, depth);
    Cache::new(FreeCtx, func, input)
}

pub(crate) fn any_answer(rng: &mut SmallRng, depth: usize) -> Answer {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // none
        weight, // never
        1,      // maybe
        1,      // cache
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => Answer::None,
        1 => Answer::Never,
        2 => Answer::Maybe(any_val(rng, new_depth)),
        3 => {
            let cache = any_cache(rng, new_depth);
            Answer::Cache(cache.into())
        }
        _ => unreachable!(),
    }
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
    let len: u8 = rng.gen();
    len as usize
}
