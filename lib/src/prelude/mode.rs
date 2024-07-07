use crate::{
    ctx::{
        free::FreeCtx,
        mut1::MutFnCtx,
        CtxMap,
    },
    mode::{
        basic::{
            EVAL,
            FORM,
            ID,
        },
        eval::Eval,
        form::Form,
        id::Id,
    },
    prelude::{
        id_mode,
        named_free_fn,
        named_mut_fn,
        Named,
        Prelude,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        Val,
    },
    Mode,
};

#[derive(Clone)]
pub(crate) struct ModePrelude {
    pub(crate) id: Named<FuncVal>,
    pub(crate) form: Named<FuncVal>,
    pub(crate) eval: Named<FuncVal>,
}

impl Default for ModePrelude {
    fn default() -> Self {
        ModePrelude {
            id: id(),
            form: form(),
            eval: eval(),
        }
    }
}

impl Prelude for ModePrelude {
    fn put(&self, m: &mut CtxMap) {
        self.id.put(m);
        self.form.put(m);
        self.eval.put(m);
    }
}

fn id() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    named_free_fn(ID, input_mode, output_mode, fn_id)
}

fn fn_id(input: Val) -> Val {
    Id.transform(FreeCtx, input)
}

fn form() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    named_mut_fn(FORM, input_mode, output_mode, fn_form)
}

fn fn_form(ctx: MutFnCtx, input: Val) -> Val {
    Form.transform(ctx, input)
}

fn eval() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    named_mut_fn(EVAL, input_mode, output_mode, fn_eval)
}

fn fn_eval(ctx: MutFnCtx, input: Val) -> Val {
    Eval.transform(ctx, input)
}
