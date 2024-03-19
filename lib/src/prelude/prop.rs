use std::rc::Rc;

use crate::{
    bool::Bool,
    ctx::{
        DefaultCtx,
        NameMap,
    },
    ctx_access::constant::CtxForConstFn,
    eval::Evaluator,
    func::FuncEval,
    logic::Prop,
    map::Map,
    prelude::{
        default_mode,
        map_mode_for_some,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        symbol_value_mode,
        utils::{
            map_remove,
            symbol,
        },
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        map::MapVal,
        prop::PropVal,
    },
    CtxForMutableFn,
    EvalMode,
    IoMode,
    Val,
};

#[derive(Clone)]
pub(crate) struct PropPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) repr: Named<FuncVal>,
    pub(crate) proved: Named<FuncVal>,
    pub(crate) func: Named<FuncVal>,
    pub(crate) input: Named<FuncVal>,
    pub(crate) output: Named<FuncVal>,
}

impl Default for PropPrelude {
    fn default() -> Self {
        PropPrelude {
            new: new(),
            repr: repr(),
            proved: proved(),
            func: func(),
            input: input(),
            output: output(),
        }
    }
}

impl Prelude for PropPrelude {
    fn put(&self, m: &mut NameMap) {
        self.new.put(m);
        self.repr.put(m);
        self.proved.put(m);
        self.func.put(m);
        self.input.put(m);
        self.output.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const PROVED: &str = "proved";

fn new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), IoMode::Eval(EvalMode::Id));
    map.insert(symbol(OUTPUT), IoMode::Eval(EvalMode::Id));
    let input_mode = map_mode_for_some(map);
    let output_mode = default_mode();
    named_mutable_fn("proposition", input_mode, output_mode, fn_new)
}

fn fn_new(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let FuncEval::Free(_) = &func.0.evaluator else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    let input = func.input_mode.eval(&mut ctx, input);
    let output = map_remove(&mut map, OUTPUT);
    let output = func.output_mode.eval(&mut ctx, output);
    let prop = Prop::new(func, input, output);
    Val::Prop(PropVal(Rc::new(prop)))
}

fn repr() -> Named<FuncVal> {
    let input_mode = default_mode();
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), IoMode::Eval(EvalMode::Id));
    map.insert(symbol(OUTPUT), IoMode::Eval(EvalMode::Id));
    map.insert(symbol(PROVED), default_mode());
    let output_mode = map_mode_for_some(map);
    named_free_fn("proposition.represent", input_mode, output_mode, fn_repr)
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
    if prop.proved() {
        repr.insert(symbol(PROVED), Val::Bool(Bool::t()));
    }
}

fn proved() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("proposition.proved", input_mode, output_mode, fn_proved)
}

fn fn_proved(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(prop.proved()))
    })
}

fn func() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("proposition.function", input_mode, output_mode, fn_func)
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
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("proposition.input", input_mode, output_mode, fn_input)
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
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("proposition.output", input_mode, output_mode, fn_output)
}

fn fn_output(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Prop(PropVal(prop)) = val else {
            return Val::default();
        };
        prop.output().clone()
    })
}
