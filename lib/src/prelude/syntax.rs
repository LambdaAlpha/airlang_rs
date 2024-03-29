use crate::{
    ctx::NameMap,
    prelude::{
        default_mode,
        named_free_fn,
        Named,
        Prelude,
    },
    string::Str,
    val::{
        func::FuncVal,
        Val,
    },
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
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("parse", input_mode, output_mode, fn_parse)
}

fn fn_parse(input: Val) -> Val {
    let Val::String(input) = input else {
        return Val::default();
    };
    crate::parse(&input).unwrap_or_default()
}

fn stringify() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("stringify", input_mode, output_mode, fn_stringify)
}

fn fn_stringify(input: Val) -> Val {
    let Ok(str) = crate::generate(&input) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}
