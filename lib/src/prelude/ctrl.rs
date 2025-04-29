use crate::{
    Byte,
    Call,
    FuncMode,
    Int,
    List,
    Map,
    Pair,
    Symbol,
    Text,
    ctx::{
        free::FreeCtx,
        map::{
            CtxMapRef,
            CtxValue,
        },
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    func::mut_static_prim::MutDispatcher,
    mode::eval::EVAL,
    prelude::{
        Named,
        Prelude,
        named_mut_fn,
    },
    transformer::Transformer,
    val::{
        Val,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct CtrlPrelude {
    pub(crate) do1: Named<FuncVal>,
    pub(crate) if1: Named<FuncVal>,
    pub(crate) match1: Named<FuncVal>,
    pub(crate) loop1: Named<FuncVal>,
    pub(crate) for1: Named<FuncVal>,
}

impl Default for CtrlPrelude {
    fn default() -> Self {
        CtrlPrelude { do1: do1(), if1: if1(), match1: match1(), loop1: loop1(), for1: for1() }
    }
}

impl Prelude for CtrlPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.do1.put(m);
        self.if1.put(m);
        self.match1.put(m);
        self.loop1.put(m);
        self.for1.put(m);
    }
}

const BREAK: &str = "break";
const CONTINUE: &str = "continue";

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
    Exit { exit: Exit, condition: Val, body: Val },
}

fn do1() -> Named<FuncVal> {
    let id = "do";
    let f = MutDispatcher::new(
        fn_do::<FreeCtx>,
        |ctx, val| fn_do(ctx, val),
        |ctx, val| fn_do(ctx, val),
    );
    let call = FuncMode::id_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_do<'a, Ctx>(ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    eval_block(ctx, input).0
}

fn if1() -> Named<FuncVal> {
    let id = "?";
    let f = MutDispatcher::new(
        fn_if::<FreeCtx>,
        |ctx, val| fn_if(ctx, val),
        |ctx, val| fn_if(ctx, val),
    );
    let call = FuncMode::id_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_if<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let condition = EVAL.transform(ctx.reborrow(), pair.first);
    let Val::Bit(b) = condition else {
        return Val::default();
    };
    let branches = Pair::from(branches);
    let branch = if b.bool() { branches.first } else { branches.second };
    eval_block(ctx, branch).0
}

fn match1() -> Named<FuncVal> {
    let id = "match";
    let f = MutDispatcher::new(
        fn_match::<FreeCtx>,
        |ctx, val| fn_match(ctx, val),
        |ctx, val| fn_match(ctx, val),
    );
    let call = FuncMode::id_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_match<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = EVAL.transform(ctx.reborrow(), pair.first);
    let Val::Pair(pair) = pair.second else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Map(map) = pair.first else {
        return Val::default();
    };
    let eval = Map::from(map)
        .into_iter()
        .find_map(|(k, v)| {
            let k = EVAL.transform(ctx.reborrow(), k);
            if k == val { Some(v) } else { None }
        })
        .unwrap_or(pair.second);
    eval_block(ctx, eval).0
}

fn loop1() -> Named<FuncVal> {
    let id = "loop";
    let f = MutDispatcher::new(
        fn_loop::<FreeCtx>,
        |ctx, val| fn_loop(ctx, val),
        |ctx, val| fn_loop(ctx, val),
    );
    let call = FuncMode::id_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_loop<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
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
            let Val::Bit(b) = EVAL.transform(ctx.reborrow(), condition.clone()) else {
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
            let Val::Bit(b) = EVAL.transform(ctx.reborrow(), condition.clone()) else {
                return Val::default();
            };
            if !b.bool() {
                break;
            }
            EVAL.transform(ctx.reborrow(), body.clone());
        }
    }
    Val::default()
}

fn for1() -> Named<FuncVal> {
    let id = "for";
    let f = MutDispatcher::new(
        fn_for::<FreeCtx>,
        |ctx, val| fn_for(ctx, val),
        |ctx, val| fn_for(ctx, val),
    );
    let call = FuncMode::id_mode();
    let mode = FuncMode { call };
    named_mut_fn(id, f, mode)
}

fn fn_for<'a, Ctx>(mut ctx: Ctx, input: Val) -> Val
where Ctx: CtxMeta<'a> {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let iterable = EVAL.transform(ctx.reborrow(), pair.first);
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
            let Some(i) = i.to_u128() else { panic!("iterate on super big int {i:?}!!!") };
            let iter = (0 .. i).map(|i| {
                let i = Int::from(i);
                Val::Int(i.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::Byte(byte) => {
            let iter = byte.iter().map(|byte| {
                let byte = Byte::from(vec![*byte]);
                Val::Byte(byte.into())
            });
            for_iter(ctx, body, name, iter)
        }
        Val::Symbol(s) => {
            let iter = s.char_indices().map(|(start, c)| {
                let symbol = Symbol::from_str(&s[start .. start + c.len_utf8()]);
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
    ValIter: Iterator<Item = Val>, {
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
            EVAL.transform(ctx.reborrow(), body.clone());
        }
    }
    Val::default()
}

fn eval_block<'a, Ctx>(ctx: Ctx, input: Val) -> (Val, CtrlFlow)
where Ctx: CtxMeta<'a> {
    let Val::List(list) = input else {
        return (EVAL.transform(ctx, input), CtrlFlow::None);
    };
    let list = List::from(list);
    let block_items: Option<List<BlockItem>> = list.into_iter().map(parse_block_item).collect();
    let Some(block_items) = block_items else {
        return (Val::default(), CtrlFlow::Error);
    };
    eval_block_items(ctx, block_items)
}

fn eval_block_items<'a, Ctx>(mut ctx: Ctx, block_items: List<BlockItem>) -> (Val, CtrlFlow)
where Ctx: CtxMeta<'a> {
    let mut output = Val::default();
    for block_item in block_items {
        match block_item {
            BlockItem::Normal(val) => {
                output = EVAL.transform(ctx.reborrow(), val);
            }
            BlockItem::Exit { exit, condition, body } => {
                let condition = EVAL.transform(ctx.reborrow(), condition);
                let Val::Bit(condition) = condition else {
                    return (Val::default(), CtrlFlow::Error);
                };
                if condition.bool() {
                    let output = EVAL.transform(ctx, body);
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
    let Some(exit) = parse_exit(s) else {
        return Some(BlockItem::Normal(Val::Call(call)));
    };
    let call = Call::from(call);
    let Val::Pair(pair) = call.input else {
        return None;
    };
    let pair = Pair::from(pair);
    let condition = pair.first;
    let body = pair.second;
    let block_item = BlockItem::Exit { exit, condition, body };
    Some(block_item)
}

fn parse_exit(str: &str) -> Option<Exit> {
    let exit = match str {
        BREAK => Exit::Break,
        CONTINUE => Exit::Continue,
        _ => return None,
    };
    Some(exit)
}

impl From<Exit> for CtrlFlow {
    fn from(value: Exit) -> Self {
        Self::Exit(value)
    }
}
