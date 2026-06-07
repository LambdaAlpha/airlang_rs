use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;

use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::resolve_name;
use crate::cfg::prim::lib::wasm::repr::type_::heap_type;

pub(super) fn ref_null(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.ref_null(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_func(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.ref_func(resolve_name(&ctx.function, input)?);
    Some(())
}

pub(super) fn ref_get_desc(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    let descriptor_type = resolve_name(&ctx.type_, input)?.1;
    sink.ref_get_desc(descriptor_type);
    Some(())
}

pub(super) fn ref_test_nullable(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_test_nullable(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_test_non_null(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_test_non_null(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_cast_nullable(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_cast_nullable(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_cast_non_null(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_cast_non_null(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_cast_desc_eq_nullable(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_cast_desc_eq_nullable(heap_type(ctx, input)?);
    Some(())
}

pub(super) fn ref_cast_desc_eq_non_null(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.ref_cast_desc_eq_non_null(heap_type(ctx, input)?);
    Some(())
}
