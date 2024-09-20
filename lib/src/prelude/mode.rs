use crate::{
    ctx::{
        free::FreeCtx,
        mut1::MutFnCtx,
        CtxValue,
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
        named_mut_fn,
        named_static_fn,
        Named,
        Prelude,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        Val,
    },
    Map,
    Mode,
    Symbol,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.id.put(m);
        self.form.put(m);
        self.eval.put(m);
    }
}

fn id() -> Named<FuncVal> {
    let call_mode = id_mode();
    let ask_mode = Mode::default();
    named_static_fn(ID, call_mode, ask_mode, true, fn_id)
}

fn fn_id(input: Val) -> Val {
    Id.transform(FreeCtx, input)
}

fn form() -> Named<FuncVal> {
    let call_mode = id_mode();
    let ask_mode = Mode::default();
    named_mut_fn(FORM, call_mode, ask_mode, true, fn_form)
}

fn fn_form(ctx: MutFnCtx, input: Val) -> Val {
    Form.transform(ctx, input)
}

fn eval() -> Named<FuncVal> {
    let call_mode = id_mode();
    let ask_mode = Mode::default();
    named_mut_fn(EVAL, call_mode, ask_mode, false, fn_eval)
}

fn fn_eval(ctx: MutFnCtx, input: Val) -> Val {
    Eval.transform(ctx, input)
}
