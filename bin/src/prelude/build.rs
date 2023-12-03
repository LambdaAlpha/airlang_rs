use {
    crate::{
        ctx::{
            ConstCtx,
            DynCtx,
        },
        eval::{
            eval,
            Cmd,
            Output,
        },
        prelude::{
            named_fn,
            Named,
            Prelude,
        },
    },
    airlang::semantics::{
        parse,
        Val,
    },
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

#[derive(Clone)]
pub(crate) struct BuildPrelude {
    pub(crate) import: Named<Rc<dyn Cmd>>,
}

impl Default for BuildPrelude {
    fn default() -> Self {
        Self {
            import: named_fn("repl.import", fn_import),
        }
    }
}

impl Prelude for BuildPrelude {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>) {
        self.import.put(m);
    }
}

fn fn_import(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, val: Val) -> Output {
    let Val::String(path) = val else {
        return Output::Eprint("import command only support string argument".into());
    };
    match std::fs::read_to_string(&*path) {
        Ok(input) => match parse(&input) {
            Ok(input) => eval(const_ctx, dyn_ctx, input),
            Err(err) => Output::Eprint(Box::new(err)),
        },
        Err(err) => Output::Eprint(Box::new(err)),
    }
}
