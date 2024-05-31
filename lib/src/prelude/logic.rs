use crate::{
    ctx::{
        mutable::CtxForMutableFn,
        CtxMap,
    },
    func::FuncTransformer,
    logic::Assert,
    prelude::{
        call_mode,
        default_mode,
        named_mutable_fn,
        Named,
        Prelude,
    },
    transformer::Transformer,
    val::{
        assert::AssertVal,
        func::FuncVal,
    },
    Call,
    Mode,
    Transform,
    Val,
};

#[derive(Clone)]
pub(crate) struct LogicPrelude {
    pub(crate) verified: Named<FuncVal>,
}

impl Default for LogicPrelude {
    fn default() -> Self {
        LogicPrelude {
            verified: verified(),
        }
    }
}

impl Prelude for LogicPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.verified.put(m);
    }
}

fn verified() -> Named<FuncVal> {
    let input_mode = call_mode(default_mode(), Mode::Predefined(Transform::Id));
    let output_mode = default_mode();
    named_mutable_fn("assert.verified", input_mode, output_mode, fn_verified)
}

fn fn_verified(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Call(call) = input else {
        return Val::default();
    };
    let call = Call::from(call);
    let Val::Func(func) = call.func else {
        return Val::default();
    };
    let FuncTransformer::Free(_) = &func.transformer else {
        return Val::default();
    };
    let input = func.input_mode.transform(ctx, call.input);
    let verified = Assert::new_verified(func, input);
    Val::Assert(AssertVal::from(verified))
}
