use crate::{
    ctx_access::free::FreeCtx,
    func::FuncCore,
    transformer::Transformer,
    val::func::FuncVal,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prop {
    func: FuncVal,
    input: Val,
    output: Val,
    proved: bool,
}

impl Prop {
    pub(crate) fn new(func: FuncVal, input: Val, output: Val) -> Self {
        debug_assert!(matches!(func.0.core, FuncCore::Free(_)));
        Self {
            func,
            input,
            output,
            proved: false,
        }
    }

    pub fn func(&self) -> &FuncVal {
        &self.func
    }

    pub fn input(&self) -> &Val {
        &self.input
    }

    pub fn output(&self) -> &Val {
        &self.output
    }

    pub fn proved(&self) -> bool {
        self.proved
    }

    pub(crate) fn new_proved(func: FuncVal, input: Val) -> Self {
        debug_assert!(matches!(func.0.core, FuncCore::Free(_)));
        let output = func.0.core.transform(&mut FreeCtx, input.clone());
        Self {
            func,
            input,
            output,
            proved: true,
        }
    }
}
