use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;
use wasm_encoder::Ordering;

use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::get;
use crate::cfg::prim::lib::wasm::repr::instruction::ordering;
use crate::cfg::prim::lib::wasm::repr::map;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_name;

pub(super) fn table_init(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    let table = resolve_name(&ctx.table, &pair.left)?;
    let element = resolve_name(&ctx.element, &pair.right)?;
    sink.table_init(table, element);
    Some(())
}

pub(super) fn table_get(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let table = resolve_name(&ctx.table, input)?;
    sink.table_get(table);
    Some(())
}

pub(super) fn table_set(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let table = resolve_name(&ctx.table, input)?;
    sink.table_set(table);
    Some(())
}

pub(super) fn table_size(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let table = resolve_name(&ctx.table, input)?;
    sink.table_size(table);
    Some(())
}

pub(super) fn table_grow(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let table = resolve_name(&ctx.table, input)?;
    sink.table_grow(table);
    Some(())
}

pub(super) fn table_fill(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let table = resolve_name(&ctx.table, input)?;
    sink.table_fill(table);
    Some(())
}

pub(super) fn table_copy(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let map = map(input)?;
    let from = resolve_name(&ctx.table, get(map, "from")?)?;
    let to = resolve_name(&ctx.table, get(map, "to")?)?;
    sink.table_copy(from, to);
    Some(())
}

pub(super) fn table_atomic_get(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, ordering) = table_ordering(ctx, input)?;
    sink.table_atomic_get(ordering, table);
    Some(())
}

pub(super) fn table_atomic_set(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, ordering) = table_ordering(ctx, input)?;
    sink.table_atomic_set(ordering, table);
    Some(())
}

pub(super) fn table_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, ordering) = table_ordering(ctx, input)?;
    sink.table_atomic_rmw_xchg(ordering, table);
    Some(())
}

pub(super) fn table_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (table, ordering) = table_ordering(ctx, input)?;
    sink.table_atomic_rmw_cmpxchg(ordering, table);
    Some(())
}

pub(super) fn elem_drop(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let element = resolve_name(&ctx.element, input)?;
    sink.elem_drop(element);
    Some(())
}

fn table_ordering(ctx: &ParseCtx, input: &Val) -> Option<(u32, Ordering)> {
    let pair = pair(input)?;
    let table = resolve_name(&ctx.table, &pair.left)?;
    let ordering = ordering(&pair.right)?;
    Some((table, ordering))
}
