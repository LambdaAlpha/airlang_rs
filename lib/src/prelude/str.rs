use crate::{
    ctx::{
        DefaultCtx,
        NameMap,
    },
    ctx_access::constant::CtxForConstFn,
    eval_mode::EvalMode,
    io_mode::IoMode,
    prelude::{
        named_const_fn,
        named_free_fn,
        Named,
        Prelude,
    },
    string::Str,
    val::{
        FuncVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct StrPrelude {
    pub(crate) length: Named<FuncVal>,
    pub(crate) concat: Named<FuncVal>,
}

impl Default for StrPrelude {
    fn default() -> Self {
        StrPrelude {
            length: length(),
            concat: concat(),
        }
    }
}

impl Prelude for StrPrelude {
    fn put(&self, m: &mut NameMap) {
        self.length.put(m);
        self.concat.put(m);
    }
}

fn length() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("string.length", input_mode, output_mode, fn_length)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::String(s) = val else {
            return Val::default();
        };
        Val::Int(s.len().into())
    })
}

fn concat() -> Named<FuncVal> {
    let input_mode = IoMode::List(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("string.concat", input_mode, output_mode, fn_concat)
}

fn fn_concat(input: Val) -> Val {
    let Val::List(strings) = input else {
        return Val::default();
    };
    let mut ret = String::new();
    for str in strings {
        let Val::String(str) = str else {
            return Val::default();
        };
        ret.push_str(&str);
    }
    Val::String(Str::from(ret))
}
