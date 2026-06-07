use airlang::semantics::val::Val;
use wasm_encoder::ConstExpr;
use wasm_encoder::InstructionSink;

use super::ParseCtx;
use super::array::array_new;
use super::array::array_new_default;
use super::array::array_new_fixed;
use super::global::global_get;
use super::number::f32_const;
use super::number::f64_const;
use super::number::i32_const;
use super::number::i64_const;
use super::number::v128_const;
use super::reference::ref_func;
use super::reference::ref_null;
use super::struct_::struct_new;
use super::struct_::struct_new_default;
use crate::cfg::prim::lib::wasm::repr::key;
use crate::cfg::prim::lib::wasm::repr::list;

pub(crate) fn const_expr(ctx: &ParseCtx, expr: &Val) -> Option<ConstExpr> {
    let list = list(expr)?;
    let mut instructions = Vec::new();
    let mut sink = InstructionSink::new(&mut instructions);
    for item in list {
        const_instr(ctx, &mut sink, item)?;
    }
    // ConstExpr will encode an end itself
    Some(ConstExpr::raw(instructions))
}

fn const_instr(ctx: &ParseCtx, sink: &mut InstructionSink, instruction: &Val) -> Option<()> {
    let (func, input) = match instruction {
        Val::Key(instruction) => return zero_arg_const_instr(sink, instruction),
        Val::Call(call) => {
            let func = key(&call.func)?;
            (func, &call.input)
        },
        _ => return None,
    };
    match func.as_ref() {
        "global.get" => global_get(ctx, sink, input),
        "i32.constant" => i32_const(ctx, sink, input),
        "i64.constant" => i64_const(ctx, sink, input),
        "f32.constant" => f32_const(ctx, sink, input),
        "f64.constant" => f64_const(ctx, sink, input),
        "v128.constant" => v128_const(ctx, sink, input),
        "reference.null" => ref_null(ctx, sink, input),
        "reference.function" => ref_func(ctx, sink, input),
        "struct.new" => struct_new(ctx, sink, input),
        "struct.new_default" => struct_new_default(ctx, sink, input),
        "array.new" => array_new(ctx, sink, input),
        "array.new_default" => array_new_default(ctx, sink, input),
        "array.new_fixed" => array_new_fixed(ctx, sink, input),
        _ => None,
    }
}

fn zero_arg_const_instr(sink: &mut InstructionSink, instruction: &str) -> Option<()> {
    let _ = match instruction {
        "i32.+" => sink.i32_add(),
        "i32.-" => sink.i32_sub(),
        "i32.*" => sink.i32_mul(),
        "i64.+" => sink.i64_add(),
        "i64.-" => sink.i64_sub(),
        "i64.*" => sink.i64_mul(),
        "reference.i31" => sink.ref_i31(),
        "extern.convert_any" => sink.extern_convert_any(),
        "any.convert_extern" => sink.any_convert_extern(),
        _ => return None,
    };
    Some(())
}
