use std::rc::Rc;

use crate::{
    ctx::NameMap,
    func::FuncCore,
    logic::Prop,
    prelude::{
        call_mode,
        default_mode,
        named_mutable_fn,
        Named,
        Prelude,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        prop::PropVal,
    },
    CtxForMutableFn,
    Mode,
    Transform,
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
    let input_mode = call_mode(default_mode(), Mode::Generic(Transform::Id));
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
    let FuncCore::Free(_) = &func.0.core else {
        return Val::default();
    };
    let input = func.input_mode.transform(&mut ctx, call.input);
    let theorem = Prop::new_proved(func, input);
    Val::Prop(PropVal(Rc::new(theorem)))
}
