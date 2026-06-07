use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::InstructionSink;
use wasm_encoder::MemArg;

use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::get;
use crate::cfg::prim::lib::wasm::repr::map;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_name;
use crate::cfg::prim::lib::wasm::repr::u32;
use crate::cfg::prim::lib::wasm::repr::u64;

pub(super) fn memory_init(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    let memory = resolve_name(&ctx.memory, &pair.left)?;
    let data = resolve_name(&ctx.data, &pair.right)?;
    sink.memory_init(memory, data);
    Some(())
}

pub(super) fn memory_size(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let memory = resolve_name(&ctx.memory, input)?;
    sink.memory_size(memory);
    Some(())
}

pub(super) fn memory_grow(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let memory = resolve_name(&ctx.memory, input)?;
    sink.memory_grow(memory);
    Some(())
}

pub(super) fn memory_fill(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let memory = resolve_name(&ctx.memory, input)?;
    sink.memory_fill(memory);
    Some(())
}

pub(super) fn memory_copy(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let map = map(input)?;
    let from = resolve_name(&ctx.memory, get(map, "from")?)?;
    let to = resolve_name(&ctx.memory, get(map, "to")?)?;
    sink.memory_copy(from, to);
    Some(())
}

pub(super) fn memory_discard(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let memory = resolve_name(&ctx.memory, input)?;
    sink.memory_discard(memory);
    Some(())
}

pub(super) fn memory_atomic_notify(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.memory_atomic_notify(mem_arg(ctx, map(input)?, 4)?);
    Some(())
}

pub(super) fn memory_atomic_wait32(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.memory_atomic_wait32(mem_arg(ctx, map(input)?, 4)?);
    Some(())
}

pub(super) fn memory_atomic_wait64(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.memory_atomic_wait64(mem_arg(ctx, map(input)?, 8)?);
    Some(())
}

pub(super) fn data_drop(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let data = resolve_name(&ctx.data, input)?;
    sink.data_drop(data);
    Some(())
}

pub(super) fn mem_arg(ctx: &ParseCtx, map: &Map<Key, Val>, default_align: u32) -> Option<MemArg> {
    let memory = get(map, "memory");
    let memory_index =
        if let Some(memory) = memory { resolve_name(&ctx.memory, memory)? } else { 0 };
    let align = if let Some(align) = get(map, "align") { u32(align)? } else { default_align };
    if !align.is_power_of_two() {
        return None;
    }
    let align = align.trailing_zeros();
    let offset = if let Some(offset) = get(map, "offset") { u64(offset)? } else { 0 };
    Some(MemArg { offset, align, memory_index })
}
