use std::mem::swap;

use crate::{
    ctx::{
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
        ref1::CtxMeta,
        CtxValue,
    },
    func::mut1::MutDispatcher,
    mode::{
        basic::BasicMode,
        eval::Eval,
    },
    prelude::{
        form_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
        Named,
        Prelude,
    },
    syntax::ASK_INFIX,
    types::either::Either,
    val::func::FuncVal,
    Ask,
    FreeCtx,
    Map,
    Mode,
    Pair,
    Symbol,
    Val,
};

#[derive(Clone)]
pub(crate) struct AskPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) new_dependent: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_output: Named<FuncVal>,
    pub(crate) set_output: Named<FuncVal>,
}

impl Default for AskPrelude {
    fn default() -> Self {
        AskPrelude {
            new: new(),
            new_dependent: new_dependent(),
            apply: apply(),
            get_func: get_func(),
            set_func: set_func(),
            get_output: get_output(),
            set_output: set_output(),
        }
    }
}

impl Prelude for AskPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.new.put(m);
        self.new_dependent.put(m);
        self.apply.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
        self.get_output.put(m);
        self.set_output.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn(ASK_INFIX, input_mode, output_mode, true, fn_new)
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    Val::Ask(Ask::new(pair.first, pair.second).into())
}

fn new_dependent() -> Named<FuncVal> {
    let input_mode = pair_mode(Mode::default(), form_mode(), BasicMode::default());
    let output_mode = Mode::default();
    named_mut_fn("??", input_mode, output_mode, true, fn_new_dependent)
}

fn fn_new_dependent(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let func = pair.first;
    let output = pair.second;
    let output = Eval.eval_output(ctx, &func, output);
    Val::Ask(Ask::new(func, output).into())
}

fn apply() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mut_fn("ask.apply", input_mode, output_mode, false, func)
}

fn fn_apply<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Ask(ask) = input else {
        return Val::default();
    };
    let ask = Ask::from(ask);
    Eval::solve(ctx, ask.func, ask.output)
}

fn get_func() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("ask.function", input_mode, output_mode, true, fn_get_func)
}

fn fn_get_func(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Ask(ask) => ask.func.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Ask(ask) => Ask::from(ask).func,
            _ => Val::default(),
        },
    })
}

fn set_func() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn(
        "ask.set_function",
        input_mode,
        output_mode,
        true,
        fn_set_func,
    )
}

fn fn_set_func(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut ask) => {
            let Some(Val::Ask(ask)) = ask.as_mut() else {
                return Val::default();
            };
            swap(&mut ask.func, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

fn get_output() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("ask.output", input_mode, output_mode, true, fn_get_output)
}

fn fn_get_output(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_dyn(ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val.as_const() {
            Val::Ask(ask) => ask.output.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Ask(ask) => Ask::from(ask).output,
            _ => Val::default(),
        },
    })
}

fn set_output() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn(
        "ask.set_output",
        input_mode,
        output_mode,
        true,
        fn_set_output,
    )
}

fn fn_set_output(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name_val = Pair::from(name_val);
    let name = name_val.first;
    let mut val = name_val.second;
    DefaultCtx.with_dyn(ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut ask) => {
            let Some(Val::Ask(ask)) = ask.as_mut() else {
                return Val::default();
            };
            swap(&mut ask.output, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
