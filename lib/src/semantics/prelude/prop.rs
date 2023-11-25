use crate::{
    semantics::{
        ctx::{
            DefaultCtx,
            NameMap,
        },
        ctx_access::constant::CtxForConstFn,
        eval_mode::EvalMode,
        func::FuncEval,
        input_mode::InputMode,
        logic::{
            Prop,
            PropCtx,
            Truth,
        },
        prelude::{
            named_const_fn,
            named_free_fn,
            utils::{
                map_remove,
                symbol,
            },
            Named,
            Prelude,
        },
        val::{
            CtxVal,
            FuncVal,
            MapVal,
            PropVal,
        },
        Val,
    },
    types::{
        Bool,
        Map,
        Reader,
        Unit,
    },
};

#[derive(Clone)]
pub(crate) struct PropPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) truth: Named<FuncVal>,
    pub(crate) func: Named<FuncVal>,
    pub(crate) input: Named<FuncVal>,
    pub(crate) output: Named<FuncVal>,
    pub(crate) before: Named<FuncVal>,
    pub(crate) after: Named<FuncVal>,
}

impl Default for PropPrelude {
    fn default() -> Self {
        PropPrelude {
            new: new(),
            repr: repr(),
            truth: truth(),
            func: func(),
            input: input(),
            output: output(),
            before: before(),
            after: after(),
        }
    }
}

impl Prelude for PropPrelude {
    fn put(&self, m: &mut NameMap) {
        self.new.put(m);
        self.repr.put(m);
        self.truth.put(m);
        self.func.put(m);
        self.input.put(m);
        self.output.put(m);
        self.before.put(m);
        self.after.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const CTX: &str = "context";
const BEFORE: &str = "before";
const AFTER: &str = "after";
const TRUTH: &str = "truth";

fn new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), InputMode::Any(EvalMode::More));
    map.insert(symbol(INPUT), InputMode::Any(EvalMode::More));
    map.insert(symbol(OUTPUT), InputMode::Any(EvalMode::More));
    map.insert(symbol(CTX), InputMode::Any(EvalMode::More));
    map.insert(symbol(BEFORE), InputMode::Any(EvalMode::More));
    map.insert(symbol(AFTER), InputMode::Any(EvalMode::More));
    let input_mode = InputMode::MapForSome(map);
    named_free_fn("proposition", input_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    let output = map_remove(&mut map, OUTPUT);
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let prop = Prop::new_free(func, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, CTX) else {
                return Val::default();
            };
            let prop = Prop::new_const(func, *ctx, input, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, BEFORE) else {
                return Val::default();
            };
            let Val::Ctx(CtxVal(after)) = map_remove(&mut map, AFTER) else {
                return Val::default();
            };
            let prop = Prop::new_mutable(func, *before, input, *after, output);
            Val::Prop(PropVal(Reader::new(prop)))
        }
    }
}

fn repr() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::More);
    named_free_fn("proposition.represent", input_mode, fn_repr)
}

fn fn_repr(input: Val) -> Val {
    let Val::Prop(PropVal(prop)) = input else {
        return Val::default();
    };
    let mut repr = MapVal::default();
    generate_prop(&mut repr, &prop);
    Val::Map(repr)
}

fn generate_prop(repr: &mut MapVal, prop: &Prop) {
    repr.insert(symbol(FUNCTION), Val::Func(prop.func().clone()));
    repr.insert(symbol(INPUT), prop.input().clone());
    repr.insert(symbol(OUTPUT), prop.output().clone());
    match prop.ctx() {
        PropCtx::Free => {}
        PropCtx::Const(ctx) => {
            repr.insert(symbol(CTX), Val::Ctx(CtxVal(Box::new(ctx.clone()))));
        }
        PropCtx::Mutable(before, after) => {
            repr.insert(symbol(BEFORE), Val::Ctx(CtxVal(Box::new(before.clone()))));
            repr.insert(symbol(AFTER), Val::Ctx(CtxVal(Box::new(after.clone()))));
        }
    }
    match prop.truth() {
        Truth::True => {
            repr.insert(symbol(TRUTH), Val::Bool(Bool::t()));
        }
        Truth::False => {
            repr.insert(symbol(TRUTH), Val::Bool(Bool::f()));
        }
        _ => {}
    };
}

fn truth() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.truth", input_mode, fn_truth)
}

fn fn_truth(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        match prop.truth() {
            Truth::None => Val::Unit(Unit),
            Truth::True => Val::Bool(Bool::t()),
            Truth::False => Val::Bool(Bool::f()),
        }
    })
}

fn func() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.function", input_mode, fn_func)
}

fn fn_func(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        Val::Func(prop.func().clone())
    })
}

fn input() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.input", input_mode, fn_input)
}

fn fn_input(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        prop.input().clone()
    })
}

fn output() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.output", input_mode, fn_output)
}

fn fn_output(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        prop.output().clone()
    })
}

fn before() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.before", input_mode, fn_before)
}

fn fn_before(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        let ctx = prop.ctx();
        match ctx {
            PropCtx::Free => Val::default(),
            PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
            PropCtx::Mutable(before, _) => Val::Ctx(CtxVal(Box::new(before.clone()))),
        }
    })
}

fn after() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("proposition.after", input_mode, fn_after)
}

fn fn_after(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        let ctx = prop.ctx();
        match ctx {
            PropCtx::Free => Val::default(),
            PropCtx::Const(ctx) => Val::Ctx(CtxVal(Box::new(ctx.clone()))),
            PropCtx::Mutable(_, after) => Val::Ctx(CtxVal(Box::new(after.clone()))),
        }
    })
}
