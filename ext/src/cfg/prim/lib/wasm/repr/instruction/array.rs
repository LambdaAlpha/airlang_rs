use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;
use wasm_encoder::Ordering;

use super::ParseCtx;
use super::ordering;
use super::resolve_name;
use crate::cfg::prim::lib::wasm::repr::get;
use crate::cfg::prim::lib::wasm::repr::map;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_type;
use crate::cfg::prim::lib::wasm::repr::type_::TypeKind;
use crate::cfg::prim::lib::wasm::repr::u32;

pub(super) fn array_new(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_new(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_new_default(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.array_new_default(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_new_fixed(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let count = u32(&pair.right)?;
    sink.array_new_fixed(array_type, count);
    Some(())
}

pub(super) fn array_new_data(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let data = resolve_name(&ctx.data, &pair.right)?;
    sink.array_new_data(array_type, data);
    Some(())
}

pub(super) fn array_init_data(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let data = resolve_name(&ctx.data, &pair.right)?;
    sink.array_init_data(array_type, data);
    Some(())
}

pub(super) fn array_new_elem(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let element = resolve_name(&ctx.element, &pair.right)?;
    sink.array_new_elem(array_type, element);
    Some(())
}

pub(super) fn array_init_elem(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let element = resolve_name(&ctx.element, &pair.right)?;
    sink.array_init_elem(array_type, element);
    Some(())
}

pub(super) fn array_get(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_get(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_get_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_get_s(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_get_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_get_u(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_set(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_set(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_fill(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.array_fill(resolve_type(&ctx.type_, TypeKind::Array, input)?);
    Some(())
}

pub(super) fn array_copy(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let map = map(input)?;
    let from = resolve_type(&ctx.type_, TypeKind::Array, get(map, "from")?)?;
    let to = resolve_type(&ctx.type_, TypeKind::Array, get(map, "to")?)?;
    sink.array_copy(from, to);
    Some(())
}

pub(super) fn array_atomic_get(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_get(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_get_s(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_get_s(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_get_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_get_u(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_set(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_set(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_add(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_add(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_sub(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_sub(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_and(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_and(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_or(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_or(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_xor(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_xor(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_xchg(ordering, array_type);
    Some(())
}

pub(super) fn array_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (array_type, ordering) = array_ordering(ctx, input)?;
    sink.array_atomic_rmw_cmpxchg(ordering, array_type);
    Some(())
}

fn array_ordering(ctx: &ParseCtx, input: &Val) -> Option<(u32, Ordering)> {
    let pair = pair(input)?;
    let array_type = resolve_type(&ctx.type_, TypeKind::Array, &pair.left)?;
    let ordering = ordering(&pair.right)?;
    Some((array_type, ordering))
}
