use std::rc::Rc;

use crate::{
    ctx::NameMap,
    eval::Evaluator,
    func::FuncEval,
    logic::Prop,
    prelude::{
        call_mode,
        default_mode,
        named_mutable_fn,
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        prop::PropVal,
    },
    CtxForMutableFn,
    EvalMode,
    IoMode,
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

fn prove() -> Named<FuncVal> {
    let input_mode = call_mode(default_mode(), IoMode::Eval(EvalMode::Id));
    let output_mode = default_mode();
    named_mutable_fn("proposition.prove", input_mode, output_mode, fn_prove)
}

fn fn_prove(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    let Val::Func(func) = call.func else {
        return Val::default();
    };
    let FuncEval::Free(_) = &func.0.evaluator else {
        return Val::default();
    };
    let input = func.input_mode.eval(&mut ctx, call.input);
    let theorem = Prop::new_proved(func, input);
    Val::Prop(PropVal(Rc::new(theorem)))
}
