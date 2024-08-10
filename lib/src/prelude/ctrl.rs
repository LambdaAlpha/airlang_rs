use crate::{
    ctx::{
        free::FreeCtx,
        map::CtxMapRef,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
    },
    func::mut1::MutDispatcher,
    mode::{
        basic::BasicMode,
        eval::Eval,
    },
    prelude::{
        form_mode,
        id_mode,
        map_mode,
        named_mut_fn,
        pair_mode,
        Named,
        Prelude,
    },
    transformer::Transformer,
    val::{
        func::FuncVal,
        Val,
    },
    Byte,
    Call,
    Int,
    List,
    Map,
    Mode,
    Pair,
    Symbol,
    Text,
};

#[derive(Clone)]
pub(crate) struct CtrlPrelude {
    pub(crate) sequence: Named<FuncVal>,
    pub(crate) if1: Named<FuncVal>,
    pub(crate) if_not: Named<FuncVal>,
    pub(crate) match1: Named<FuncVal>,
    pub(crate) match_ordered: Named<FuncVal>,
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
            match_ordered: match_ordered(),
            while1: while1(),
            while_not: while_not(),
            for1: for1(),
        }
    }
}

impl Prelude for CtrlPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.sequence.put(m);
        self.if1.put(m);
        self.if_not.put(m);
        self.match1.put(m);
        self.match_ordered.put(m);
        self.while1.put(m);
        self.while_not.put(m);
        self.for1.put(m);
    }
}

const BREAK: &str = "break";
const UNIT_BREAK: &str = ".break";
const ELSE_BREAK: &str = "else_break";
const UNIT_ELSE_BREAK: &str = ".else_break";
const CONTINUE: &str = "continue";
const UNIT_CONTINUE: &str = ".continue";
const ELSE_CONTINUE: &str = "else_continue";
const UNIT_ELSE_CONTINUE: &str = ".else_continue";

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

#[derive(Clone)]
enum BlockItem {
    Normal(Val),
    UnitExit {
        exit: Exit,
        target: bool,
        body: Val,
    },
    BoolExit {
        exit: Exit,
        target: bool,
        condition: Val,
        body: Val,
    },
}

fn sequence() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_sequence::<FreeCtx>,
        |ctx, val| fn_sequence(ctx, val),
        |ctx, val| fn_sequence(ctx, val),
    );
    named_mut_fn(";", input_mode, output_mode, false, func)
}

fn fn_sequence<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    eval_block(ctx, input).0
}

fn if1() -> Named<FuncVal> {
    let input_mode = pair_mode(Mode::default(), id_mode(), BasicMode::default());
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_if::<FreeCtx>,
        |ctx, val| fn_if(ctx, val),
        |ctx, val| fn_if(ctx, val),
    );
    named_mut_fn("if", input_mode, output_mode, false, func)
}

fn fn_if<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
    eval_block(ctx, branch).0
}

fn if_not() -> Named<FuncVal> {
    let input_mode = pair_mode(Mode::default(), id_mode(), BasicMode::default());
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_if_not::<FreeCtx>,
        |ctx, val| fn_if_not(ctx, val),
        |ctx, val| fn_if_not(ctx, val),
    );
    named_mut_fn("if_not", input_mode, output_mode, false, func)
}

fn fn_if_not<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
    eval_block(ctx, branch).0
}

fn match1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::default(),
        pair_mode(
            map_mode(Map::default(), form_mode(), id_mode(), BasicMode::default()),
            id_mode(),
            BasicMode::default(),
        ),
        BasicMode::default(),
    );
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_match::<FreeCtx>,
        |ctx, val| fn_match(ctx, val),
        |ctx, val| fn_match(ctx, val),
    );
    named_mut_fn("match", input_mode, output_mode, false, func)
}

fn fn_match<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
    let mut map = Map::from(map);
    let eval = map.remove(&val).unwrap_or(default);
    eval_block(ctx, eval).0
}

fn match_ordered() -> Named<FuncVal> {
    let input_mode = pair_mode(Mode::default(), id_mode(), BasicMode::default());
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_match_ordered::<FreeCtx>,
        |ctx, val| fn_match_ordered(ctx, val),
        |ctx, val| fn_match_ordered(ctx, val),
    );
    named_mut_fn(";match", input_mode, output_mode, false, func)
}

fn fn_match_ordered<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
    let Val::List(list) = pair.first else {
        return Val::default();
    };
    let list = List::from(list);
    let arms = list
        .into_iter()
        .map(|val| {
            let Val::Pair(pair) = val else {
                return None;
            };
            Some(pair)
        })
        .collect::<Option<Vec<_>>>();
    let Some(arms) = arms else {
        return Val::default();
    };
    for arm in arms {
        let arm = Pair::from(arm);
        let v = Eval.transform(ctx.reborrow(), arm.first);
        if v == val {
            return eval_block(ctx, arm.second).0;
        }
    }
    let default = pair.second;
    eval_block(ctx, default).0
}

fn while1() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_while::<FreeCtx>,
        |ctx, val| fn_while(ctx, val),
        |ctx, val| fn_while(ctx, val),
    );
    named_mut_fn("while", input_mode, output_mode, false, func)
}

fn fn_while<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let body = pair.second;
    if let Val::List(body) = body {
        let body = List::from(body);
        let block_items: Option<List<BlockItem>> = body.into_iter().map(parse_block_item).collect();
        let Some(block_items) = block_items else {
            return Val::default();
        };
        loop {
            let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
                return Val::default();
            };
            if !b.bool() {
                break;
            }
            let (output, ctrl_flow) = eval_block_items(ctx.reborrow(), block_items.clone());
            match ctrl_flow {
                CtrlFlow::None => {}
                CtrlFlow::Error => return Val::default(),
                CtrlFlow::Exit(exit) => match exit {
                    Exit::Continue => {}
                    Exit::Break => return output,
                },
            }
        }
    } else {
        loop {
            let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
                return Val::default();
            };
            if !b.bool() {
                break;
            }
            Eval.transform(ctx.reborrow(), body.clone());
        }
    }
    Val::default()
}

fn while_not() -> Named<FuncVal> {
    let input_mode = id_mode();
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_while_not::<FreeCtx>,
        |ctx, val| fn_while_not(ctx, val),
        |ctx, val| fn_while_not(ctx, val),
    );
    named_mut_fn("while_not", input_mode, output_mode, false, func)
}

fn fn_while_not<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
{
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let body = pair.second;
    if let Val::List(body) = body {
        let body = List::from(body);
        let block_items: Option<List<BlockItem>> = body.into_iter().map(parse_block_item).collect();
        let Some(block_items) = block_items else {
            return Val::default();
        };
        loop {
            let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
                return Val::default();
            };
            if b.bool() {
                break;
            }
            let (output, ctrl_flow) = eval_block_items(ctx.reborrow(), block_items.clone());
            match ctrl_flow {
                CtrlFlow::None => {}
                CtrlFlow::Error => return Val::default(),
                CtrlFlow::Exit(exit) => match exit {
                    Exit::Continue => {}
                    Exit::Break => return output,
                },
            }
        }
    } else {
        loop {
            let Val::Bool(b) = Eval.transform(ctx.reborrow(), condition.clone()) else {
                return Val::default();
            };
            if b.bool() {
                break;
            }
            Eval.transform(ctx.reborrow(), body.clone());
        }
    }
    Val::default()
}

fn for1() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::default(),
        pair_mode(form_mode(), id_mode(), BasicMode::default()),
        BasicMode::default(),
    );
    let output_mode = Mode::default();
    let func = MutDispatcher::new(
        fn_for::<FreeCtx>,
        |ctx, val| fn_for(ctx, val),
        |ctx, val| fn_for(ctx, val),
    );
    named_mut_fn("for", input_mode, output_mode, false, func)
}

fn fn_for<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where
    Ctx: CtxMeta<'a>,
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
                panic!("iterate on super big int {i:?}!!!")
            };
            let iter = (0..i).map(|i| {
                let i = Int::from(i);
                Val::Int(i.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::Byte(byte) => {
            let iter = byte.as_ref().iter().map(|byte| {
                let byte = Byte::from(std::slice::from_ref(byte));
                Val::Byte(byte.into())
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
        Val::Text(t) => {
            let iter = t.chars().map(|c| {
                let str = Text::from(c.to_string());
                Val::Text(str.into())
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
    Ctx: CtxMeta<'a>,
    ValIter: Iterator<Item = Val>,
{
    let Ok(variables) = ctx.reborrow().get_variables() else {
        return Val::default();
    };
    if !variables.is_assignable(name.clone()) {
        return Val::default();
    }
    if let Val::List(body) = body {
        let body = List::from(body);
        let block_items: Option<List<BlockItem>> = body.into_iter().map(parse_block_item).collect();
        let Some(block_items) = block_items else {
            return Val::default();
        };
        for val in values {
            let Ok(variables) = ctx.reborrow().get_variables_mut() else {
                return Val::default();
            };
            variables
                .put_value(name.clone(), CtxValue::new(val))
                .expect("name should be assignable");
            let (output, ctrl_flow) = eval_block_items(ctx.reborrow(), block_items.clone());
            match ctrl_flow {
                CtrlFlow::None => {}
                CtrlFlow::Error => return Val::default(),
                CtrlFlow::Exit(exit) => match exit {
                    Exit::Continue => {}
                    Exit::Break => return output,
                },
            }
        }
    } else {
        for val in values {
            let Ok(variables) = ctx.reborrow().get_variables_mut() else {
                return Val::default();
            };
            variables
                .put_value(name.clone(), CtxValue::new(val))
                .expect("name should be assignable");
            Eval.transform(ctx.reborrow(), body.clone());
        }
    }
    Val::default()
}

fn eval_block<'a, Ctx>(ctx: Ctx, input: Val) -> (Val, CtrlFlow)
where
    Ctx: CtxMeta<'a>,
{
    let Val::List(list) = input else {
        return (Eval.transform(ctx, input), CtrlFlow::None);
    };
    let list = List::from(list);
    let block_items: Option<List<BlockItem>> = list.into_iter().map(parse_block_item).collect();
    let Some(block_items) = block_items else {
        return (Val::default(), CtrlFlow::Error);
    };
    eval_block_items(ctx, block_items)
}

fn eval_block_items<'a, Ctx>(mut ctx: Ctx, block_items: List<BlockItem>) -> (Val, CtrlFlow)
where
    Ctx: CtxMeta<'a>,
{
    let mut output = Val::default();
    for block_item in block_items {
        match block_item {
            BlockItem::Normal(val) => {
                output = Eval.transform(ctx.reborrow(), val);
            }
            BlockItem::UnitExit { exit, target, body } => {
                output = Eval.transform(ctx.reborrow(), body);
                if output.is_unit() == target {
                    return (output, CtrlFlow::from(exit));
                }
            }
            BlockItem::BoolExit {
                exit,
                target,
                condition,
                body,
            } => {
                let condition = Eval.transform(ctx.reborrow(), condition);
                let Val::Bool(condition) = condition else {
                    return (Val::default(), CtrlFlow::Error);
                };
                if condition.bool() == target {
                    let output = Eval.transform(ctx, body);
                    return (output, CtrlFlow::from(exit));
                }
                output = Val::default();
            }
        }
    }
    (output, CtrlFlow::None)
}

fn parse_block_item(val: Val) -> Option<BlockItem> {
    let Val::Call(call) = val else {
        return Some(BlockItem::Normal(val));
    };
    let Val::Symbol(s) = &call.func else {
        return Some(BlockItem::Normal(Val::Call(call)));
    };
    let Some(ParseExit {
        is_unit,
        exit,
        target,
    }) = parse_exit(s)
    else {
        return Some(BlockItem::Normal(Val::Call(call)));
    };
    let call = Call::from(call);
    let block_item = if is_unit {
        let body = call.input;
        BlockItem::UnitExit { exit, target, body }
    } else {
        let Val::Pair(pair) = call.input else {
            return None;
        };
        let pair = Pair::from(pair);
        let condition = pair.first;
        let body = pair.second;
        BlockItem::BoolExit {
            exit,
            target,
            condition,
            body,
        }
    };
    Some(block_item)
}

struct ParseExit {
    is_unit: bool,
    exit: Exit,
    target: bool,
}

impl ParseExit {
    fn new(is_unit: bool, exit: Exit, target: bool) -> Self {
        Self {
            is_unit,
            exit,
            target,
        }
    }
}

fn parse_exit(str: &str) -> Option<ParseExit> {
    let (is_unit, exit, target) = match str {
        BREAK => (false, Exit::Break, true),
        UNIT_BREAK => (true, Exit::Break, true),
        ELSE_BREAK => (false, Exit::Break, false),
        UNIT_ELSE_BREAK => (true, Exit::Break, false),
        CONTINUE => (false, Exit::Continue, true),
        UNIT_CONTINUE => (true, Exit::Continue, true),
        ELSE_CONTINUE => (false, Exit::Continue, false),
        UNIT_ELSE_CONTINUE => (true, Exit::Continue, false),
        _ => {
            return None;
        }
    };
    Some(ParseExit::new(is_unit, exit, target))
}

impl From<Exit> for CtrlFlow {
    fn from(value: Exit) -> Self {
        Self::Exit(value)
    }
}
