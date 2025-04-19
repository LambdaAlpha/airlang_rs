use std::mem::swap;

use crate::{
    Bit,
    ConstFnCtx,
    Either,
    FuncMode,
    Map,
    Pair,
    Symbol,
    ctx::{
        main::MainCtx,
        map::CtxValue,
        mut1::MutFnCtx,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    syntax::{
        EITHER_THAT,
        EITHER_THIS,
    },
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct EitherPrelude {
    pub(crate) this: Named<FuncVal>,
    pub(crate) that: Named<FuncVal>,
    pub(crate) is_this: Named<FuncVal>,
    pub(crate) is_that: Named<FuncVal>,
    pub(crate) get: Named<FuncVal>,
    pub(crate) set: Named<FuncVal>,
}

impl Default for EitherPrelude {
    fn default() -> Self {
        EitherPrelude {
            this: this(),
            that: that(),
            is_this: is_this(),
            is_that: is_that(),
            get: get(),
            set: set(),
        }
    }
}

impl Prelude for EitherPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.this.put(m);
        self.that.put(m);
        self.is_this.put(m);
        self.is_that.put(m);
        self.get.put(m);
        self.set.put(m);
    }
}

fn this() -> Named<FuncVal> {
    let id = EITHER_THIS;
    let f = fn_this;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_this(input: Val) -> Val {
    Val::Either(Either::This(input).into())
}

fn that() -> Named<FuncVal> {
    let id = EITHER_THAT;
    let f = fn_that;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_that(input: Val) -> Val {
    Val::Either(Either::That(input).into())
}

fn is_this() -> Named<FuncVal> {
    let id = "either.is_this";
    let f = fn_is_this;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_this(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref(ctx, pair.first, |v| {
        let Val::Either(either) = v else {
            return Val::default();
        };
        Val::Bit(Bit::new(matches!(&**either, Either::This(_))))
    })
}

fn is_that() -> Named<FuncVal> {
    let id = "either.is_that";
    let f = fn_is_that;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_is_that(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref(ctx, pair.first, |v| {
        let Val::Either(either) = v else {
            return Val::default();
        };
        Val::Bit(Bit::new(matches!(&**either, Either::That(_))))
    })
}

fn get() -> Named<FuncVal> {
    let id = "either.get";
    let f = fn_get;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_get(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref(ctx, pair.first, |v| {
        let Val::Either(either) = v else {
            return Val::default();
        };
        match &**either {
            Either::This(v) => v.clone(),
            Either::That(v) => v.clone(),
        }
    })
}

fn set() -> Named<FuncVal> {
    let id = "either.set";
    let f = fn_set;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_set(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    MainCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut either) => {
            let Some(Val::Either(either)) = either.as_mut() else {
                return Val::default();
            };
            match &mut **either {
                Either::This(this) => swap(this, &mut val),
                Either::That(that) => swap(that, &mut val),
            }
            val
        }
        Either::That(_) => Val::default(),
    })
}
