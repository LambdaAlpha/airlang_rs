use std::mem::swap;

use crate::{
    ConstFnCtx,
    FreeCtx,
    FuncMode,
    Map,
    Pair,
    Solve,
    Symbol,
    Val,
    core::EvalCore,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    func::mut_static_prim::MutDispatcher,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    syntax::SOLVE,
    types::either::Either,
    val::func::FuncVal,
};

#[derive(Clone)]
pub(crate) struct SolvePrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_output: Named<FuncVal>,
    pub(crate) set_output: Named<FuncVal>,
}

impl Default for SolvePrelude {
    fn default() -> Self {
        SolvePrelude {
            new: new(),
            apply: apply(),
            get_func: get_func(),
            set_func: set_func(),
            get_output: get_output(),
            set_output: set_output(),
        }
    }
}

impl Prelude for SolvePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.apply.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
        self.get_output.put(m);
        self.set_output.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let id = SOLVE;
    let f = fn_new;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Solve(Solve::new(pair.first, pair.second).into())
}

fn apply() -> Named<FuncVal> {
    let id = "solve.apply";
    let f = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    let mode = FuncMode::default();
    let cacheable = false;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    let Val::Solve(solve) = input else {
        return Val::default();
    };
    let solve = Solve::from(solve);
    EvalCore::solve(ctx, solve.func, solve.output)
}

fn get_func() -> Named<FuncVal> {
    let id = "solve.function";
    let f = fn_get_func;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Solve(solve) => solve.func.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Solve(solve) => Solve::from(solve).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let id = "solve.set_function";
    let f = fn_set_func;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut solve) => {
            let Some(Val::Solve(solve)) = solve.as_mut() else {
                return Val::default();
            };
            swap(&mut solve.func, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}

fn get_output() -> Named<FuncVal> {
    let id = "solve.output";
    let f = fn_get_output;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_get_output(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_dyn(ctx, pair.first, |ref_or_val| match ref_or_val {
        Either::This(val) => match val.as_const() {
            Val::Solve(solve) => solve.output.clone(),
            _ => Val::default(),
        },
        Either::That(val) => match val {
            Val::Solve(solve) => Solve::from(solve).output,
            _ => Val::default(),
        },
    })
}

fn set_output() -> Named<FuncVal> {
    let id = "solve.set_output";
    let f = fn_set_output;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_set_output(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx::with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::This(mut solve) => {
            let Some(Val::Solve(solve)) = solve.as_mut() else {
                return Val::default();
            };
            swap(&mut solve.output, &mut val);
            val
        }
        Either::That(_) => Val::default(),
    })
}
