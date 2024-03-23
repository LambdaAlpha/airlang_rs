use crate::{
    ctx::NameMap,
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
    },
    prelude::{
        default_mode,
        named_free_fn,
        named_mutable_fn,
        Named,
        Prelude,
    },
    transform::{
        eval::Eval,
        id::Id,
        lazy::Lazy,
        Transform,
    },
    transformer::{
        Transformer,
        EVAL,
        ID,
        LAZY,
    },
    val::{
        func::FuncVal,
        Val,
    },
    Mode,
};

#[derive(Clone)]
pub(crate) struct TransformPrelude {
    pub(crate) eval: Named<FuncVal>,
    pub(crate) id: Named<FuncVal>,
    pub(crate) lazy: Named<FuncVal>,
}

impl Default for TransformPrelude {
    fn default() -> Self {
        TransformPrelude {
            eval: eval(),
            id: id(),
            lazy: lazy(),
        }
    }
}

impl Prelude for TransformPrelude {
    fn put(&self, m: &mut NameMap) {
        self.eval.put(m);
        self.id.put(m);
        self.lazy.put(m);
    }
}

fn eval() -> Named<FuncVal> {
    let input_mode = Mode::Generic(Transform::Id);
    let output_mode = default_mode();
    named_mutable_fn(EVAL, input_mode, output_mode, fn_eval)
}

fn fn_eval(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eval.transform(&mut ctx, input)
}

fn id() -> Named<FuncVal> {
    let input_mode = Mode::Generic(Transform::Id);
    let output_mode = default_mode();
    named_free_fn(ID, input_mode, output_mode, fn_id)
}

fn fn_id(input: Val) -> Val {
    Id.transform(&mut FreeCtx, input)
}

fn lazy() -> Named<FuncVal> {
    let input_mode = Mode::Generic(Transform::Id);
    let output_mode = default_mode();
    named_mutable_fn(LAZY, input_mode, output_mode, fn_lazy)
}

fn fn_lazy(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Lazy.transform(&mut ctx, input)
}
