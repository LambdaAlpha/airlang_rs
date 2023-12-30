use crate::{
    ctx::NameMap,
    func::FuncEval,
    logic::Prop,
    map::Map,
    prelude::{
        default_mode,
        map_mode_for_some,
        named_free_fn,
        utils::{
            map_remove,
            symbol,
        },
        Named,
        Prelude,
    },
    types::refer::Reader,
    val::{
        ctx::CtxVal,
        func::FuncVal,
        prop::PropVal,
    },
    Val,
};

#[derive(Clone)]
pub(crate) struct LogicPrelude {
    pub(crate) theorem_new: Named<FuncVal>,
    pub(crate) prove: Named<FuncVal>,
}

impl Default for LogicPrelude {
    fn default() -> Self {
        LogicPrelude {
            theorem_new: theorem_new(),
            prove: prove(),
        }
    }
}

impl Prelude for LogicPrelude {
    fn put(&self, m: &mut NameMap) {
        self.theorem_new.put(m);
        self.prove.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";
const CTX: &str = "context";
const BEFORE: &str = "before";

fn theorem_new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), default_mode());
    map.insert(symbol(CTX), default_mode());
    map.insert(symbol(BEFORE), default_mode());
    let input_mode = map_mode_for_some(map);
    let output_mode = default_mode();
    named_free_fn("theorem", input_mode, output_mode, fn_theorem_new)
}

fn fn_theorem_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    match &func.0.evaluator {
        FuncEval::Free(_) => {
            let theorem = Prop::new_free_theorem(func, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
        FuncEval::Const(_) => {
            let Val::Ctx(CtxVal(ctx)) = map_remove(&mut map, CTX) else {
                return Val::default();
            };
            let theorem = Prop::new_const_theorem(func, *ctx, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
        FuncEval::Mutable(_) => {
            let Val::Ctx(CtxVal(before)) = map_remove(&mut map, BEFORE) else {
                return Val::default();
            };
            let theorem = Prop::new_mutable_theorem(func, *before, input);
            Val::Prop(PropVal(Reader::new(theorem)))
        }
    }
}

fn prove() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("prove", input_mode, output_mode, fn_prove)
}

fn fn_prove(input: Val) -> Val {
    let Val::Prop(PropVal(prop)) = input else {
        return Val::default();
    };
    let theorem = Prop::prove(Prop::clone(&*prop));
    Val::Prop(PropVal(Reader::new(theorem)))
}
