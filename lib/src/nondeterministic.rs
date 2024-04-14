use std::{
    cmp::min,
    rc::Rc,
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
    ctx::{
        Ctx,
        CtxMap,
        CtxValue,
        Invariant,
    },
    extension::UnitExt,
    float::Float,
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
    logic::Prop,
    map::Map,
    mode::{
        ListItemMode,
        Mode,
    },
    pair::Pair,
    prelude::{
        Prelude,
        PRELUDE,
    },
    problem::Verified,
    string::Str,
    symbol::Symbol,
    transform::Transform,
    unit::Unit,
    val::{
        ask::AskVal,
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        pair::PairVal,
        prop::PropVal,
    },
    Answer,
    AnswerVal,
    Ask,
    AskDepMode,
    AskMode,
    Call,
    CallDepMode,
    CallMode,
    ListMode,
    MapMode,
    PairMode,
    SymbolMode,
    Val,
    ValExt,
    ValMode,
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
        1,      // ask
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
        9 => Val::Ask(Box::new(any_ask(rng, new_depth))),
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
    let sign: bool = rng.gen();
    let integral: u32 = rng.gen();
    let integral: String = integral.to_string();
    let fractional: u32 = rng.gen();
    let fractional: String = fractional.to_string();
    let exp_sign: bool = rng.gen();
    let exp: u8 = rng.gen();
    let exp: String = exp.to_string();
    Float::from_parts(sign, &integral, &fractional, exp_sign, &exp)
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
        rng.gen_range(Symbol::MIN..=Symbol::MAX) as u8
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

pub(crate) fn any_ask(rng: &mut SmallRng, depth: usize) -> AskVal {
    AskVal::new(any_val(rng, depth), any_val(rng, depth))
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

pub(crate) fn any_invariant(rng: &mut SmallRng) -> Invariant {
    const INVARIANTS: [Invariant; 3] = [Invariant::None, Invariant::Final, Invariant::Const];
    *(INVARIANTS.choose(rng).unwrap())
}

pub(crate) fn any_ctx(rng: &mut SmallRng, depth: usize) -> CtxVal {
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
        Some(any_ctx(rng, depth).0)
    } else {
        None
    };
    let ctx = Ctx { map: ctx_map, meta };
    CtxVal(Box::new(ctx))
}

pub(crate) fn any_transform(rng: &mut SmallRng) -> Transform {
    const TRANSFORMS: [Transform; 3] = [Transform::Eval, Transform::Id, Transform::Lazy];
    *(TRANSFORMS.choose(rng).unwrap())
}

pub(crate) fn any_val_mode(rng: &mut SmallRng, depth: usize) -> ValMode {
    let symbol = any_symbol_mode(rng);
    let pair = Box::new(any_pair_mode(rng, depth));
    let call = Box::new(any_call_mode(rng, depth));
    let ask = Box::new(any_ask_mode(rng, depth));
    let list = Box::new(any_list_mode(rng, depth));
    let map = Box::new(any_map_mode(rng, depth));
    ValMode {
        symbol,
        pair,
        call,
        ask,
        list,
        map,
    }
}

pub(crate) fn any_symbol_mode(rng: &mut SmallRng) -> SymbolMode {
    if rng.gen() {
        SymbolMode::Eval
    } else {
        SymbolMode::Id
    }
}

pub(crate) fn any_pair_mode(rng: &mut SmallRng, depth: usize) -> PairMode {
    let new_depth = depth + 1;
    Pair::new(any_mode(rng, new_depth), any_mode(rng, new_depth))
}

pub(crate) fn any_call_mode(rng: &mut SmallRng, depth: usize) -> CallMode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight,      // eval
        weight >> 1, // structure
        1,           // dependent
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => CallMode::Eval,
        1 => CallMode::Struct(Call::new(
            any_mode(rng, new_depth),
            any_mode(rng, new_depth),
        )),
        2 => CallMode::Dependent(CallDepMode {
            unit: any_mode(rng, new_depth),
            bool: any_mode(rng, new_depth),
            int: any_mode(rng, new_depth),
            float: any_mode(rng, new_depth),
            bytes: any_mode(rng, new_depth),
            string: any_mode(rng, new_depth),
            symbol: any_mode(rng, new_depth),
        }),
        _ => unreachable!(),
    }
}

pub(crate) fn any_ask_mode(rng: &mut SmallRng, depth: usize) -> AskMode {
    let weight: usize = 1 << min(depth, 32);
    let weights = [
        weight,      // eval
        weight >> 1, // structure
        1,           // dependent
    ];
    let dist = WeightedIndex::new(weights).unwrap();
    let i = dist.sample(rng);
    let new_depth = depth + 1;

    match i {
        0 => AskMode::Eval,
        1 => AskMode::Struct(Ask::new(any_mode(rng, new_depth), any_mode(rng, new_depth))),
        2 => AskMode::Dependent(AskDepMode {
            unit: any_mode(rng, new_depth),
            bool: any_mode(rng, new_depth),
            int: any_mode(rng, new_depth),
            float: any_mode(rng, new_depth),
            bytes: any_mode(rng, new_depth),
            string: any_mode(rng, new_depth),
            symbol: any_mode(rng, new_depth),
        }),
        _ => unreachable!(),
    }
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

    match i {
        0 => Mode::Predefined(any_transform(rng)),
        1 => Mode::Custom(Box::new(any_val_mode(rng, new_depth))),
        _ => unreachable!(),
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
                ctx: *any_ctx(rng, depth).0,
                input_name: any_symbol(rng),
                caller: CtxFreeInfo {},
            })),
            1 => FuncTransformer::Const(FuncImpl::Composed(Composed {
                body: any_val(rng, depth),
                ctx: *any_ctx(rng, depth).0,
                input_name: any_symbol(rng),
                caller: CtxConstInfo {
                    name: any_symbol(rng),
                },
            })),
            2 => FuncTransformer::Mutable(FuncImpl::Composed(Composed {
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
            transformer,
        };
        FuncVal(Rc::new(func))
    }
}

pub(crate) fn any_free_func(rng: &mut SmallRng, depth: usize) -> FuncVal {
    if rng.gen() {
        let prelude = PRELUDE.with(|prelude| {
            let mut m = CtxMap::default();
            prelude.put(&mut m);
            m
        });
        let func = prelude
            .into_values()
            .filter(|v| {
                let Val::Func(func) = &v.val else {
                    return false;
                };
                let FuncTransformer::Free(_) = &func.transformer else {
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
        let input_mode = any_mode(rng, depth);
        let output_mode = any_mode(rng, depth);
        let transformer = FuncTransformer::Free(FuncImpl::Composed(Composed {
            body: any_val(rng, depth),
            ctx: *any_ctx(rng, depth).0,
            input_name: any_symbol(rng),
            caller: CtxFreeInfo {},
        }));
        let func = Func {
            input_mode,
            output_mode,
            transformer,
        };
        FuncVal(Rc::new(func))
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
    PropVal(Rc::new(prop))
}

pub(crate) fn any_proved_prop(rng: &mut SmallRng, depth: usize) -> PropVal {
    let func = any_free_func(rng, depth);
    let input = any_val(rng, depth);
    let prop = Prop::new_proved(func, input);
    PropVal(Rc::new(prop))
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
