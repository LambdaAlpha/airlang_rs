use crate::{
    Cache,
    Call,
    Case,
    CaseVal,
    Mode,
    Symbol,
    Val,
    bit::Bit,
    core::EvalCore,
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
    },
    map::Map,
    mode::{
        eval::Eval,
        primitive::PrimitiveMode,
    },
    prelude::{
        Named,
        Prelude,
        form_mode,
        map_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
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
};

#[derive(Clone)]
pub(crate) struct CasePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) new_cache: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) is_cache: Named<FuncVal>,
    pub(crate) func: Named<FuncVal>,
    pub(crate) input: Named<FuncVal>,
    pub(crate) output: Named<FuncVal>,
}

impl Default for CasePrelude {
    fn default() -> Self {
        CasePrelude {
            new: new(),
            new_cache: new_cache(),
            repr: repr(),
            is_cache: is_cache(),
            func: func(),
            input: input(),
            output: output(),
        }
    }
}

impl Prelude for CasePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.new_cache.put(m);
        self.repr.put(m);
        self.is_cache.put(m);
        self.func.put(m);
        self.input.put(m);
        self.output.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const IS_CACHE: &str = "is_cache";

fn new() -> Named<FuncVal> {
    let id = "case";
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), Mode::default());
    map.insert(symbol(INPUT), form_mode());
    map.insert(symbol(OUTPUT), form_mode());
    let call_mode = map_mode(
        map,
        Mode::default(),
        Mode::default(),
        PrimitiveMode::default(),
    );
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_new;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_new(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let func = map_remove(&mut map, FUNCTION);
    let input = map_remove(&mut map, INPUT);
    let input = EvalCore::call_eval_input(&Eval, ctx.reborrow(), &func, input);
    let output = map_remove(&mut map, OUTPUT);
    let output = EvalCore::ask_eval_output(&Eval, ctx, &func, output);
    let case = Case::new(func, input, output);
    Val::Case(CaseVal::Trivial(case.into()))
}

fn new_cache() -> Named<FuncVal> {
    let id = "case.cache";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_new_cache;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_new_cache(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    let call = Call::from(call);
    let Val::Func(func) = call.func else {
        return Val::default();
    };
    let input = func.call_mode().transform(ctx.reborrow(), call.input);
    let cache = Cache::new(ctx, func, input);
    Val::Case(CaseVal::Cache(cache.into()))
}

fn repr() -> Named<FuncVal> {
    let id = "case.represent";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), Mode::default());
    map.insert(symbol(INPUT), form_mode());
    map.insert(symbol(OUTPUT), form_mode());
    map.insert(symbol(IS_CACHE), Mode::default());
    let ask_mode = map_mode(
        map,
        Mode::default(),
        Mode::default(),
        PrimitiveMode::default(),
    );
    let cacheable = true;
    let f = fn_repr;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_repr(input: Val) -> Val {
    let Val::Case(case) = input else {
        return Val::default();
    };
    let mut repr = MapVal::from(Map::<Val, Val>::default());
    generate_case(&mut repr, &case);
    Val::Map(repr)
}

fn generate_case(repr: &mut MapVal, case: &CaseVal) {
    repr.insert(symbol(FUNCTION), case.as_ref().func.clone());
    repr.insert(symbol(INPUT), case.as_ref().input.clone());
    repr.insert(symbol(OUTPUT), case.as_ref().output.clone());
    if matches!(case, CaseVal::Cache(_)) {
        repr.insert(symbol(IS_CACHE), Val::Bit(Bit::t()));
    }
}

fn is_cache() -> Named<FuncVal> {
    let id = "case.is_cache";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_is_cache;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_is_cache(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Case(case) = val else {
            return Val::default();
        };
        Val::Bit(Bit::new(matches!(case, CaseVal::Cache(_))))
    })
}

fn func() -> Named<FuncVal> {
    let id = "case.function";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_func;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_func(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Case(case) = val else {
            return Val::default();
        };
        case.as_ref().func.clone()
    })
}

fn input() -> Named<FuncVal> {
    let id = "case.input";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_input;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_input(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Case(case) = val else {
            return Val::default();
        };
        case.as_ref().input.clone()
    })
}

fn output() -> Named<FuncVal> {
    let id = "case.output";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_output;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_output(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::Case(case) = val else {
            return Val::default();
        };
        case.as_ref().output.clone()
    })
}
