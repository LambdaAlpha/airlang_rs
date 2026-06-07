use airlang::semantics::val::Val;
use num_traits::ToPrimitive;
use wasm_encoder::Ieee32;
use wasm_encoder::Ieee64;
use wasm_encoder::InstructionSink;
use wasm_encoder::Lane;
use wasm_encoder::MemArg;

use super::memory::mem_arg;
use crate::cfg::prim::lib::wasm::repr::ParseCtx;
use crate::cfg::prim::lib::wasm::repr::decimal;
use crate::cfg::prim::lib::wasm::repr::get;
use crate::cfg::prim::lib::wasm::repr::int;
use crate::cfg::prim::lib::wasm::repr::list;
use crate::cfg::prim::lib::wasm::repr::map;

pub(super) fn i32_const(_ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_const(int(input)?.to_i32()?);
    Some(())
}

pub(super) fn i32_load(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_load(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_load8_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_load8_s(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_load8_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_load8_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_load16_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_load16_s(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_load16_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_load16_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_store(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_store(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_store8(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_store8(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_store16(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i32_store16(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_load(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_load(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_load8_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_load8_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_load16_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_load16_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_store(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_store(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_store8(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_store8(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_store16(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_store16(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_add(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_add(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_sub(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_sub(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_and(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_and(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_or(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_or(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_xor(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_xor(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_xchg(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw_cmpxchg(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_add_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_add_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_sub_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_sub_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_and_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_and_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_or_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_or_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_xor_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_xor_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_xchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_xchg_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw8_cmpxchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw8_cmpxchg_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_add_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_add_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_sub_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_sub_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_and_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_and_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_or_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_or_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_xor_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_xor_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_xchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_xchg_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i32_atomic_rmw16_cmpxchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32_atomic_rmw16_cmpxchg_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_const(_ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_const(int(input)?.to_i64()?);
    Some(())
}

pub(super) fn i64_load(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_load8_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load8_s(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_load8_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load8_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_load16_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load16_s(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_load16_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load16_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_load32_s(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load32_s(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_load32_u(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_load32_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_store(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_store(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_store8(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_store8(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_store16(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_store16(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_store32(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.i64_store32(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_load(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_load(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_load8_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_load8_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_load16_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_load16_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_load32_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_load32_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_store(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_store(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_store8(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_store8(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_store16(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_store16(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_store32(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_store32(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_add(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_add(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_sub(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_sub(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_and(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_and(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_or(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_or(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_xor(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_xor(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_xchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_xchg(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw_cmpxchg(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw_cmpxchg(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_add_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_add_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_sub_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_sub_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_and_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_and_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_or_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_or_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_xor_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_xor_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_xchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_xchg_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw8_cmpxchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw8_cmpxchg_u(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_add_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_add_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_sub_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_sub_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_and_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_and_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_or_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_or_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_xor_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_xor_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_xchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_xchg_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw16_cmpxchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw16_cmpxchg_u(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_add_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_add_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_sub_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_sub_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_and_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_and_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_or_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_or_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_xor_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_xor_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_xchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_xchg_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn i64_atomic_rmw32_cmpxchg_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64_atomic_rmw32_cmpxchg_u(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn f32_const(_ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f32_const(Ieee32::from(decimal(input)?.to_f32()?));
    Some(())
}

pub(super) fn f32_load(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f32_load(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn f32_store(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f32_store(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn f64_const(_ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f64_const(Ieee64::from(decimal(input)?.to_f64()?));
    Some(())
}

pub(super) fn f64_load(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f64_load(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn f64_store(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.f64_store(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_const(_ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.v128_const(int(input)?.to_i128()?);
    Some(())
}

pub(super) fn v128_load(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.v128_load(memory_arg(ctx, input, 16)?);
    Some(())
}

pub(super) fn v128_load8x8_s(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load8x8_s(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load8x8_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load8x8_u(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load16x4_s(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load16x4_s(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load16x4_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load16x4_u(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load32x2_s(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load32x2_s(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load32x2_u(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load32x2_u(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load8_splat(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load8_splat(memory_arg(ctx, input, 1)?);
    Some(())
}

pub(super) fn v128_load8_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 1)?;
    sink.v128_load8_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_load16_splat(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load16_splat(memory_arg(ctx, input, 2)?);
    Some(())
}

pub(super) fn v128_load16_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 2)?;
    sink.v128_load16_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_load32_splat(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load32_splat(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn v128_load32_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 4)?;
    sink.v128_load32_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_load32_zero(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load32_zero(memory_arg(ctx, input, 4)?);
    Some(())
}

pub(super) fn v128_load64_splat(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load64_splat(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_load64_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 8)?;
    sink.v128_load64_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_load64_zero(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.v128_load64_zero(memory_arg(ctx, input, 8)?);
    Some(())
}

pub(super) fn v128_store(ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val) -> Option<()> {
    sink.v128_store(memory_arg(ctx, input, 16)?);
    Some(())
}

pub(super) fn v128_store8_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 1)?;
    sink.v128_store8_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_store16_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 2)?;
    sink.v128_store16_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_store32_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 4)?;
    sink.v128_store32_lane(memory_arg, lane);
    Some(())
}

pub(super) fn v128_store64_lane(
    ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let (memory_arg, lane) = memory_lane(ctx, input, 8)?;
    sink.v128_store64_lane(memory_arg, lane);
    Some(())
}

pub(super) fn i8x16_extract_lane_s(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i8x16_extract_lane_s(lane(input)?);
    Some(())
}

pub(super) fn i8x16_extract_lane_u(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i8x16_extract_lane_u(lane(input)?);
    Some(())
}

pub(super) fn i8x16_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i8x16_replace_lane(lane(input)?);
    Some(())
}

pub(super) fn i8x16_shuffle(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    let list = list(input)?;
    if list.len() != 16 {
        return None;
    }
    let lanes: Option<Vec<Lane>> = list.iter().map(lane).collect();
    let lanes: [Lane; 16] = lanes?.try_into().expect("length must be 16");
    sink.i8x16_shuffle(lanes);
    Some(())
}

pub(super) fn i16x8_extract_lane_s(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i16x8_extract_lane_s(lane(input)?);
    Some(())
}

pub(super) fn i16x8_extract_lane_u(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i16x8_extract_lane_u(lane(input)?);
    Some(())
}

pub(super) fn i16x8_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i16x8_replace_lane(lane(input)?);
    Some(())
}

pub(super) fn i32x4_extract_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32x4_extract_lane(lane(input)?);
    Some(())
}

pub(super) fn i32x4_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i32x4_replace_lane(lane(input)?);
    Some(())
}

pub(super) fn i64x2_extract_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64x2_extract_lane(lane(input)?);
    Some(())
}

pub(super) fn i64x2_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.i64x2_replace_lane(lane(input)?);
    Some(())
}

pub(super) fn f32x4_extract_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.f32x4_extract_lane(lane(input)?);
    Some(())
}

pub(super) fn f32x4_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.f32x4_replace_lane(lane(input)?);
    Some(())
}

pub(super) fn f64x2_extract_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.f64x2_extract_lane(lane(input)?);
    Some(())
}

pub(super) fn f64x2_replace_lane(
    _ctx: &ParseCtx, sink: &mut InstructionSink, input: &Val,
) -> Option<()> {
    sink.f64x2_replace_lane(lane(input)?);
    Some(())
}

fn memory_lane(ctx: &ParseCtx, input: &Val, default_align: u32) -> Option<(MemArg, Lane)> {
    let map = map(input)?;
    let mem_arg = mem_arg(ctx, map, default_align)?;
    let lane = lane(get(map, "lane")?)?;
    Some((mem_arg, lane))
}

fn lane(input: &Val) -> Option<Lane> {
    int(input)?.to_u8()
}

fn memory_arg(ctx: &ParseCtx, input: &Val, default_align: u32) -> Option<MemArg> {
    let map = map(input)?;
    mem_arg(ctx, map, default_align)
}
