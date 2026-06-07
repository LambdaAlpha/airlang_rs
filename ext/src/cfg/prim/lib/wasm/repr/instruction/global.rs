use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;
use wasm_encoder::Ordering;

use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::instruction::ordering;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_name;

pub(super) fn global_get(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.global_get(resolve_name(&ctx.global, input)?);
    Some(())
}

pub(super) fn global_set(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.global_set(resolve_name(&ctx.global, input)?);
    Some(())
}

pub(super) fn global_atomic_get(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_get(ordering, global);
    Some(())
}

pub(super) fn global_atomic_set(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_set(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_add(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_add(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_sub(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_sub(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_and(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_and(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_or(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_or(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_xor(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_xor(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_xchg(ordering, global);
    Some(())
}

pub(super) fn global_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (global, ordering) = global_ordering(ctx, input)?;
    sink.global_atomic_rmw_cmpxchg(ordering, global);
    Some(())
}

fn global_ordering(ctx: &ParseCtx, input: &Val) -> Option<(u32, Ordering)> {
    let pair = pair(input)?;
    let global = resolve_name(&ctx.global, &pair.left)?;
    let ordering = ordering(&pair.right)?;
    Some((global, ordering))
}
