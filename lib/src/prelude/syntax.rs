use crate::{
    ctx::CtxValue,
    prelude::{
        named_free_fn,
        Named,
        Prelude,
    },
    text::Text,
    val::{
        func::FuncVal,
        Val,
    },
    Map,
    Mode,
    Symbol,
};

#[derive(Clone)]
pub(crate) struct SyntaxPrelude {
    pub(crate) parse: Named<FuncVal>,
    pub(crate) generate: Named<FuncVal>,
}

impl Default for SyntaxPrelude {
    fn default() -> Self {
        SyntaxPrelude {
            parse: parse(),
            generate: generate(),
        }
    }
}

impl Prelude for SyntaxPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.parse.put(m);
        self.generate.put(m);
    }
}

fn parse() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("parse", input_mode, output_mode, true, fn_parse)
}

fn fn_parse(input: Val) -> Val {
    let Val::Text(input) = input else {
        return Val::default();
    };
    crate::parse(&input).unwrap_or_default()
}

fn generate() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("generate", input_mode, output_mode, true, fn_generate)
}

fn fn_generate(input: Val) -> Val {
    let Ok(str) = crate::generate(&input) else {
        return Val::default();
    };
    Val::Text(Text::from(str).into())
}
