use crate::{
    bool::Bool,
    ctx::{
        CtxMap,
        DefaultCtx,
    },
    ctx_access::constant::CtxForConstFn,
    func::FuncTransformer,
    logic::Assert,
    map::Map,
    prelude::{
        default_mode,
        map_some_mode,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        symbol_id_mode,
        Named,
        Prelude,
    },
    transformer::Transformer,
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        func::FuncVal,
        map::MapVal,
    },
    CtxForMutableFn,
    Mode,
    Transform,
    Val,
};

#[derive(Clone)]
pub(crate) struct AssertPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) verified: Named<FuncVal>,
    pub(crate) func: Named<FuncVal>,
    pub(crate) input: Named<FuncVal>,
    pub(crate) output: Named<FuncVal>,
}

impl Default for AssertPrelude {
    fn default() -> Self {
        AssertPrelude {
            new: new(),
            repr: repr(),
            verified: is_verified(),
            func: func(),
            input: input(),
            output: output(),
        }
    }
}

impl Prelude for AssertPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.repr.put(m);
        self.verified.put(m);
        self.func.put(m);
        self.input.put(m);
        self.output.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const VERIFIED: &str = "verified";

fn new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), Mode::Predefined(Transform::Id));
    map.insert(symbol(OUTPUT), Mode::Predefined(Transform::Id));
    let input_mode = map_some_mode(map);
    let output_mode = default_mode();
    named_mutable_fn("assert", input_mode, output_mode, fn_new)
}

fn fn_new(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let FuncTransformer::Free(_) = &func.transformer else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    let input = func.input_mode.transform(ctx.reborrow(), input);
    let output = map_remove(&mut map, OUTPUT);
    let output = func.output_mode.transform(ctx, output);
    let assert = Assert::new(func, input, output);
    Val::Assert(assert.into())
}

fn repr() -> Named<FuncVal> {
    let input_mode = default_mode();
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), Mode::Predefined(Transform::Id));
    map.insert(symbol(OUTPUT), Mode::Predefined(Transform::Id));
    map.insert(symbol(VERIFIED), default_mode());
    let output_mode = map_some_mode(map);
    named_free_fn("assert.represent", input_mode, output_mode, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Assert(assert) = input else {
        return Val::default();
    };
    let mut repr = MapVal::from(Map::<Val, Val>::default());
    generate_assert(&mut repr, &assert);
    Val::Map(repr)
}

fn generate_assert(repr: &mut MapVal, assert: &Assert) {
    repr.insert(symbol(FUNCTION), Val::Func(assert.func().clone()));
    repr.insert(symbol(INPUT), assert.input().clone());
    repr.insert(symbol(OUTPUT), assert.output().clone());
    if assert.is_verified() {
        repr.insert(symbol(VERIFIED), Val::Bool(Bool::t()));
    }
}

fn is_verified() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn(
        "assert.is_verified",
        input_mode,
        output_mode,
        fn_is_verified,
    )
}

fn fn_is_verified(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Assert(assert) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(assert.is_verified()))
    })
}

fn func() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("assert.function", input_mode, output_mode, fn_func)
}

fn fn_func(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Assert(assert) = val else {
            return Val::default();
        };
        Val::Func(assert.func().clone())
    })
}

fn input() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("assert.input", input_mode, output_mode, fn_input)
}

fn fn_input(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Assert(assert) = val else {
            return Val::default();
        };
        assert.input().clone()
    })
}

fn output() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("assert.output", input_mode, output_mode, fn_output)
}

fn fn_output(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Assert(assert) = val else {
            return Val::default();
        };
        assert.output().clone()
    })
}
