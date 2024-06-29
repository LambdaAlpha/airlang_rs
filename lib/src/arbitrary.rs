use std::cmp::min;

use num_bigint::BigInt;
use rand::{
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
    Rng,
};

use crate::{
    answer::Answer,
    bool::Bool,
    byte::Byte,
    ctx::{
        Ctx,
        CtxMap,
        CtxValue,
        Invariant,
    },
    extension::UnitExt,
    func::{
        Composed,
        CtxConstInfo,
        CtxFreeInfo,
        CtxMutableInfo,
        Func,
        FuncImpl,
        FuncTransformer,
    },
    int::Int,
    list::List,
    map::Map,
    mode::{
        ListItemMode,
        Mode,
    },
    number::Number,
    pair::Pair,
    prelude::{
        Prelude,
        PRELUDE,
    },
    symbol::Symbol,
    text::Text,
    transform::Transform,
    unit::Unit,
    val::func::FuncVal,
    Ask,
    Cache,
    Call,
    Case,
    CaseVal,
    Comment,
    FreeCtx,
    ListMode,
    MapMode,
    PairMode,
    Val,
    ValExt,
    ValMode,
};

pub(crate) fn any_val(rng: &mut SmallRng, depth: usize) -> Val {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // unit
        weight, // bool
        weight, // symbol
        weight, // int
        weight, // number
        weight, // byte
        weight, // text
        1,      // pair
        1,      // call
        1,      // ask
        1,      // comment
        1,      // list
        1,      // map
        1,      // ctx
        1,      // func
        1,      // case
        1,      // answer
        1,      // extension
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => Val::Unit(any_unit(rng)),
        1 => Val::Bool(any_bool(rng)),
        2 => Val::Symbol(any_symbol(rng)),
        3 => Val::Int(any_int(rng).into()),
        4 => Val::Number(any_number(rng).into()),
        5 => Val::Byte(any_byte(rng).into()),
        6 => Val::Text(any_text(rng).into()),
        7 => Val::Pair(any_pair(rng, new_depth).into()),
        8 => Val::Call(any_call(rng, new_depth).into()),
        9 => Val::Ask(any_ask(rng, new_depth).into()),
        10 => Val::Comment(any_comment(rng, new_depth).into()),
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

pub(crate) fn any_bool(rng: &mut SmallRng) -> Bool {
    Bool::new(rng.gen())
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

pub(crate) fn any_pair(rng: &mut SmallRng, depth: usize) -> Pair<Val, Val> {
    Pair::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_call(rng: &mut SmallRng, depth: usize) -> Call<Val, Val> {
    Call::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_ask(rng: &mut SmallRng, depth: usize) -> Ask<Val, Val> {
    Ask::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_comment(rng: &mut SmallRng, depth: usize) -> Comment<Val, Val> {
    Comment::new(any_val(rng, depth), any_val(rng, depth))
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

pub(crate) fn any_ctx(rng: &mut SmallRng, depth: usize) -> Ctx {
    let len = any_len_weighted(rng, depth);
    let mut ctx_map = Map::with_capacity(len);
    for _ in 0..len {
        let ctx_value = CtxValue {
            val: any_val(rng, depth),
            invariant: any_invariant(rng),
        };
        ctx_map.insert(any_symbol(rng), ctx_value);
    }
    let meta = if rng.gen_bool(0.1) {
        Some(Box::new(any_ctx(rng, depth)))
    } else {
        None
    };
    Ctx::new(ctx_map, meta)
}

pub(crate) fn any_transform(rng: &mut SmallRng) -> Transform {
    const TRANSFORMS: [Transform; 3] = [Transform::Eval, Transform::Id, Transform::Form];
    *(TRANSFORMS.choose(rng).unwrap())
}

pub(crate) fn any_val_mode(rng: &mut SmallRng, depth: usize) -> ValMode {
    let pair = any_pair_mode(rng, depth);
    let list = any_list_mode(rng, depth);
    let map = any_map_mode(rng, depth);
    ValMode { pair, list, map }
}

pub(crate) fn any_pair_mode(rng: &mut SmallRng, depth: usize) -> PairMode {
    let new_depth = depth + 1;
    Pair::new(any_mode(rng, new_depth), any_mode(rng, new_depth))
}

pub(crate) fn any_list_mode(rng: &mut SmallRng, depth: usize) -> ListMode {
    let new_depth = depth + 1;

    match rng.gen_range(0..2) {
        0 => ListMode::All(any_mode(rng, new_depth)),
        1 => {
            let left = any_len_weighted(rng, depth) >> 1;
            let right = any_len_weighted(rng, depth) >> 1;
            let mut list = Vec::with_capacity(left + 1 + right);
            for _ in 0..left {
                list.push(ListItemMode {
                    ellipsis: false,
                    mode: any_mode(rng, new_depth),
                });
            }
            if rng.gen() {
                list.push(ListItemMode {
                    ellipsis: true,
                    mode: any_mode(rng, new_depth),
                });
            }
            for _ in 0..right {
                list.push(ListItemMode {
                    ellipsis: false,
                    mode: any_mode(rng, new_depth),
                });
            }
            ListMode::Some(List::from(list))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn any_map_mode(rng: &mut SmallRng, depth: usize) -> MapMode {
    let new_depth = depth + 1;

    match rng.gen_range(0..2) {
        0 => MapMode::All(Pair::new(
            any_mode(rng, new_depth),
            any_mode(rng, new_depth),
        )),
        1 => {
            let len = any_len_weighted(rng, new_depth);
            let mut map = Map::with_capacity(len);
            for _ in 0..len {
                map.insert(any_val(rng, new_depth), any_mode(rng, new_depth));
            }
            MapMode::Some(map)
        }
        _ => unreachable!(),
    }
}

pub(crate) fn any_mode(rng: &mut SmallRng, depth: usize) -> Mode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // predefined
        1,      // custom
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    let default = any_transform(rng);
    let specialized = match i {
        0 => None,
        1 => Some(Box::new(any_val_mode(rng, new_depth))),
        _ => unreachable!(),
    };
    Mode {
        default,
        specialized,
    }
}

pub(crate) fn any_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.gen() {
        let prelude = PRELUDE.with(|prelude| {
            let mut m = CtxMap::default();
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
        let input_mode = any_mode(rng, depth);
        let output_mode = any_mode(rng, depth);
        let transformer = match rng.gen_range(0..3) {
            0 => FuncTransformer::Free(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                prelude: any_ctx(rng, depth),
                input_name: any_symbol(rng),
                ctx: CtxFreeInfo {},
            })),
            1 => FuncTransformer::Const(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                prelude: any_ctx(rng, depth),
                input_name: any_symbol(rng),
                ctx: CtxConstInfo {
                    name: any_symbol(rng),
                },
            })),
            2 => FuncTransformer::Mutable(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                prelude: any_ctx(rng, depth),
                input_name: any_symbol(rng),
                ctx: CtxMutableInfo {
                    name: any_symbol(rng),
                },
            })),
            _ => unreachable!(),
        };
        let func = Func {
            input_mode,
            output_mode,
            transformer,
        };
        FuncVal::from(func)
    }
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
