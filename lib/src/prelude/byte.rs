use crate::{
    Byte,
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Int,
    Map,
    MutFnCtx,
    Pair,
    Symbol,
    Val,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
};

#[derive(Clone)]
pub(crate) struct BytePrelude {
    pub(crate) length: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) join: Named<FuncVal>,
}

#[allow(clippy::derivable_impls)]
impl Default for BytePrelude {
    fn default() -> Self {
        BytePrelude { length: length(), push: push(), join: join() }
    }
}

impl Prelude for BytePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.length.put(m);
        self.push.put(m);
        self.join.put(m);
    }
}

fn length() -> Named<FuncVal> {
    let id = "byte.length";
    let f = fn_length;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Byte(t) = val else {
            return Val::default();
        };
        let len: Int = t.as_ref().len().into();
        Val::Int(len.into())
    })
}

fn push() -> Named<FuncVal> {
    let id = "byte.push";
    let f = fn_push;
    let call = ref_pair_mode();
    let abstract1 = call.clone();
    let ask = FuncMode::default_mode();
    let mode = FuncMode { call, abstract1, ask };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_push(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Byte(b) = pair.second else {
        return Val::default();
    };
    DefaultCtx::with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::Byte(byte) = val else {
            return;
        };
        byte.push(&b);
    })
}

fn join() -> Named<FuncVal> {
    let id = "byte.join";
    let f = fn_join;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_join(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let separator: &[u8] = match &pair.first {
        Val::Unit(_) => &[],
        Val::Byte(b) => b,
        _ => return Val::default(),
    };
    let Val::List(bytes) = &pair.second else {
        return Val::default();
    };
    let bytes: Option<Vec<&[u8]>> = bytes
        .iter()
        .map(|v| {
            let Val::Byte(b) = v else {
                return None;
            };
            let byte: &[u8] = b;
            Some(byte)
        })
        .collect();
    let Some(bytes) = bytes else {
        return Val::default();
    };
    let byte = bytes.join(separator);
    Val::Byte(Byte::from(byte).into())
}
