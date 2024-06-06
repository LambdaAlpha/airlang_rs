use std::mem::swap;

use crate::{
    ctx::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        ref1::CtxMeta,
        CtxMap,
        DefaultCtx,
    },
    func::MutableDispatcher,
    prelude::{
        ask_mode,
        default_mode,
        named_const_fn,
        named_mutable_fn,
        pair_mode,
        symbol_id_mode,
        Named,
        Prelude,
    },
    syntax::ASK_INFIX,
    transform::eval::Eval,
    types::either::Either,
    val::func::FuncVal,
    Ask,
    FreeCtx,
    Mode,
    Pair,
    Transform,
    Val,
};

#[derive(Clone)]
pub(crate) struct AskPrelude {
    pub(crate) new: Named<FuncVal>,
    pub(crate) get_func: Named<FuncVal>,
    pub(crate) set_func: Named<FuncVal>,
    pub(crate) get_output: Named<FuncVal>,
    pub(crate) set_output: Named<FuncVal>,
    pub(crate) apply: Named<FuncVal>,
}

impl Default for AskPrelude {
    fn default() -> Self {
        AskPrelude {
            new: new(),
            get_func: get_func(),
            set_func: set_func(),
            get_output: get_output(),
            set_output: set_output(),
            apply: apply(),
        }
    }
}

impl Prelude for AskPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.new.put(m);
        self.get_func.put(m);
        self.set_func.put(m);
        self.get_output.put(m);
        self.set_output.put(m);
        self.apply.put(m);
    }
}

fn new() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), Mode::Predefined(Transform::Id));
    let output_mode = default_mode();
    named_mutable_fn(ASK_INFIX, input_mode, output_mode, fn_new)
}

fn fn_new(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let func = pair.first;
    let output = pair.second;
    let output = Eval.eval_output(ctx, &func, output);
    Val::Ask(Ask::new(func, output).into())
}

fn get_func() -> Named<FuncVal> {
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("ask.function", input_mode, output_mode, fn_get_func)
}

fn fn_get_func(ctx: CtxForConstFn, input: Val) -> Val {
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
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("ask.set_function", input_mode, output_mode, fn_set_func)
}

fn fn_set_func(ctx: CtxForMutableFn, input: Val) -> Val {
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
    let input_mode = symbol_id_mode();
    let output_mode = default_mode();
    named_const_fn("ask.output", input_mode, output_mode, fn_get_output)
}

fn fn_get_output(ctx: CtxForConstFn, input: Val) -> Val {
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
    let input_mode = pair_mode(symbol_id_mode(), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("ask.set_output", input_mode, output_mode, fn_set_output)
}

fn fn_set_output(ctx: CtxForMutableFn, input: Val) -> Val {
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

fn apply() -> Named<FuncVal> {
    let input_mode = ask_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_apply::<FreeCtx>,
        |ctx, val| fn_apply(ctx, val),
        |ctx, val| fn_apply(ctx, val),
    );
    named_mutable_fn("ask.apply", input_mode, output_mode, func)
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
