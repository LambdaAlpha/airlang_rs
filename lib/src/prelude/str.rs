use crate::{
    ctx::{
        DefaultCtx,
        NameMap,
    },
    ctx_access::constant::CtxForConstFn,
    eval_mode::EvalMode,
    io_mode::ListMode,
    prelude::{
        default_mode,
        list_mode,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        symbol_value_mode,
        Named,
        Prelude,
    },
    string::Str,
    val::{
        func::FuncVal,
        Val,
    },
    Bytes,
    CtxForMutableFn,
};

#[derive(Clone)]
pub(crate) struct StrPrelude {
    pub(crate) from_utf8: Named<FuncVal>,
    pub(crate) to_bytes: Named<FuncVal>,
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) concat: Named<FuncVal>,
}

impl Default for StrPrelude {
    fn default() -> Self {
        StrPrelude {
            from_utf8: from_utf8(),
            to_bytes: to_bytes(),
            length: length(),
            push: push(),
            concat: concat(),
        }
    }
}

impl Prelude for StrPrelude {
    fn put(&self, m: &mut NameMap) {
        self.from_utf8.put(m);
        self.to_bytes.put(m);
        self.length.put(m);
        self.push.put(m);
        self.concat.put(m);
    }
}

fn from_utf8() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("string.from_utf8", input_mode, output_mode, fn_from_utf8)
}

fn fn_from_utf8(input: Val) -> Val {
    let Val::Bytes(bytes) = input else {
        return Val::default();
    };
    if let Ok(str) = String::from_utf8(bytes.into()) {
        Val::String(Str::from(str))
    } else {
        Val::default()
    }
}

fn to_bytes() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("string.to_bytes", input_mode, output_mode, fn_to_bytes)
}

fn fn_to_bytes(input: Val) -> Val {
    let Val::String(str) = input else {
        return Val::default();
    };
    Val::Bytes(Bytes::from(String::from(str).into_bytes()))
}

fn length() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
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

fn push() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_value_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("string.push", input_mode, output_mode, fn_push)
}

fn fn_push(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::String(s) = pair.second else {
        return Val::default();
    };
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, pair.first, |val| {
        let Val::String(str) = val else {
            return;
        };
        str.push_str(&s);
    })
}

fn concat() -> Named<FuncVal> {
    let input_mode = list_mode(ListMode::Eval(EvalMode::Eager));
    let output_mode = default_mode();
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
