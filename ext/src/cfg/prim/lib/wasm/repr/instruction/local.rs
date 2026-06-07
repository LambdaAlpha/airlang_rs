use std::collections::HashMap;
use std::collections::hash_map::Entry;

use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::BlockType;
use wasm_encoder::Catch;
use wasm_encoder::Handle;
use wasm_encoder::InstructionSink;
use wasm_encoder::RefType;
use wasm_encoder::ValType;

use super::ParseCtx;
use super::instruction;
use crate::cfg::prim::lib::wasm::repr::Sections;
use crate::cfg::prim::lib::wasm::repr::contains;
use crate::cfg::prim::lib::wasm::repr::get;
use crate::cfg::prim::lib::wasm::repr::key;
use crate::cfg::prim::lib::wasm::repr::list;
use crate::cfg::prim::lib::wasm::repr::map;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_name;
use crate::cfg::prim::lib::wasm::repr::resolve_type;
use crate::cfg::prim::lib::wasm::repr::triple;
use crate::cfg::prim::lib::wasm::repr::type_::TypeKind;
use crate::cfg::prim::lib::wasm::repr::type_::parse_val_type;
use crate::cfg::prim::lib::wasm::repr::type_::ref_type;

pub(crate) struct LocalCtx {
    local: HashMap<Key, u32>,
    label: HashMap<Key, u32>,
    depth: u32,
}

impl LocalCtx {
    pub(crate) fn new(local: HashMap<Key, u32>) -> Self {
        Self { local, label: HashMap::default(), depth: 0 }
    }

    pub(crate) fn locals(&self) -> &HashMap<Key, u32> {
        &self.local
    }
}

pub(super) fn local_get(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.local_get(resolve_name(local.locals(), input)?);
    Some(())
}

pub(super) fn local_set(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.local_set(resolve_name(local.locals(), input)?);
    Some(())
}

pub(super) fn local_tee(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.local_tee(resolve_name(local.locals(), input)?);
    Some(())
}

// todo design typed_select_multi
pub(super) fn select(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.typed_select(parse_val_type(ctx, input)?);
    Some(())
}

pub(super) fn block(
    ctx: &ParseCtx, local: &mut LocalCtx, sections: &mut Sections, sink: &mut InstructionSink,
    input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let map = map(&pair.left)?;
    let list = list(&pair.right)?;

    enter_block(local, map)?;
    sink.block(block_type(ctx, map)?);
    for item in list {
        instruction(ctx, local, sections, sink, item)?;
    }
    sink.end();
    leave_block(local, map)?;
    Some(())
}

pub(super) fn loop_(
    ctx: &ParseCtx, local: &mut LocalCtx, sections: &mut Sections, sink: &mut InstructionSink,
    input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let map = map(&pair.left)?;
    let list = list(&pair.right)?;

    enter_block(local, map)?;
    sink.loop_(block_type(ctx, map)?);
    for item in list {
        instruction(ctx, local, sections, sink, item)?;
    }
    sink.end();
    leave_block(local, map)?;
    Some(())
}

pub(super) fn if_(
    ctx: &ParseCtx, local: &mut LocalCtx, sections: &mut Sections, sink: &mut InstructionSink,
    input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let map = map(&pair.left)?;
    let (then, else_) = match &pair.right {
        Val::Pair(pair) => {
            let then_body = list(&pair.left)?;
            let else_body = list(&pair.right)?;
            (then_body, Some(else_body))
        },
        Val::List(list) => (&**list, None),
        _ => return None,
    };

    enter_block(local, map)?;
    sink.if_(block_type(ctx, map)?);
    for item in then {
        instruction(ctx, local, sections, sink, item)?;
    }
    if let Some(else_) = else_ {
        sink.else_();
        for item in else_ {
            instruction(ctx, local, sections, sink, item)?;
        }
    }
    sink.end();
    leave_block(local, map)?;
    Some(())
}

pub(super) fn try_table(
    ctx: &ParseCtx, local: &mut LocalCtx, sections: &mut Sections, sink: &mut InstructionSink,
    input: &Val,
) -> Option<()> {
    let (map0, body, catches) = triple(input)?;
    let map = map(map0)?;
    let body = list(body)?;
    let catches = list(catches)?;

    enter_block(local, map)?;
    let catches: Option<Vec<Catch>> = catches.iter().map(|v| catch(ctx, local, v)).collect();
    sink.try_table(block_type(ctx, map)?, catches?);
    for item in body {
        instruction(ctx, local, sections, sink, item)?;
    }
    sink.end();
    leave_block(local, map)?;
    Some(())
}

fn catch(ctx: &ParseCtx, local: &LocalCtx, input: &Val) -> Option<Catch> {
    let map = map(input)?;
    let reference = contains(map, "reference")?;
    let label = resolve_label(local, get(map, "label")?)?;
    let catch = if let Some(tag) = get(map, "tag") {
        let tag = resolve_name(&ctx.tag, tag)?;
        if reference { Catch::OneRef { tag, label } } else { Catch::One { tag, label } }
    } else {
        if reference { Catch::AllRef { label } } else { Catch::All { label } }
    };
    Some(catch)
}

pub(super) fn br(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.br(resolve_label(local, input)?);
    Some(())
}

pub(super) fn br_if(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.br_if(resolve_label(local, input)?);
    Some(())
}

pub(super) fn br_table(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let items = list(input)?;
    if items.is_empty() {
        return None;
    }
    let labels: Option<Vec<u32>> = items.iter().map(|v| resolve_label(local, v)).collect();
    let mut labels = labels?;
    let default = labels.pop().expect("branch_table requires at least one label");
    sink.br_table(labels, default);
    Some(())
}

pub(super) fn br_on_null(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.br_on_null(resolve_label(local, input)?);
    Some(())
}

pub(super) fn br_on_non_null(
    _ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.br_on_non_null(resolve_label(local, input)?);
    Some(())
}

pub(super) fn br_on_cast(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (from, to, label) = cast_arg(ctx, local, input)?;
    sink.br_on_cast(label, from, to);
    Some(())
}

pub(super) fn br_on_cast_fail(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (from, to, label) = cast_arg(ctx, local, input)?;
    sink.br_on_cast_fail(label, from, to);
    Some(())
}

pub(super) fn br_on_cast_desc_eq(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (from, to, label) = cast_arg(ctx, local, input)?;
    sink.br_on_cast_desc_eq(label, from, to);
    Some(())
}

pub(super) fn br_on_cast_desc_eq_fail(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (from, to, label) = cast_arg(ctx, local, input)?;
    sink.br_on_cast_desc_eq_fail(label, from, to);
    Some(())
}

fn cast_arg(ctx: &ParseCtx, local: &LocalCtx, input: &Val) -> Option<(RefType, RefType, u32)> {
    let map = map(input)?;
    let label = resolve_label(local, get(map, "label")?)?;
    let from = ref_type(ctx, get(map, "from")?)?;
    let to = ref_type(ctx, get(map, "to")?)?;
    Some((from, to, label))
}

pub(super) fn call(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.call(resolve_name(&ctx.function, input)?);
    Some(())
}

pub(super) fn return_call(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.return_call(resolve_name(&ctx.function, input)?);
    Some(())
}

pub(super) fn call_ref(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.call_ref(resolve_type(&ctx.type_, TypeKind::Func, input)?);
    Some(())
}

pub(super) fn return_call_ref(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.return_call_ref(resolve_type(&ctx.type_, TypeKind::Func, input)?);
    Some(())
}

pub(super) fn call_indirect(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, func_type) = call_indirect_arg(ctx, input)?;
    sink.call_indirect(table, func_type);
    Some(())
}

pub(super) fn return_call_indirect(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, func_type) = call_indirect_arg(ctx, input)?;
    sink.return_call_indirect(table, func_type);
    Some(())
}

fn call_indirect_arg(ctx: &ParseCtx, input: &Val) -> Option<(u32, u32)> {
    let map = map(input)?;
    let table = resolve_name(&ctx.table, get(map, "table")?)?;
    let func_type = resolve_type(&ctx.type_, TypeKind::Func, get(map, "type")?)?;
    Some((table, func_type))
}

pub(super) fn throw(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.throw(resolve_name(&ctx.tag, input)?);
    Some(())
}

pub(super) fn cont_new(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.cont_new(resolve_type(&ctx.type_, TypeKind::Cont, input)?);
    Some(())
}

pub(super) fn cont_bind(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let continuation1 = resolve_type(&ctx.type_, TypeKind::Cont, &pair.left)?;
    let continuation2 = resolve_type(&ctx.type_, TypeKind::Cont, &pair.right)?;
    sink.cont_bind(continuation1, continuation2);
    Some(())
}

pub(super) fn suspend(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.suspend(resolve_name(&ctx.tag, input)?);
    Some(())
}

pub(super) fn resume(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let map = map(input)?;
    let cont_type = resolve_type(&ctx.type_, TypeKind::Cont, get(map, "type")?)?;
    let handles = handles(ctx, local, get(map, "handlers")?)?;
    sink.resume(cont_type, handles);
    Some(())
}

pub(super) fn resume_throw(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let map = map(input)?;
    let cont_type = resolve_type(&ctx.type_, TypeKind::Cont, get(map, "type")?)?;
    let tag = resolve_name(&ctx.tag, get(map, "tag")?)?;
    let handles = handles(ctx, local, get(map, "handlers")?)?;
    sink.resume_throw(cont_type, tag, handles);
    Some(())
}

pub(super) fn resume_throw_ref(
    ctx: &ParseCtx, local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let map = map(input)?;
    let cont_type = resolve_type(&ctx.type_, TypeKind::Cont, get(map, "type")?)?;
    let handles = handles(ctx, local, get(map, "handlers")?)?;
    sink.resume_throw_ref(cont_type, handles);
    Some(())
}

fn handles(ctx: &ParseCtx, local: &LocalCtx, input: &Val) -> Option<Vec<Handle>> {
    let list = list(input)?;
    list.iter().map(|v| handle(ctx, local, v)).collect()
}

fn handle(ctx: &ParseCtx, local: &LocalCtx, input: &Val) -> Option<Handle> {
    let map = map(input)?;
    let tag = resolve_name(&ctx.tag, get(map, "tag")?)?;
    let label = resolve_label(local, get(map, "label")?);
    let handle = if let Some(label) = label {
        Handle::OnLabel { tag, label }
    } else {
        Handle::OnSwitch { tag }
    };
    Some(handle)
}

pub(super) fn switch(
    ctx: &ParseCtx, _local: &LocalCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let map = map(input)?;
    let cont_type = resolve_type(&ctx.type_, TypeKind::Cont, get(map, "type")?)?;
    let tag = resolve_name(&ctx.tag, get(map, "tag")?)?;
    sink.switch(cont_type, tag);
    Some(())
}

fn block_type(ctx: &ParseCtx, map: &Map<Key, Val>) -> Option<BlockType> {
    let Some(type_) = get(map, "type") else {
        return Some(BlockType::Empty);
    };
    match type_ {
        Val::Call(_) => {
            let ref_type = ref_type(ctx, type_)?;
            Some(BlockType::Result(ValType::Ref(ref_type)))
        },
        Val::Key(key) => {
            let block_type = match key.as_ref() {
                "i32" => BlockType::Result(ValType::I32),
                "i64" => BlockType::Result(ValType::I64),
                "f32" => BlockType::Result(ValType::F32),
                "f64" => BlockType::Result(ValType::F64),
                "v128" => BlockType::Result(ValType::V128),
                _ => {
                    let type_ = resolve_type(&ctx.type_, TypeKind::Func, type_)?;
                    BlockType::FunctionType(type_)
                },
            };
            Some(block_type)
        },
        _ => None,
    }
}

fn enter_block(ctx: &mut LocalCtx, map: &Map<Key, Val>) -> Option<()> {
    ctx.depth += 1;
    let Some(label) = get(map, "label") else { return Some(()) };
    match ctx.label.entry(key(label)?.clone()) {
        Entry::Occupied(_) => None,
        Entry::Vacant(entry) => {
            entry.insert(ctx.depth);
            Some(())
        },
    }
}

fn leave_block(ctx: &mut LocalCtx, map: &Map<Key, Val>) -> Option<()> {
    ctx.depth -= 1;
    let Some(label) = get(map, "label") else { return Some(()) };
    match ctx.label.entry(key(label)?.clone()) {
        Entry::Occupied(entry) => {
            entry.remove();
            Some(())
        },
        Entry::Vacant(_) => None,
    }
}

pub(super) fn resolve_label(ctx: &LocalCtx, label: &Val) -> Option<u32> {
    let depth = ctx.label.get(key(label)?)?;
    Some(ctx.depth - depth)
}
