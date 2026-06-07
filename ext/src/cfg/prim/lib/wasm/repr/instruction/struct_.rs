use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;
use wasm_encoder::Ordering;

use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::instruction::ordering;
use crate::cfg::prim::lib::wasm::repr::key;
use crate::cfg::prim::lib::wasm::repr::pair;
use crate::cfg::prim::lib::wasm::repr::resolve_type;
use crate::cfg::prim::lib::wasm::repr::triple;
use crate::cfg::prim::lib::wasm::repr::type_::TypeKind;

pub(super) fn struct_new(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, input)?;
    sink.struct_new(struct_);
    Some(())
}

pub(super) fn struct_new_default(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, input)?;
    sink.struct_new_default(struct_);
    Some(())
}

pub(super) fn struct_new_desc(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, input)?;
    sink.struct_new_desc(struct_);
    Some(())
}

pub(super) fn struct_new_default_desc(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, input)?;
    sink.struct_new_default_desc(struct_);
    Some(())
}

pub(super) fn struct_get(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let (struct_, field) = struct_field(ctx, input)?;
    sink.struct_get(struct_, field);
    Some(())
}

pub(super) fn struct_get_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let (struct_, field) = struct_field(ctx, input)?;
    sink.struct_get_s(struct_, field);
    Some(())
}

pub(super) fn struct_get_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let (struct_, field) = struct_field(ctx, input)?;
    sink.struct_get_u(struct_, field);
    Some(())
}

pub(super) fn struct_set(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let (struct_, field) = struct_field(ctx, input)?;
    sink.struct_set(struct_, field);
    Some(())
}

pub(super) fn struct_atomic_get(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_get(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_get_s(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_get_s(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_get_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_get_u(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_set(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_set(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_add(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_add(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_sub(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_sub(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_and(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_and(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_or(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_or(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_xor(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_xor(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_xchg(ordering, struct_, field);
    Some(())
}

pub(super) fn struct_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (struct_, field, ordering) = struct_field_ordering(ctx, input)?;
    sink.struct_atomic_rmw_cmpxchg(ordering, struct_, field);
    Some(())
}

fn struct_field(ctx: &ParseCtx, input: &Val) -> Option<(u32, u32)> {
    let pair = pair(input)?;
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, &pair.left)?;
    let field = *ctx.field.get(&(struct_, key(&pair.right)?.clone()))?;
    Some((struct_, field))
}

fn struct_field_ordering(ctx: &ParseCtx, input: &Val) -> Option<(u32, u32, Ordering)> {
    let (struct_, field, ordering0) = triple(input)?;
    let struct_ = resolve_type(&ctx.type_, TypeKind::Struct, struct_)?;
    let field = *ctx.field.get(&(struct_, key(field)?.clone()))?;
    let ordering = ordering(ordering0)?;
    Some((struct_, field, ordering))
}
