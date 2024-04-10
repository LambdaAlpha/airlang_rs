use crate::{
    ctx::CtxMap,
    ctx_access::{
        free::FreeCtx,
        CtxAccessor,
    },
    func::MutableDispatcher,
    prelude::{
        default_mode,
        map_all_mode,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    transform::{
        eval::{
            Eval,
            EvalByRef,
        },
        Transform,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        Val,
    },
    Mode,
};

#[derive(Clone)]
pub(crate) struct CtrlPrelude {
    pub(crate) sequence: Named<FuncVal>,
    pub(crate) if1: Named<FuncVal>,
    pub(crate) if_not: Named<FuncVal>,
    pub(crate) match1: Named<FuncVal>,
    pub(crate) while1: Named<FuncVal>,
    pub(crate) while_not: Named<FuncVal>,
}

impl Default for CtrlPrelude {
    fn default() -> Self {
        CtrlPrelude {
            sequence: sequence(),
            if1: if1(),
            if_not: if_not(),
            match1: match1(),
            while1: while1(),
            while_not: while_not(),
        }
    }
}

impl Prelude for CtrlPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.sequence.put(m);
        self.if1.put(m);
        self.if_not.put(m);
        self.match1.put(m);
        self.while1.put(m);
        self.while_not.put(m);
    }
}

const BREAK: &str = "break";
const ELSE_BREAK: &str = "else_break";
const CONTINUE: &str = "continue";
const ELSE_CONTINUE: &str = "else_continue";

#[derive(Copy, Clone)]
enum CtrlFlowTag {
    Error,
    None,
    Continue,
    Break,
}

fn sequence() -> Named<FuncVal> {
    let input_mode = Mode::Predefined(Transform::Id);
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_sequence::<FreeCtx>,
        |ctx, val| fn_sequence(ctx, val),
        |ctx, val| fn_sequence(ctx, val),
    );
    named_mutable_fn(";", input_mode, output_mode, func)
}

fn fn_sequence<Ctx: CtxAccessor>(ctx: Ctx, input: Val) -> Val {
    block(ctx, input).0
}

fn if1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        default_mode(),
        pair_mode(
            Mode::Predefined(Transform::Id),
            Mode::Predefined(Transform::Id),
        ),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_if::<FreeCtx>,
        |ctx, val| fn_if(ctx, val),
        |ctx, val| fn_if(ctx, val),
    );
    named_mutable_fn("if", input_mode, output_mode, func)
}

fn fn_if<Ctx: CtxAccessor>(ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let condition = pair.first;
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let Val::Bool(b) = condition else {
        return Val::default();
    };
    let branch = if b.bool() {
        branches.first
    } else {
        branches.second
    };
    block(ctx, branch).0
}

fn if_not() -> Named<FuncVal> {
    let input_mode = pair_mode(
        default_mode(),
        pair_mode(
            Mode::Predefined(Transform::Id),
            Mode::Predefined(Transform::Id),
        ),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_if_not::<FreeCtx>,
        |ctx, val| fn_if_not(ctx, val),
        |ctx, val| fn_if_not(ctx, val),
    );
    named_mutable_fn("if_not", input_mode, output_mode, func)
}

fn fn_if_not<Ctx: CtxAccessor>(ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let condition = pair.first;
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let Val::Bool(b) = condition else {
        return Val::default();
    };
    let branch = if b.bool() {
        branches.second
    } else {
        branches.first
    };
    block(ctx, branch).0
}

fn match1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        default_mode(),
        pair_mode(
            map_all_mode(
                Mode::Predefined(Transform::Id),
                Mode::Predefined(Transform::Id),
            ),
            Mode::Predefined(Transform::Id),
        ),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_match::<FreeCtx>,
        |ctx, val| fn_match(ctx, val),
        |ctx, val| fn_match(ctx, val),
    );
    named_mutable_fn("match", input_mode, output_mode, func)
}

fn fn_match<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let val = pair.first;
    let Val::Pair(pair) = pair.second else {
        return Val::default();
    };
    let Val::Map(map) = pair.first else {
        return Val::default();
    };
    let default = pair.second;
    let eval = map
        .into_iter()
        .find_map(|(k, v)| {
            let k = Eval.transform(&mut ctx, k);
            if k == val { Some(v) } else { None }
        })
        .unwrap_or(default);
    block(ctx, eval).0
}

fn while1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::Predefined(Transform::Id),
        Mode::Predefined(Transform::Id),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_while::<FreeCtx>,
        |ctx, val| fn_while(ctx, val),
        |ctx, val| fn_while(ctx, val),
    );
    named_mutable_fn("while", input_mode, output_mode, func)
}

fn fn_while<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let condition = pair.first;
    let body = pair.second;
    loop {
        let Val::Bool(b) = EvalByRef.transform(&mut ctx, &condition) else {
            return Val::default();
        };
        if !b.bool() {
            break;
        }
        let (output, tag) = block_by_ref(&mut ctx, &body);
        match tag {
            CtrlFlowTag::Error => return output,
            CtrlFlowTag::None => {}
            CtrlFlowTag::Continue => {}
            CtrlFlowTag::Break => return output,
        }
    }
    Val::default()
}

fn while_not() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::Predefined(Transform::Id),
        Mode::Predefined(Transform::Id),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_while_not::<FreeCtx>,
        |ctx, val| fn_while_not(ctx, val),
        |ctx, val| fn_while_not(ctx, val),
    );
    named_mutable_fn("while_not", input_mode, output_mode, func)
}

fn fn_while_not<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let condition = pair.first;
    let body = pair.second;
    loop {
        let Val::Bool(b) = EvalByRef.transform(&mut ctx, &condition) else {
            return Val::default();
        };
        if b.bool() {
            break;
        }
        let (output, tag) = block_by_ref(&mut ctx, &body);
        match tag {
            CtrlFlowTag::Error => return output,
            CtrlFlowTag::None => {}
            CtrlFlowTag::Continue => {}
            CtrlFlowTag::Break => return output,
        }
    }
    Val::default()
}

fn block<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> (Val, CtrlFlowTag) {
    let Val::List(list) = input else {
        return (Eval.transform(&mut ctx, input), CtrlFlowTag::None);
    };
    let mut output = Val::default();
    for val in list {
        let Val::Call(call) = val else {
            output = Eval.transform(&mut ctx, val);
            continue;
        };
        let Val::Symbol(s) = &call.func else {
            output = Eval.transform(&mut ctx, Val::Call(call));
            continue;
        };
        let Some((tag, target)) = parse_tag(s) else {
            output = Eval.transform(&mut ctx, Val::Call(call));
            continue;
        };
        let Val::Pair(pair) = call.input else {
            return (Val::default(), CtrlFlowTag::Error);
        };
        let condition = Eval.transform(&mut ctx, pair.first);
        let Val::Bool(condition) = condition else {
            return (Val::default(), CtrlFlowTag::Error);
        };
        if condition.bool() == target {
            return (Eval.transform(&mut ctx, pair.second), tag);
        }
    }
    (output, CtrlFlowTag::None)
}

fn block_by_ref<Ctx: CtxAccessor>(ctx: &mut Ctx, input: &Val) -> (Val, CtrlFlowTag) {
    let Val::List(list) = input else {
        return (EvalByRef.transform(ctx, input), CtrlFlowTag::None);
    };
    let mut output = Val::default();
    for val in list {
        let Val::Call(call) = val else {
            output = EvalByRef.transform(ctx, val);
            continue;
        };
        let Val::Symbol(s) = &call.func else {
            output = EvalByRef.transform(ctx, val);
            continue;
        };
        let Some((tag, target)) = parse_tag(s) else {
            output = EvalByRef.transform(ctx, val);
            continue;
        };
        let Val::Pair(pair) = &call.input else {
            return (Val::default(), CtrlFlowTag::Error);
        };
        let condition = EvalByRef.transform(ctx, &pair.first);
        let Val::Bool(condition) = condition else {
            return (Val::default(), CtrlFlowTag::Error);
        };
        if condition.bool() == target {
            return (EvalByRef.transform(ctx, &pair.second), tag);
        }
    }
    (output, CtrlFlowTag::None)
}

fn parse_tag(tag: &str) -> Option<(CtrlFlowTag, bool /* target */)> {
    let (tag, target) = match tag {
        BREAK => (CtrlFlowTag::Break, true),
        ELSE_BREAK => (CtrlFlowTag::Break, false),
        CONTINUE => (CtrlFlowTag::Continue, true),
        ELSE_CONTINUE => (CtrlFlowTag::Continue, false),
        _ => {
            return None;
        }
    };
    Some((tag, target))
}
