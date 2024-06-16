use crate::{
    ctx::CtxMap,
    prelude::{
        named_free_fn,
        Named,
        Prelude,
    },
    syntax::ANNOTATE_INFIX,
    FuncVal,
    Mode,
    Pair,
    Val,
};

#[derive(Clone)]
pub(crate) struct AnnotatePrelude {
    pub(crate) new: Named<FuncVal>,
}

impl Default for AnnotatePrelude {
    fn default() -> Self {
        AnnotatePrelude { new: new() }
    }
}

impl Prelude for AnnotatePrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn(ANNOTATE_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Pair::from(pair).second
}
