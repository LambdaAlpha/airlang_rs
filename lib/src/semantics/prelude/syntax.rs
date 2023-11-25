use crate::{
    semantics::{
        ctx::NameMap,
        eval_mode::EvalMode,
        input_mode::InputMode,
        prelude::{
            named_free_fn,
            Named,
            Prelude,
        },
        val::{
            FuncVal,
            Val,
        },
    },
    types::Str,
};

#[derive(Clone)]
pub(crate) struct SyntaxPrelude {
    pub(crate) parse: Named<FuncVal>,
    pub(crate) stringify: Named<FuncVal>,
}

impl Default for SyntaxPrelude {
    fn default() -> Self {
        SyntaxPrelude {
            parse: parse(),
            stringify: stringify(),
        }
    }
}

impl Prelude for SyntaxPrelude {
    fn put(&self, m: &mut NameMap) {
        self.parse.put(m);
        self.stringify.put(m);
    }
}

fn parse() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::More);
    named_free_fn("parse", input_mode, fn_parse)
}

fn fn_parse(input: Val) -> Val {
    let Val::String(input) = input else {
        return Val::default();
    };
    crate::semantics::parse(&input).unwrap_or_default()
}

fn stringify() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::More);
    named_free_fn("stringify", input_mode, fn_stringify)
}

fn fn_stringify(input: Val) -> Val {
    let Ok(str) = crate::semantics::generate(&input) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}
