use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::free_impl;
use crate::text::Text;
use crate::val::Val;

#[derive(Clone)]
pub(crate) struct SyntaxPrelude {
    pub(crate) parse: FreeStaticPrimFuncVal,
    pub(crate) generate: FreeStaticPrimFuncVal,
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

fn parse() -> FreeStaticPrimFuncVal {
    FreeFn { id: "syntax.parse", f: free_impl(fn_parse), mode: FuncMode::default() }.free_static()
}

fn fn_parse(input: Val) -> Val {
    let Val::Text(input) = input else {
        return Val::default();
    };
    crate::parse(&input).unwrap_or_default()
}

fn generate() -> FreeStaticPrimFuncVal {
    FreeFn { id: "syntax.generate", f: free_impl(fn_generate), mode: FuncMode::default() }
        .free_static()
}

fn fn_generate(input: Val) -> Val {
    let Ok(str) = crate::generate(&input) else {
        return Val::default();
    };
    Val::Text(Text::from(str).into())
}
