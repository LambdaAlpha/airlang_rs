use crate::{
    FuncMode,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_free_fn,
    },
    text::Text,
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct SyntaxPrelude {
    pub(crate) parse: Named<FuncVal>,
    pub(crate) generate: Named<FuncVal>,
}

impl Default for SyntaxPrelude {
    fn default() -> Self {
        SyntaxPrelude { parse: parse(), generate: generate() }
    }
}

impl Prelude for SyntaxPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.parse.put(ctx);
        self.generate.put(ctx);
    }
}

fn parse() -> Named<FuncVal> {
    let id = "syntax.parse";
    let f = fn_parse;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_parse(input: Val) -> Val {
    let Val::Text(input) = input else {
        return Val::default();
    };
    crate::parse(&input).unwrap_or_default()
}

fn generate() -> Named<FuncVal> {
    let id = "syntax.generate";
    let f = fn_generate;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_generate(input: Val) -> Val {
    let Ok(str) = crate::generate(&input) else {
        return Val::default();
    };
    Val::Text(Text::from(str).into())
}
