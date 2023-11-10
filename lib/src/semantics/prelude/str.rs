use crate::{
    semantics::{
        ctx::{
            DefaultCtx,
            NameMap,
        },
        ctx_access::constant::CtxForConstFn,
        eval_mode::EvalMode,
        input_mode::InputMode,
        prelude::{
            named_const_fn,
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
pub(crate) struct StrPrelude {
    length: Named<FuncVal>,
    concat: Named<FuncVal>,
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
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("string_length", input_mode, fn_length)
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
    let input_mode = InputMode::List(EvalMode::Eval);
    named_free_fn("string_concat", input_mode, fn_concat)
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
