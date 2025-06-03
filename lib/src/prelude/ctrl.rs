use crate::Byte;
use crate::Call;
use crate::CodeMode;
use crate::Contract;
use crate::FuncMode;
use crate::Int;
use crate::List;
use crate::Map;
use crate::MutStaticFn;
use crate::MutStaticPrimFuncVal;
use crate::Pair;
use crate::PrimMode;
use crate::Symbol;
use crate::SymbolMode;
use crate::Text;
use crate::ctx::pattern::PatternCtx;
use crate::ctx::pattern::assign_pattern;
use crate::ctx::pattern::match_pattern;
use crate::ctx::pattern::parse_pattern;
use crate::func::func_mode::DEFAULT_MODE;
use crate::prelude::DynFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::mut_impl;
use crate::val::Val;

#[derive(Clone)]
pub(crate) struct CtrlPrelude {
    pub(crate) do1: MutStaticPrimFuncVal,
    pub(crate) if1: MutStaticPrimFuncVal,
    pub(crate) match1: MutStaticPrimFuncVal,
    pub(crate) loop1: MutStaticPrimFuncVal,
    pub(crate) for1: MutStaticPrimFuncVal,
}

impl Default for CtrlPrelude {
    fn default() -> Self {
        CtrlPrelude { do1: do1(), if1: if1(), match1: match1(), loop1: loop1(), for1: for1() }
    }
}

impl Prelude for CtrlPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.do1.put(ctx);
        self.if1.put(ctx);
        self.match1.put(ctx);
        self.loop1.put(ctx);
        self.for1.put(ctx);
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

fn do1() -> MutStaticPrimFuncVal {
    DynFn {
        id: "do",
        f: mut_impl(fn_do),
        mode: FuncMode { forward: FuncMode::id_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_do(ctx: &mut Val, input: Val) -> Val {
    eval_block(ctx, input).0
}

fn if1() -> MutStaticPrimFuncVal {
    DynFn {
        id: "?",
        f: mut_impl(fn_if),
        mode: FuncMode { forward: FuncMode::id_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_if(ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Pair(branches) = pair.second else {
        return Val::default();
    };
    let condition = DEFAULT_MODE.mut_static_call(ctx, pair.first);
    let Val::Bit(b) = condition else {
        return Val::default();
    };
    let branches = Pair::from(branches);
    let branch = if b.bool() { branches.first } else { branches.second };
    eval_block(ctx, branch).0
}

fn match1() -> MutStaticPrimFuncVal {
    DynFn {
        id: "match",
        f: mut_impl(fn_match),
        mode: FuncMode { forward: FuncMode::id_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_match(ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = DEFAULT_MODE.mut_static_call(ctx, pair.first);
    let Val::List(list) = pair.second else {
        return Val::default();
    };
    let mode = PrimMode::symbol_call(SymbolMode::Literal, CodeMode::Form);
    for item in List::from(list) {
        let Val::Pair(pair) = item else {
            return Val::default();
        };
        let pair = Pair::from(pair);
        let pattern = mode.mut_static_call(ctx, pair.first);
        let Some(pattern) = parse_pattern(PatternCtx::default(), pattern) else {
            return Val::default();
        };
        if match_pattern(&pattern, &val) {
            if let Val::Ctx(ctx_val) = ctx {
                assign_pattern(ctx_val, pattern, val);
            }
            return eval_block(ctx, pair.second).0;
        }
    }
    Val::default()
}

fn loop1() -> MutStaticPrimFuncVal {
    DynFn {
        id: "loop",
        f: mut_impl(fn_loop),
        mode: FuncMode { forward: FuncMode::id_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_loop(ctx: &mut Val, input: Val) -> Val {
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
            let Val::Bit(b) = DEFAULT_MODE.mut_static_call(ctx, condition.clone()) else {
                return Val::default();
            };
            if !b.bool() {
                break;
            }
            let (output, ctrl_flow) = eval_block_items(ctx, block_items.clone());
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
            let Val::Bit(b) = DEFAULT_MODE.mut_static_call(ctx, condition.clone()) else {
                return Val::default();
            };
            if !b.bool() {
                break;
            }
            DEFAULT_MODE.mut_static_call(ctx, body.clone());
        }
    }
    Val::default()
}

fn for1() -> MutStaticPrimFuncVal {
    DynFn {
        id: "for",
        f: mut_impl(fn_for),
        mode: FuncMode { forward: FuncMode::id_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_for(ctx: &mut Val, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let iterable = DEFAULT_MODE.mut_static_call(ctx, pair.first);
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

fn for_iter<ValIter>(ctx: &mut Val, body: Val, name: Symbol, values: ValIter) -> Val
where ValIter: Iterator<Item = Val> {
    if let Val::List(body) = body {
        let body = List::from(body);
        let block_items: Option<List<BlockItem>> = body.into_iter().map(parse_block_item).collect();
        let Some(block_items) = block_items else {
            return Val::default();
        };
        for val in values {
            if let Val::Ctx(ctx_val) = ctx {
                let _ = ctx_val.variables_mut().put(name.clone(), val, Contract::None);
            }
            let (output, ctrl_flow) = eval_block_items(ctx, block_items.clone());
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
            if let Val::Ctx(ctx_val) = ctx {
                let _ = ctx_val.variables_mut().put(name.clone(), val, Contract::None);
            }
            DEFAULT_MODE.mut_static_call(ctx, body.clone());
        }
    }
    Val::default()
}

fn eval_block(ctx: &mut Val, input: Val) -> (Val, CtrlFlow) {
    let Val::List(list) = input else {
        return (DEFAULT_MODE.mut_static_call(ctx, input), CtrlFlow::None);
    };
    let list = List::from(list);
    let block_items: Option<List<BlockItem>> = list.into_iter().map(parse_block_item).collect();
    let Some(block_items) = block_items else {
        return (Val::default(), CtrlFlow::Error);
    };
    eval_block_items(ctx, block_items)
}

fn eval_block_items(ctx: &mut Val, block_items: List<BlockItem>) -> (Val, CtrlFlow) {
    let mut output = Val::default();
    for block_item in block_items {
        match block_item {
            BlockItem::Normal(val) => {
                output = DEFAULT_MODE.mut_static_call(ctx, val);
            }
            BlockItem::Exit { exit, condition, body } => {
                let condition = DEFAULT_MODE.mut_static_call(ctx, condition);
                let Val::Bit(condition) = condition else {
                    return (Val::default(), CtrlFlow::Error);
                };
                if condition.bool() {
                    let output = DEFAULT_MODE.mut_static_call(ctx, body);
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
