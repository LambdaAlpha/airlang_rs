use crate::{
    ctx_access::free::FreeCtx,
    func::FuncTransformer,
    transformer::Transformer,
    val::func::FuncVal,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assert {
    func: FuncVal,
    input: Val,
    output: Val,
    verified: bool,
}

impl Assert {
    pub(crate) fn new(func: FuncVal, input: Val, output: Val) -> Self {
        debug_assert!(matches!(func.transformer, FuncTransformer::Free(_)));
        Self {
            func,
            input,
            output,
            verified: false,
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

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub(crate) fn new_verified(func: FuncVal, input: Val) -> Self {
        debug_assert!(matches!(func.transformer, FuncTransformer::Free(_)));
        let output = func.transformer.transform(&mut FreeCtx, input.clone());
        Self {
            func,
            input,
            output,
            verified: true,
        }
    }
}
