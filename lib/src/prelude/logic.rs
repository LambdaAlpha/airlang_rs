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
        func::FuncVal,
        prop::PropVal,
    },
    Val,
};

#[derive(Clone)]
pub(crate) struct LogicPrelude {
    pub(crate) prove: Named<FuncVal>,
}

impl Default for LogicPrelude {
    fn default() -> Self {
        LogicPrelude { prove: prove() }
    }
}

impl Prelude for LogicPrelude {
    fn put(&self, m: &mut NameMap) {
        self.prove.put(m);
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";

fn prove() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(symbol(FUNCTION), default_mode());
    map.insert(symbol(INPUT), default_mode());
    let input_mode = map_mode_for_some(map);
    let output_mode = default_mode();
    named_free_fn("proposition.prove", input_mode, output_mode, fn_prove)
}

fn fn_prove(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let Val::Func(func) = map_remove(&mut map, FUNCTION) else {
        return Val::default();
    };
    let input = map_remove(&mut map, INPUT);
    let FuncEval::Free(_) = &func.0.evaluator else {
        return Val::default();
    };
    let theorem = Prop::new_theorem(func, input);
    Val::Prop(PropVal(Reader::new(theorem)))
}
