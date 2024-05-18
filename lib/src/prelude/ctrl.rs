use crate::{
    ctx::{
        CtxMap,
        CtxRef,
        CtxValue,
    },
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
        symbol_id_mode,
        Named,
        Prelude,
    },
    transform::{
        eval::Eval,
        Transform,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        Val,
    },
    Bytes,
    Call,
    Int,
    List,
    Map,
    Mode,
    Pair,
    Str,
    Symbol,
};

#[derive(Clone)]
pub(crate) struct CtrlPrelude {
    pub(crate) sequence: Named<FuncVal>,
    pub(crate) if1: Named<FuncVal>,
    pub(crate) if_not: Named<FuncVal>,
    pub(crate) match1: Named<FuncVal>,
    pub(crate) while1: Named<FuncVal>,
    pub(crate) while_not: Named<FuncVal>,
    pub(crate) for1: Named<FuncVal>,
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
            for1: for1(),
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
        self.for1.put(m);
    }
}

const BREAK: &str = "break";
const ELSE_BREAK: &str = "else_break";
const CONTINUE: &str = "continue";
const ELSE_CONTINUE: &str = "else_continue";

#[derive(Copy, Clone)]
enum Exit {
    Continue,
    Break,
}

#[derive(Copy, Clone)]
enum CtrlFlow {
    None,
    Error,
    Exit(Exit),
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

fn fn_sequence<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
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

fn fn_if<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let Val::Bool(b) = condition else {
        return Val::default();
    };
    let branches = Pair::from(branches);
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

fn fn_if_not<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let Val::Bool(b) = condition else {
        return Val::default();
    };
    let branches = Pair::from(branches);
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

fn fn_match<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = pair.first;
    let Val::Pair(pair) = pair.second else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Map(map) = pair.first else {
        return Val::default();
    };
    let default = pair.second;
    let map = Map::from(map);
    let eval = map
        .into_iter()
        .find_map(|(k, v)| {
            let k = Eval.transform(ctx.reborrow(), k);
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

fn fn_while<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let body = pair.second;
    loop {
        let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
            return Val::default();
        };
        if !b.bool() {
            break;
        }
        let (output, ctrl_flow) = block(ctx.reborrow(), body.clone());
        match ctrl_flow {
            CtrlFlow::None => {}
            CtrlFlow::Error => return Val::default(),
            CtrlFlow::Exit(exit) => match exit {
                Exit::Continue => {}
                Exit::Break => return output,
            },
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

fn fn_while_not<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let body = pair.second;
    loop {
        let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
            return Val::default();
        };
        if b.bool() {
            break;
        }
        let (output, ctrl_flow) = block(ctx.reborrow(), body.clone());
        match ctrl_flow {
            CtrlFlow::None => {}
            CtrlFlow::Error => return Val::default(),
            CtrlFlow::Exit(exit) => match exit {
                Exit::Continue => {}
                Exit::Break => return output,
            },
        }
    }
    Val::default()
}

fn for1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        default_mode(),
        pair_mode(symbol_id_mode(), Mode::Predefined(Transform::Id)),
    );
    let output_mode = default_mode();
    let func = MutableDispatcher::new(
        fn_for::<FreeCtx>,
        |ctx, val| fn_for(ctx, val),
        |ctx, val| fn_for(ctx, val),
    );
    named_mutable_fn("for", input_mode, output_mode, func)
}

fn fn_for<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxAccessor<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let iterable = pair.first;
    let Val::Pair(name_body) = pair.second else {
        return Val::default();
    };
    let name_body = Pair::from(name_body);
    let Val::Symbol(name) = name_body.first else {
        return Val::default();
    };
    let body = name_body.second;
    match iterable {
        Val::Int(i) => {
            let i = Int::from(i);
            if i.is_negative() {
                return Val::default();
            }
            let Some(i) = i.to_u128() else {
                panic!("iterate on super big int {i}!!!")
            };
            let iter = (0..i).map(|i| {
                let i = Int::from(i);
                Val::Int(i.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::Bytes(bytes) => {
            let iter = bytes.as_ref().iter().map(|byte| {
                let bytes = Bytes::from(std::slice::from_ref(byte));
                Val::Bytes(bytes.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::Symbol(s) => {
            let iter = s.char_indices().map(|(start, c)| {
                let symbol = Symbol::from_str(&s[start..start + c.len_utf8()]);
                Val::Symbol(symbol)
            });
            for_iter(ctx, body, name, iter)
        }
        Val::String(str) => {
            let iter = str.chars().map(|c| {
                let str = Str::from(c.to_string());
                Val::String(str.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::List(list) => {
            let list = List::from(list);
            let iter = list.into_iter();
            for_iter(ctx, body, name, iter)
        }
        Val::Map(map) => {
            let map = Map::from(map);
            let iter = map.into_iter().map(|pair| {
                let pair = Pair::new(pair.0, pair.1);
                Val::Pair(pair.into())
            });
            for_iter(ctx, body, name, iter)
        }
        _ => Val::default(),
    }
}

fn for_iter<'a, Ctx, ValIter>(mut ctx: Ctx, body: Val, name: Symbol, values: ValIter) -> Val
where
    Ctx: CtxAccessor<'a>,
    ValIter: Iterator<Item = Val>,
{
    for val in values {
        let _ = ctx.reborrow().put_value(name.clone(), CtxValue::new(val));
        let (output, ctrl_flow) = block(ctx.reborrow(), body.clone());
        match ctrl_flow {
            CtrlFlow::None => {}
            CtrlFlow::Error => return Val::default(),
            CtrlFlow::Exit(exit) => match exit {
                Exit::Continue => {}
                Exit::Break => return output,
            },
        }
    }
    Val::default()
}

fn block<'a, Ctx>(mut ctx: Ctx, input: Val) -> (Val, CtrlFlow)
where
    Ctx: CtxAccessor<'a>,
{
    let Val::List(list) = input else {
        return (Eval.transform(ctx, input), CtrlFlow::None);
    };
    let list = List::from(list);
    let mut output = Val::default();
    for val in list {
        let Val::Call(call) = val else {
            output = Eval.transform(ctx.reborrow(), val);
            continue;
        };
        let Val::Symbol(s) = &call.func else {
            output = Eval.transform(ctx.reborrow(), Val::Call(call));
            continue;
        };
        let Some((exit, target)) = parse_exit(s) else {
            output = Eval.transform(ctx.reborrow(), Val::Call(call));
            continue;
        };
        let call = Call::from(call);
        let Val::Pair(pair) = call.input else {
            return (Val::default(), CtrlFlow::Error);
        };
        let pair = Pair::from(pair);
        let condition = Eval.transform(ctx.reborrow(), pair.first);
        let Val::Bool(condition) = condition else {
            return (Val::default(), CtrlFlow::Error);
        };
        if condition.bool() == target {
            return (Eval.transform(ctx, pair.second), CtrlFlow::from(exit));
        }
    }
    (output, CtrlFlow::None)
}

fn parse_exit(str: &str) -> Option<(Exit, bool /* target */)> {
    let (exit, target) = match str {
        BREAK => (Exit::Break, true),
        ELSE_BREAK => (Exit::Break, false),
        CONTINUE => (Exit::Continue, true),
        ELSE_CONTINUE => (Exit::Continue, false),
        _ => {
            return None;
        }
    };
    Some((exit, target))
}

impl From<Exit> for CtrlFlow {
    fn from(value: Exit) -> Self {
        Self::Exit(value)
    }
}
