use crate::{
    ctx::NameMap,
    prelude::{
        default_mode,
        named_free_fn,
        pair_mode,
        Named,
        Prelude,
    },
    syntax::ANNOTATION_INFIX,
    FuncVal,
    Val,
};

#[derive(Clone)]
pub(crate) struct AnnotationPrelude {
    pub(crate) new: Named<FuncVal>,
}

impl Default for AnnotationPrelude {
    fn default() -> Self {
        AnnotationPrelude { new: new() }
    }
}

impl Prelude for AnnotationPrelude {
    fn put(&self, m: &mut NameMap) {
        self.new.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn(ANNOTATION_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    pair.second
}
