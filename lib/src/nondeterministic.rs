use std::{
    cmp::min,
    mem::variant_count,
};

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
    bool::Bool,
    bytes::Bytes,
    call::Call,
    ctx::{
        Ctx,
        InvariantTag,
        NameMap,
        TaggedVal,
    },
    eval_mode::EvalMode,
    extension::UnitExt,
    float::Float,
    func::{
        Composed,
        CtxConstInfo,
        CtxFreeInfo,
        CtxMutableInfo,
        Func,
        FuncEval,
        FuncImpl,
    },
    int::Int,
    io_mode::{
        IoMode,
        ListItemMode,
    },
    list::List,
    logic::Prop,
    map::Map,
    pair::Pair,
    prelude::{
        Prelude,
        PRELUDE,
    },
    problem::Verified,
    reverse::Reverse,
    string::Str,
    symbol::Symbol,
    types::refer::Reader,
    unit::Unit,
    val::{
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        pair::PairVal,
        prop::PropVal,
        reverse::ReverseVal,
    },
    Answer,
    AnswerVal,
    CallMode,
    ListMode,
    MapMode,
    MatchMode,
    PairMode,
    ReverseMode,
    Val,
    ValExt,
};

pub(crate) fn any_val(rng: &mut SmallRng, depth: usize) -> Val {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // unit
        weight, // bool
        weight, // int
        weight, // float
        weight, // bytes
        weight, // symbol
        weight, // string
        1,      // pair
        1,      // call
        1,      // reverse
        1,      // list
        1,      // map
        1,      // ctx
        1,      // func
        1,      // prop
        1,      // answer
        1,      // extension
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => Val::Unit(any_unit(rng)),
        1 => Val::Bool(any_bool(rng)),
        2 => Val::Int(any_int(rng)),
        3 => Val::Float(any_float(rng)),
        4 => Val::Bytes(any_bytes(rng)),
        5 => Val::Symbol(any_symbol(rng)),
        6 => Val::String(any_string(rng)),
        7 => Val::Pair(Box::new(any_pair(rng, new_depth))),
        8 => Val::Call(Box::new(any_call(rng, new_depth))),
        9 => Val::Reverse(Box::new(any_reverse(rng, new_depth))),
        10 => Val::List(any_list(rng, new_depth)),
        11 => Val::Map(any_map(rng, new_depth)),
        12 => Val::Ctx(any_ctx(rng, new_depth)),
        13 => Val::Func(any_func(rng, new_depth)),
        14 => Val::Prop(any_prop(rng, new_depth)),
        15 => Val::Answer(any_answer(rng, new_depth)),
        16 => Val::Ext(any_extension(rng, new_depth)),
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

pub(crate) fn any_float(rng: &mut SmallRng) -> Float {
    Float::from(rng.gen::<f64>())
}

pub(crate) fn any_bytes(rng: &mut SmallRng) -> Bytes {
    let len = any_len(rng);
    let mut bytes = vec![0u8; len];
    rng.fill(&mut *bytes);
    Bytes::from(bytes)
}

struct DistSymbol;

impl Distribution<u8> for DistSymbol {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHARSET: &[u8] = b"\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789\
            `~!@#$%^&*-_=+\\|;:'\"<.>/?\
        ";
        const RANGE: u32 = CHARSET.len() as u32;
        let i = rng.gen_range(0..RANGE);
        CHARSET[i as usize]
    }
}

impl DistString for DistSymbol {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
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

pub(crate) fn any_string(rng: &mut SmallRng) -> Str {
    let len = any_len(rng);
    let s: String = rng.sample_iter::<char, _>(Standard).take(len).collect();
    Str::from(s)
}

pub(crate) fn any_pair(rng: &mut SmallRng, depth: usize) -> PairVal {
    PairVal::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_call(rng: &mut SmallRng, depth: usize) -> CallVal {
    CallVal::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_reverse(rng: &mut SmallRng, depth: usize) -> ReverseVal {
    ReverseVal::new(any_val(rng, depth), any_val(rng, depth))
}

pub(crate) fn any_list(rng: &mut SmallRng, depth: usize) -> ListVal {
    let len = any_len_weighted(rng, depth);
    let mut list = Vec::with_capacity(len);
    for _ in 0..len {
        list.push(any_val(rng, depth));
    }
    List::from(list)
}

pub(crate) fn any_map(rng: &mut SmallRng, depth: usize) -> MapVal {
    let len = any_len_weighted(rng, depth);
    let mut map = Map::with_capacity(len);
    for _ in 0..len {
        map.insert(any_val(rng, depth), any_val(rng, depth));
    }
    map
}

pub(crate) fn any_invariant_tag(rng: &mut SmallRng) -> InvariantTag {
    const TAGS: [InvariantTag; 3] = [InvariantTag::None, InvariantTag::Final, InvariantTag::Const];
    *(TAGS.choose(rng).unwrap())
}

pub(crate) fn any_ctx(rng: &mut SmallRng, depth: usize) -> CtxVal {
    let len = any_len_weighted(rng, depth);
    let mut name_map = Map::with_capacity(len);
    for _ in 0..len {
        let tagged_val = TaggedVal {
            val: any_val(rng, depth),
            tag: any_invariant_tag(rng),
        };
        name_map.insert(any_symbol(rng), tagged_val);
    }
    let meta = if rng.gen_bool(0.1) {
        Some(any_ctx(rng, depth).0)
    } else {
        None
    };
    let ctx = Ctx { name_map, meta };
    CtxVal(Box::new(ctx))
}

pub(crate) fn any_eval_mode(rng: &mut SmallRng) -> EvalMode {
    const EVAL_MODES: [EvalMode; 3] = [EvalMode::Value, EvalMode::Eager, EvalMode::Lazy];
    *(EVAL_MODES.choose(rng).unwrap())
}

pub(crate) fn any_match_mode(rng: &mut SmallRng, depth: usize) -> MatchMode {
    let new_depth = depth + 1;
    let symbol = any_eval_mode(rng);
    let pair = Box::new(any_pair_mode(rng, new_depth));
    let call = Box::new(any_call_mode(rng, new_depth));
    let reverse = Box::new(any_reverse_mode(rng, new_depth));
    let list = Box::new(any_list_mode(rng, new_depth));
    let map = Box::new(any_map_mode(rng, new_depth));
    MatchMode {
        symbol,
        pair,
        call,
        reverse,
        list,
        map,
    }
}

pub(crate) fn any_pair_mode(rng: &mut SmallRng, depth: usize) -> PairMode {
    let new_depth = depth + 1;
    let weight: usize = 1 << min(depth, 32);
    let weights = [weight, 1];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    if i == 0 {
        PairMode::Eval(any_eval_mode(rng))
    } else {
        PairMode::Pair(Pair::new(
            any_io_mode(rng, new_depth),
            any_io_mode(rng, new_depth),
        ))
    }
}

pub(crate) fn any_call_mode(rng: &mut SmallRng, depth: usize) -> CallMode {
    let new_depth = depth + 1;
    let weight: usize = 1 << min(depth, 32);
    let weights = [weight, 1];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    if i == 0 {
        CallMode::Eval(any_eval_mode(rng))
    } else {
        CallMode::Call(Call::new(
            any_io_mode(rng, new_depth),
            any_io_mode(rng, new_depth),
        ))
    }
}

pub(crate) fn any_reverse_mode(rng: &mut SmallRng, depth: usize) -> ReverseMode {
    let new_depth = depth + 1;
    let weight: usize = 1 << min(depth, 32);
    let weights = [weight, 1];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    if i == 0 {
        ReverseMode::Eval(any_eval_mode(rng))
    } else {
        ReverseMode::Reverse(Reverse::new(
            any_io_mode(rng, new_depth),
            any_io_mode(rng, new_depth),
        ))
    }
}

pub(crate) fn any_list_mode(rng: &mut SmallRng, depth: usize) -> ListMode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // list
        1,      // list for all
        1,      // list for some
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => ListMode::Eval(any_eval_mode(rng)),
        1 => ListMode::ForAll(any_io_mode(rng, new_depth)),
        2 => {
            let left = any_len_weighted(rng, depth) >> 1;
            let right = any_len_weighted(rng, depth) >> 1;
            let mut list = Vec::with_capacity(left + 1 + right);
            for _ in 0..left {
                list.push(ListItemMode {
                    ellipsis: false,
                    io_mode: any_io_mode(rng, new_depth),
                });
            }
            if rng.gen() {
                list.push(ListItemMode {
                    ellipsis: true,
                    io_mode: any_io_mode(rng, new_depth),
                });
            }
            for _ in 0..right {
                list.push(ListItemMode {
                    ellipsis: false,
                    io_mode: any_io_mode(rng, new_depth),
                });
            }
            ListMode::ForSome(List::from(list))
        }
        _ => unreachable!(),
    }
}

pub(crate) fn any_map_mode(rng: &mut SmallRng, depth: usize) -> MapMode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // map
        1,      // map for all
        1,      // map for some
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => MapMode::Eval(any_eval_mode(rng)),
        1 => MapMode::ForAll(Pair::new(
            any_io_mode(rng, new_depth),
            any_io_mode(rng, new_depth),
        )),
        2 => {
            let len = any_len_weighted(rng, new_depth);
            let mut map = Map::with_capacity(len);
            for _ in 0..len {
                map.insert(any_val(rng, new_depth), any_io_mode(rng, new_depth));
            }
            MapMode::ForSome(map)
        }
        _ => unreachable!(),
    }
}

pub(crate) fn any_io_mode(rng: &mut SmallRng, depth: usize) -> IoMode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [weight, 1];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    if i == 0 {
        IoMode::Eval(any_eval_mode(rng))
    } else {
        IoMode::Match(any_match_mode(rng, depth))
    }
}

pub(crate) fn any_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.gen() {
        let prelude = PRELUDE.with(|prelude| {
            let mut m = NameMap::default();
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
        let input_mode = any_io_mode(rng, depth);
        let output_mode = any_io_mode(rng, depth);
        let evaluator = match rng.gen_range(0..variant_count::<FuncEval>()) {
            0 => FuncEval::Free(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                ctx: *any_ctx(rng, depth).0,
                input_name: any_symbol(rng),
                caller: CtxFreeInfo {},
            })),
            1 => FuncEval::Const(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                ctx: *any_ctx(rng, depth).0,
                input_name: any_symbol(rng),
                caller: CtxConstInfo {
                    name: any_symbol(rng),
                },
            })),
            2 => FuncEval::Mutable(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                ctx: *any_ctx(rng, depth).0,
                input_name: any_symbol(rng),
                caller: CtxMutableInfo {
                    name: any_symbol(rng),
                },
            })),
            _ => unreachable!(),
        };
        let func = Func {
            input_mode,
            output_mode,
            evaluator,
        };
        FuncVal(Reader::new(func))
    }
}

pub(crate) fn any_free_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.gen() {
        let prelude = PRELUDE.with(|prelude| {
            let mut m = NameMap::default();
            prelude.put(&mut m);
            m
        });
        let func = prelude
            .into_values()
            .filter(|v| {
                let Val::Func(func) = &v.val else {
                    return false;
                };
                let FuncEval::Free(_) = &func.evaluator else {
                    return false;
                };
                true
            })
            .choose(rng)
            .unwrap();
        let Val::Func(func) = func.val else {
            unreachable!()
        };
        func
    } else {
        let input_mode = any_io_mode(rng, depth);
        let output_mode = any_io_mode(rng, depth);
        let evaluator = FuncEval::Free(FuncImpl::Composed(Composed {
            body: any_val(rng, depth),
            ctx: *any_ctx(rng, depth).0,
            input_name: any_symbol(rng),
            caller: CtxFreeInfo {},
        }));
        let func = Func {
            input_mode,
            output_mode,
            evaluator,
        };
        FuncVal(Reader::new(func))
    }
}

pub(crate) fn any_prop(rng: &mut SmallRng, depth: usize) -> PropVal {
    let func = any_free_func(rng, depth);
    let input = any_val(rng, depth);
    let prop = if rng.gen() {
        Prop::new_proved(func, input)
    } else {
        let output = any_val(rng, depth);
        Prop::new(func, input, output)
    };
    PropVal(Reader::new(prop))
}

pub(crate) fn any_proved_prop(rng: &mut SmallRng, depth: usize) -> PropVal {
    let func = any_free_func(rng, depth);
    let input = any_val(rng, depth);
    let prop = Prop::new_proved(func, input);
    PropVal(Reader::new(prop))
}

pub(crate) fn any_answer(rng: &mut SmallRng, depth: usize) -> AnswerVal {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight, // unsolved
        weight, // unsolvable
        1,      // unverified
        1,      // verified
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    let answer = match i {
        0 => Answer::Unsolved,
        1 => Answer::Unsolvable,
        2 => Answer::Unverified(any_val(rng, new_depth)),
        3 => Answer::Verified(Verified(any_proved_prop(rng, new_depth))),
        _ => unreachable!(),
    };
    AnswerVal::from(Box::new(answer))
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
