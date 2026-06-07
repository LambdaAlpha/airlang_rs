pub(super) use self::constant::const_expr;
pub(super) use self::local::LocalCtx;

_____!();

use airlang::_____;
use airlang::semantics::val::Val;
use wasm_encoder::InstructionSink;
use wasm_encoder::Ordering;

use self::array::array_atomic_get;
use self::array::array_atomic_get_s;
use self::array::array_atomic_get_u;
use self::array::array_atomic_rmw_add;
use self::array::array_atomic_rmw_and;
use self::array::array_atomic_rmw_cmpxchg;
use self::array::array_atomic_rmw_or;
use self::array::array_atomic_rmw_sub;
use self::array::array_atomic_rmw_xchg;
use self::array::array_atomic_rmw_xor;
use self::array::array_atomic_set;
use self::array::array_copy;
use self::array::array_fill;
use self::array::array_get;
use self::array::array_get_s;
use self::array::array_get_u;
use self::array::array_init_data;
use self::array::array_init_elem;
use self::array::array_new;
use self::array::array_new_data;
use self::array::array_new_default;
use self::array::array_new_elem;
use self::array::array_new_fixed;
use self::array::array_set;
use self::global::global_atomic_get;
use self::global::global_atomic_rmw_add;
use self::global::global_atomic_rmw_and;
use self::global::global_atomic_rmw_cmpxchg;
use self::global::global_atomic_rmw_or;
use self::global::global_atomic_rmw_sub;
use self::global::global_atomic_rmw_xchg;
use self::global::global_atomic_rmw_xor;
use self::global::global_atomic_set;
use self::global::global_get;
use self::global::global_set;
use self::local::block;
use self::local::br;
use self::local::br_if;
use self::local::br_on_cast;
use self::local::br_on_cast_desc_eq;
use self::local::br_on_cast_desc_eq_fail;
use self::local::br_on_cast_fail;
use self::local::br_on_non_null;
use self::local::br_on_null;
use self::local::br_table;
use self::local::call;
use self::local::call_indirect;
use self::local::call_ref;
use self::local::cont_bind;
use self::local::cont_new;
use self::local::if_;
use self::local::local_get;
use self::local::local_set;
use self::local::local_tee;
use self::local::loop_;
use self::local::resume;
use self::local::resume_throw;
use self::local::resume_throw_ref;
use self::local::return_call;
use self::local::return_call_indirect;
use self::local::return_call_ref;
use self::local::select;
use self::local::suspend;
use self::local::switch;
use self::local::throw;
use self::local::try_table;
use self::memory::data_drop;
use self::memory::memory_atomic_notify;
use self::memory::memory_atomic_wait32;
use self::memory::memory_atomic_wait64;
use self::memory::memory_copy;
use self::memory::memory_discard;
use self::memory::memory_fill;
use self::memory::memory_grow;
use self::memory::memory_init;
use self::memory::memory_size;
use self::number::f32_const;
use self::number::f32_load;
use self::number::f32_store;
use self::number::f32x4_extract_lane;
use self::number::f32x4_replace_lane;
use self::number::f64_const;
use self::number::f64_load;
use self::number::f64_store;
use self::number::f64x2_extract_lane;
use self::number::f64x2_replace_lane;
use self::number::i8x16_extract_lane_s;
use self::number::i8x16_extract_lane_u;
use self::number::i8x16_replace_lane;
use self::number::i8x16_shuffle;
use self::number::i16x8_extract_lane_s;
use self::number::i16x8_extract_lane_u;
use self::number::i16x8_replace_lane;
use self::number::i32_atomic_load;
use self::number::i32_atomic_load8_u;
use self::number::i32_atomic_load16_u;
use self::number::i32_atomic_rmw_add;
use self::number::i32_atomic_rmw_and;
use self::number::i32_atomic_rmw_cmpxchg;
use self::number::i32_atomic_rmw_or;
use self::number::i32_atomic_rmw_sub;
use self::number::i32_atomic_rmw_xchg;
use self::number::i32_atomic_rmw_xor;
use self::number::i32_atomic_rmw8_add_u;
use self::number::i32_atomic_rmw8_and_u;
use self::number::i32_atomic_rmw8_cmpxchg_u;
use self::number::i32_atomic_rmw8_or_u;
use self::number::i32_atomic_rmw8_sub_u;
use self::number::i32_atomic_rmw8_xchg_u;
use self::number::i32_atomic_rmw8_xor_u;
use self::number::i32_atomic_rmw16_add_u;
use self::number::i32_atomic_rmw16_and_u;
use self::number::i32_atomic_rmw16_cmpxchg_u;
use self::number::i32_atomic_rmw16_or_u;
use self::number::i32_atomic_rmw16_sub_u;
use self::number::i32_atomic_rmw16_xchg_u;
use self::number::i32_atomic_rmw16_xor_u;
use self::number::i32_atomic_store;
use self::number::i32_atomic_store8;
use self::number::i32_atomic_store16;
use self::number::i32_const;
use self::number::i32_load;
use self::number::i32_load8_s;
use self::number::i32_load8_u;
use self::number::i32_load16_s;
use self::number::i32_load16_u;
use self::number::i32_store;
use self::number::i32_store8;
use self::number::i32_store16;
use self::number::i32x4_extract_lane;
use self::number::i32x4_replace_lane;
use self::number::i64_atomic_load;
use self::number::i64_atomic_load8_u;
use self::number::i64_atomic_load16_u;
use self::number::i64_atomic_load32_u;
use self::number::i64_atomic_rmw_add;
use self::number::i64_atomic_rmw_and;
use self::number::i64_atomic_rmw_cmpxchg;
use self::number::i64_atomic_rmw_or;
use self::number::i64_atomic_rmw_sub;
use self::number::i64_atomic_rmw_xchg;
use self::number::i64_atomic_rmw_xor;
use self::number::i64_atomic_rmw8_add_u;
use self::number::i64_atomic_rmw8_and_u;
use self::number::i64_atomic_rmw8_cmpxchg_u;
use self::number::i64_atomic_rmw8_or_u;
use self::number::i64_atomic_rmw8_sub_u;
use self::number::i64_atomic_rmw8_xchg_u;
use self::number::i64_atomic_rmw8_xor_u;
use self::number::i64_atomic_rmw16_add_u;
use self::number::i64_atomic_rmw16_and_u;
use self::number::i64_atomic_rmw16_cmpxchg_u;
use self::number::i64_atomic_rmw16_or_u;
use self::number::i64_atomic_rmw16_sub_u;
use self::number::i64_atomic_rmw16_xchg_u;
use self::number::i64_atomic_rmw16_xor_u;
use self::number::i64_atomic_rmw32_add_u;
use self::number::i64_atomic_rmw32_and_u;
use self::number::i64_atomic_rmw32_cmpxchg_u;
use self::number::i64_atomic_rmw32_or_u;
use self::number::i64_atomic_rmw32_sub_u;
use self::number::i64_atomic_rmw32_xchg_u;
use self::number::i64_atomic_rmw32_xor_u;
use self::number::i64_atomic_store;
use self::number::i64_atomic_store8;
use self::number::i64_atomic_store16;
use self::number::i64_atomic_store32;
use self::number::i64_const;
use self::number::i64_load;
use self::number::i64_load8_s;
use self::number::i64_load8_u;
use self::number::i64_load16_s;
use self::number::i64_load16_u;
use self::number::i64_load32_s;
use self::number::i64_load32_u;
use self::number::i64_store;
use self::number::i64_store8;
use self::number::i64_store16;
use self::number::i64_store32;
use self::number::i64x2_extract_lane;
use self::number::i64x2_replace_lane;
use self::number::v128_const;
use self::number::v128_load;
use self::number::v128_load8_lane;
use self::number::v128_load8_splat;
use self::number::v128_load8x8_s;
use self::number::v128_load8x8_u;
use self::number::v128_load16_lane;
use self::number::v128_load16_splat;
use self::number::v128_load16x4_s;
use self::number::v128_load16x4_u;
use self::number::v128_load32_lane;
use self::number::v128_load32_splat;
use self::number::v128_load32_zero;
use self::number::v128_load32x2_s;
use self::number::v128_load32x2_u;
use self::number::v128_load64_lane;
use self::number::v128_load64_splat;
use self::number::v128_load64_zero;
use self::number::v128_store;
use self::number::v128_store8_lane;
use self::number::v128_store16_lane;
use self::number::v128_store32_lane;
use self::number::v128_store64_lane;
use self::reference::ref_cast_desc_eq_non_null;
use self::reference::ref_cast_desc_eq_nullable;
use self::reference::ref_cast_non_null;
use self::reference::ref_cast_nullable;
use self::reference::ref_func;
use self::reference::ref_get_desc;
use self::reference::ref_null;
use self::reference::ref_test_non_null;
use self::reference::ref_test_nullable;
use self::struct_::struct_atomic_get;
use self::struct_::struct_atomic_get_s;
use self::struct_::struct_atomic_get_u;
use self::struct_::struct_atomic_rmw_add;
use self::struct_::struct_atomic_rmw_and;
use self::struct_::struct_atomic_rmw_cmpxchg;
use self::struct_::struct_atomic_rmw_or;
use self::struct_::struct_atomic_rmw_sub;
use self::struct_::struct_atomic_rmw_xchg;
use self::struct_::struct_atomic_rmw_xor;
use self::struct_::struct_atomic_set;
use self::struct_::struct_get;
use self::struct_::struct_get_s;
use self::struct_::struct_get_u;
use self::struct_::struct_new;
use self::struct_::struct_new_default;
use self::struct_::struct_new_default_desc;
use self::struct_::struct_new_desc;
use self::struct_::struct_set;
use self::table::elem_drop;
use self::table::table_atomic_get;
use self::table::table_atomic_rmw_cmpxchg;
use self::table::table_atomic_rmw_xchg;
use self::table::table_atomic_set;
use self::table::table_copy;
use self::table::table_fill;
use self::table::table_get;
use self::table::table_grow;
use self::table::table_init;
use self::table::table_set;
use self::table::table_size;
use self::zero_arg::zero_arg_instruction;
use super::ParseCtx;
use super::Sections;
use super::key;
use super::resolve_name;

pub(super) fn instruction(
    ctx: &ParseCtx, local: &mut LocalCtx, sections: &mut Sections, sink: &mut InstructionSink,
    instruction: &Val,
) -> Option<()> {
    let (func, input) = match instruction {
        Val::Key(instruction) => return zero_arg_instruction(sink, instruction),
        Val::Call(call) => {
            let func = key(&call.func)?;
            (func, &call.input)
        },
        _ => return None,
    };
    match func.as_ref() {
        "local.get" => local_get(ctx, local, sink, input),
        "local.set" => local_set(ctx, local, sink, input),
        "local.tee" => local_tee(ctx, local, sink, input),
        "select" => select(ctx, local, sink, input),
        "block" => block(ctx, local, sections, sink, input),
        "if" => if_(ctx, local, sections, sink, input),
        "loop" => loop_(ctx, local, sections, sink, input),
        "try_table" => try_table(ctx, local, sections, sink, input),
        "branch" => br(ctx, local, sink, input),
        "branch_if" => br_if(ctx, local, sink, input),
        "branch_table" => br_table(ctx, local, sink, input),
        "branch_on_null" => br_on_null(ctx, local, sink, input),
        "branch_on_non_null" => br_on_non_null(ctx, local, sink, input),
        "branch_on_cast" => br_on_cast(ctx, local, sink, input),
        "branch_on_cast_fail" => br_on_cast_fail(ctx, local, sink, input),
        "branch_on_cast_descriptor_equal" => br_on_cast_desc_eq(ctx, local, sink, input),
        "branch_on_cast_descriptor_equal_fail" => br_on_cast_desc_eq_fail(ctx, local, sink, input),
        "call" => call(ctx, local, sink, input),
        "return_call" => return_call(ctx, local, sink, input),
        "call_reference" => call_ref(ctx, local, sink, input),
        "return_call_reference" => return_call_ref(ctx, local, sink, input),
        "call_indirect" => call_indirect(ctx, local, sink, input),
        "return_call_indirect" => return_call_indirect(ctx, local, sink, input),
        "throw" => throw(ctx, local, sink, input),
        "continuation.new" => cont_new(ctx, local, sink, input),
        "continuation.bind" => cont_bind(ctx, local, sink, input),
        "suspend" => suspend(ctx, local, sink, input),
        "resume" => resume(ctx, local, sink, input),
        "resume.throw" => resume_throw(ctx, local, sink, input),
        "resume.throw_reference" => resume_throw_ref(ctx, local, sink, input),
        "switch" => switch(ctx, local, sink, input),
        // ----------------------------------------------------------------
        "global.get" => global_get(ctx, sink, input),
        "global.set" => global_set(ctx, sink, input),
        "global.atomic.get" => global_atomic_get(ctx, sink, input),
        "global.atomic.set" => global_atomic_set(ctx, sink, input),
        "global.atomic.read_modify_write.+" => global_atomic_rmw_add(ctx, sink, input),
        "global.atomic.read_modify_write.-" => global_atomic_rmw_sub(ctx, sink, input),
        "global.atomic.read_modify_write.and" => global_atomic_rmw_and(ctx, sink, input),
        "global.atomic.read_modify_write.or" => global_atomic_rmw_or(ctx, sink, input),
        "global.atomic.read_modify_write.xor" => global_atomic_rmw_xor(ctx, sink, input),
        "global.atomic.read_modify_write.exchange" => global_atomic_rmw_xchg(ctx, sink, input),
        "global.atomic.read_modify_write.compare_exchange" => {
            global_atomic_rmw_cmpxchg(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "memory.init" => {
            sections.need_data_count = true;
            memory_init(ctx, sink, input)
        },
        "memory.size" => memory_size(ctx, sink, input),
        "memory.grow" => memory_grow(ctx, sink, input),
        "memory.fill" => memory_fill(ctx, sink, input),
        "memory.copy" => memory_copy(ctx, sink, input),
        "memory.discard" => memory_discard(ctx, sink, input),
        "memory.atomic.notify" => memory_atomic_notify(ctx, sink, input),
        "memory.atomic.wait32" => memory_atomic_wait32(ctx, sink, input),
        "memory.atomic.wait64" => memory_atomic_wait64(ctx, sink, input),
        "data.drop" => {
            sections.need_data_count = true;
            data_drop(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "table.init" => table_init(ctx, sink, input),
        "table.get" => table_get(ctx, sink, input),
        "table.set" => table_set(ctx, sink, input),
        "table.size" => table_size(ctx, sink, input),
        "table.grow" => table_grow(ctx, sink, input),
        "table.fill" => table_fill(ctx, sink, input),
        "table.copy" => table_copy(ctx, sink, input),
        "table.atomic.get" => table_atomic_get(ctx, sink, input),
        "table.atomic.set" => table_atomic_set(ctx, sink, input),
        "table.atomic.read_modify_write.exchange" => table_atomic_rmw_xchg(ctx, sink, input),
        "table.atomic.read_modify_write.compare_exchange" => {
            table_atomic_rmw_cmpxchg(ctx, sink, input)
        },
        "element.drop" => elem_drop(ctx, sink, input),
        // ----------------------------------------------------------------
        "reference.null" => ref_null(ctx, sink, input),
        "reference.function" => ref_func(ctx, sink, input),
        "reference.get_descriptor" => ref_get_desc(ctx, sink, input),
        "reference.test_nullable" => ref_test_nullable(ctx, sink, input),
        "reference.test_non_null" => ref_test_non_null(ctx, sink, input),
        "reference.cast_nullable" => ref_cast_nullable(ctx, sink, input),
        "reference.cast_non_null" => ref_cast_non_null(ctx, sink, input),
        "reference.cast_descriptor_equal_nullable" => ref_cast_desc_eq_nullable(ctx, sink, input),
        "reference.cast_descriptor_equal_non_null" => ref_cast_desc_eq_non_null(ctx, sink, input),
        // ----------------------------------------------------------------
        "struct.new" => struct_new(ctx, sink, input),
        "struct.new_default" => struct_new_default(ctx, sink, input),
        "struct.new_descriptor" => struct_new_desc(ctx, sink, input),
        "struct.new_default_descriptor" => struct_new_default_desc(ctx, sink, input),
        "struct.get" => struct_get(ctx, sink, input),
        "struct.get_signed" => struct_get_s(ctx, sink, input),
        "struct.get_unsigned" => struct_get_u(ctx, sink, input),
        "struct.set" => struct_set(ctx, sink, input),
        "struct.atomic.get" => struct_atomic_get(ctx, sink, input),
        "struct.atomic.get_signed" => struct_atomic_get_s(ctx, sink, input),
        "struct.atomic.get_unsigned" => struct_atomic_get_u(ctx, sink, input),
        "struct.atomic.set" => struct_atomic_set(ctx, sink, input),
        "struct.atomic.read_modify_write.+" => struct_atomic_rmw_add(ctx, sink, input),
        "struct.atomic.read_modify_write.-" => struct_atomic_rmw_sub(ctx, sink, input),
        "struct.atomic.read_modify_write.and" => struct_atomic_rmw_and(ctx, sink, input),
        "struct.atomic.read_modify_write.or" => struct_atomic_rmw_or(ctx, sink, input),
        "struct.atomic.read_modify_write.xor" => struct_atomic_rmw_xor(ctx, sink, input),
        "struct.atomic.read_modify_write.exchange" => struct_atomic_rmw_xchg(ctx, sink, input),
        "struct.atomic.read_modify_write.compare_exchange" => {
            struct_atomic_rmw_cmpxchg(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "array.new" => array_new(ctx, sink, input),
        "array.new_default" => array_new_default(ctx, sink, input),
        "array.new_fixed" => array_new_fixed(ctx, sink, input),
        "array.new_data" => {
            sections.need_data_count = true;
            array_new_data(ctx, sink, input)
        },
        "array.init_data" => {
            sections.need_data_count = true;
            array_init_data(ctx, sink, input)
        },
        "array.new_element" => array_new_elem(ctx, sink, input),
        "array.init_element" => array_init_elem(ctx, sink, input),
        "array.get" => array_get(ctx, sink, input),
        "array.get_signed" => array_get_s(ctx, sink, input),
        "array.get_unsigned" => array_get_u(ctx, sink, input),
        "array.set" => array_set(ctx, sink, input),
        "array.fill" => array_fill(ctx, sink, input),
        "array.copy" => array_copy(ctx, sink, input),
        "array.atomic.get" => array_atomic_get(ctx, sink, input),
        "array.atomic.get_signed" => array_atomic_get_s(ctx, sink, input),
        "array.atomic.get_unsigned" => array_atomic_get_u(ctx, sink, input),
        "array.atomic.set" => array_atomic_set(ctx, sink, input),
        "array.atomic.read_modify_write.+" => array_atomic_rmw_add(ctx, sink, input),
        "array.atomic.read_modify_write.-" => array_atomic_rmw_sub(ctx, sink, input),
        "array.atomic.read_modify_write.and" => array_atomic_rmw_and(ctx, sink, input),
        "array.atomic.read_modify_write.or" => array_atomic_rmw_or(ctx, sink, input),
        "array.atomic.read_modify_write.xor" => array_atomic_rmw_xor(ctx, sink, input),
        "array.atomic.read_modify_write.exchange" => array_atomic_rmw_xchg(ctx, sink, input),
        "array.atomic.read_modify_write.compare_exchange" => {
            array_atomic_rmw_cmpxchg(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "i32.constant" => i32_const(ctx, sink, input),
        "i32.load" => i32_load(ctx, sink, input),
        "i32.load8_signed" => i32_load8_s(ctx, sink, input),
        "i32.load8_unsigned" => i32_load8_u(ctx, sink, input),
        "i32.load16_signed" => i32_load16_s(ctx, sink, input),
        "i32.load16_unsigned" => i32_load16_u(ctx, sink, input),
        "i32.store" => i32_store(ctx, sink, input),
        "i32.store8" => i32_store8(ctx, sink, input),
        "i32.store16" => i32_store16(ctx, sink, input),
        "i32.atomic.load" => i32_atomic_load(ctx, sink, input),
        "i32.atomic.load8_unsigned" => i32_atomic_load8_u(ctx, sink, input),
        "i32.atomic.load16_unsigned" => i32_atomic_load16_u(ctx, sink, input),
        "i32.atomic.store" => i32_atomic_store(ctx, sink, input),
        "i32.atomic.store8" => i32_atomic_store8(ctx, sink, input),
        "i32.atomic.store16" => i32_atomic_store16(ctx, sink, input),
        "i32.atomic.read_modify_write.+" => i32_atomic_rmw_add(ctx, sink, input),
        "i32.atomic.read_modify_write.-" => i32_atomic_rmw_sub(ctx, sink, input),
        "i32.atomic.read_modify_write.and" => i32_atomic_rmw_and(ctx, sink, input),
        "i32.atomic.read_modify_write.or" => i32_atomic_rmw_or(ctx, sink, input),
        "i32.atomic.read_modify_write.xor" => i32_atomic_rmw_xor(ctx, sink, input),
        "i32.atomic.read_modify_write.exchange" => i32_atomic_rmw_xchg(ctx, sink, input),
        "i32.atomic.read_modify_write.compare_exchange" => i32_atomic_rmw_cmpxchg(ctx, sink, input),
        "i32.atomic.read_modify_write8.+_unsigned" => i32_atomic_rmw8_add_u(ctx, sink, input),
        "i32.atomic.read_modify_write8.-_unsigned" => i32_atomic_rmw8_sub_u(ctx, sink, input),
        "i32.atomic.read_modify_write8.and_unsigned" => i32_atomic_rmw8_and_u(ctx, sink, input),
        "i32.atomic.read_modify_write8.or_unsigned" => i32_atomic_rmw8_or_u(ctx, sink, input),
        "i32.atomic.read_modify_write8.xor_unsigned" => i32_atomic_rmw8_xor_u(ctx, sink, input),
        "i32.atomic.read_modify_write8.exchange_unsigned" => {
            i32_atomic_rmw8_xchg_u(ctx, sink, input)
        },
        "i32.atomic.read_modify_write8.compare_exchange_unsigned" => {
            i32_atomic_rmw8_cmpxchg_u(ctx, sink, input)
        },
        "i32.atomic.read_modify_write16.+_unsigned" => i32_atomic_rmw16_add_u(ctx, sink, input),
        "i32.atomic.read_modify_write16.-_unsigned" => i32_atomic_rmw16_sub_u(ctx, sink, input),
        "i32.atomic.read_modify_write16.and_unsigned" => i32_atomic_rmw16_and_u(ctx, sink, input),
        "i32.atomic.read_modify_write16.or_unsigned" => i32_atomic_rmw16_or_u(ctx, sink, input),
        "i32.atomic.read_modify_write16.xor_unsigned" => i32_atomic_rmw16_xor_u(ctx, sink, input),
        "i32.atomic.read_modify_write16.exchange_unsigned" => {
            i32_atomic_rmw16_xchg_u(ctx, sink, input)
        },
        "i32.atomic.read_modify_write16.compare_exchange_unsigned" => {
            i32_atomic_rmw16_cmpxchg_u(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "i64.constant" => i64_const(ctx, sink, input),
        "i64.load" => i64_load(ctx, sink, input),
        "i64.load8_signed" => i64_load8_s(ctx, sink, input),
        "i64.load8_unsigned" => i64_load8_u(ctx, sink, input),
        "i64.load16_signed" => i64_load16_s(ctx, sink, input),
        "i64.load16_unsigned" => i64_load16_u(ctx, sink, input),
        "i64.load32_signed" => i64_load32_s(ctx, sink, input),
        "i64.load32_unsigned" => i64_load32_u(ctx, sink, input),
        "i64.store" => i64_store(ctx, sink, input),
        "i64.store8" => i64_store8(ctx, sink, input),
        "i64.store16" => i64_store16(ctx, sink, input),
        "i64.store32" => i64_store32(ctx, sink, input),
        "i64.atomic.load" => i64_atomic_load(ctx, sink, input),
        "i64.atomic.load8_unsigned" => i64_atomic_load8_u(ctx, sink, input),
        "i64.atomic.load16_unsigned" => i64_atomic_load16_u(ctx, sink, input),
        "i64.atomic.load32_unsigned" => i64_atomic_load32_u(ctx, sink, input),
        "i64.atomic.store" => i64_atomic_store(ctx, sink, input),
        "i64.atomic.store8" => i64_atomic_store8(ctx, sink, input),
        "i64.atomic.store16" => i64_atomic_store16(ctx, sink, input),
        "i64.atomic.store32" => i64_atomic_store32(ctx, sink, input),
        "i64.atomic.read_modify_write.+" => i64_atomic_rmw_add(ctx, sink, input),
        "i64.atomic.read_modify_write.-" => i64_atomic_rmw_sub(ctx, sink, input),
        "i64.atomic.read_modify_write.and" => i64_atomic_rmw_and(ctx, sink, input),
        "i64.atomic.read_modify_write.or" => i64_atomic_rmw_or(ctx, sink, input),
        "i64.atomic.read_modify_write.xor" => i64_atomic_rmw_xor(ctx, sink, input),
        "i64.atomic.read_modify_write.exchange" => i64_atomic_rmw_xchg(ctx, sink, input),
        "i64.atomic.read_modify_write.compare_exchange" => i64_atomic_rmw_cmpxchg(ctx, sink, input),
        "i64.atomic.read_modify_write8.+_unsigned" => i64_atomic_rmw8_add_u(ctx, sink, input),
        "i64.atomic.read_modify_write8.-_unsigned" => i64_atomic_rmw8_sub_u(ctx, sink, input),
        "i64.atomic.read_modify_write8.and_unsigned" => i64_atomic_rmw8_and_u(ctx, sink, input),
        "i64.atomic.read_modify_write8.or_unsigned" => i64_atomic_rmw8_or_u(ctx, sink, input),
        "i64.atomic.read_modify_write8.xor_unsigned" => i64_atomic_rmw8_xor_u(ctx, sink, input),
        "i64.atomic.read_modify_write8.exchange_unsigned" => {
            i64_atomic_rmw8_xchg_u(ctx, sink, input)
        },
        "i64.atomic.read_modify_write8.compare_exchange_unsigned" => {
            i64_atomic_rmw8_cmpxchg_u(ctx, sink, input)
        },
        "i64.atomic.read_modify_write16.+_unsigned" => i64_atomic_rmw16_add_u(ctx, sink, input),
        "i64.atomic.read_modify_write16.-_unsigned" => i64_atomic_rmw16_sub_u(ctx, sink, input),
        "i64.atomic.read_modify_write16.and_unsigned" => i64_atomic_rmw16_and_u(ctx, sink, input),
        "i64.atomic.read_modify_write16.or_unsigned" => i64_atomic_rmw16_or_u(ctx, sink, input),
        "i64.atomic.read_modify_write16.xor_unsigned" => i64_atomic_rmw16_xor_u(ctx, sink, input),
        "i64.atomic.read_modify_write16.exchange_unsigned" => {
            i64_atomic_rmw16_xchg_u(ctx, sink, input)
        },
        "i64.atomic.read_modify_write16.compare_exchange_unsigned" => {
            i64_atomic_rmw16_cmpxchg_u(ctx, sink, input)
        },
        "i64.atomic.read_modify_write32.+_unsigned" => i64_atomic_rmw32_add_u(ctx, sink, input),
        "i64.atomic.read_modify_write32.-_unsigned" => i64_atomic_rmw32_sub_u(ctx, sink, input),
        "i64.atomic.read_modify_write32.and_unsigned" => i64_atomic_rmw32_and_u(ctx, sink, input),
        "i64.atomic.read_modify_write32.or_unsigned" => i64_atomic_rmw32_or_u(ctx, sink, input),
        "i64.atomic.read_modify_write32.xor_unsigned" => i64_atomic_rmw32_xor_u(ctx, sink, input),
        "i64.atomic.read_modify_write32.exchange_unsigned" => {
            i64_atomic_rmw32_xchg_u(ctx, sink, input)
        },
        "i64.atomic.read_modify_write32.compare_exchange_unsigned" => {
            i64_atomic_rmw32_cmpxchg_u(ctx, sink, input)
        },
        // ----------------------------------------------------------------
        "f32.constant" => f32_const(ctx, sink, input),
        "f32.load" => f32_load(ctx, sink, input),
        "f32.store" => f32_store(ctx, sink, input),
        // ----------------------------------------------------------------
        "f64.constant" => f64_const(ctx, sink, input),
        "f64.load" => f64_load(ctx, sink, input),
        "f64.store" => f64_store(ctx, sink, input),
        // ----------------------------------------------------------------
        "v128.constant" => v128_const(ctx, sink, input),
        "v128.load" => v128_load(ctx, sink, input),
        "v128.load8x8_signed" => v128_load8x8_s(ctx, sink, input),
        "v128.load8x8_unsigned" => v128_load8x8_u(ctx, sink, input),
        "v128.load16x4_signed" => v128_load16x4_s(ctx, sink, input),
        "v128.load16x4_unsigned" => v128_load16x4_u(ctx, sink, input),
        "v128.load32x2_signed" => v128_load32x2_s(ctx, sink, input),
        "v128.load32x2_unsigned" => v128_load32x2_u(ctx, sink, input),
        "v128.load8_splat" => v128_load8_splat(ctx, sink, input),
        "v128.load8_lane" => v128_load8_lane(ctx, sink, input),
        "v128.load16_splat" => v128_load16_splat(ctx, sink, input),
        "v128.load16_lane" => v128_load16_lane(ctx, sink, input),
        "v128.load32_splat" => v128_load32_splat(ctx, sink, input),
        "v128.load32_lane" => v128_load32_lane(ctx, sink, input),
        "v128.load32_zero" => v128_load32_zero(ctx, sink, input),
        "v128.load64_splat" => v128_load64_splat(ctx, sink, input),
        "v128.load64_lane" => v128_load64_lane(ctx, sink, input),
        "v128.load64_zero" => v128_load64_zero(ctx, sink, input),
        "v128.store" => v128_store(ctx, sink, input),
        "v128.store8_lane" => v128_store8_lane(ctx, sink, input),
        "v128.store16_lane" => v128_store16_lane(ctx, sink, input),
        "v128.store32_lane" => v128_store32_lane(ctx, sink, input),
        "v128.store64_lane" => v128_store64_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        "i8x16.extract_lane_signed" => i8x16_extract_lane_s(ctx, sink, input),
        "i8x16.extract_lane_unsigned" => i8x16_extract_lane_u(ctx, sink, input),
        "i8x16.replace_lane" => i8x16_replace_lane(ctx, sink, input),
        "i8x16.shuffle" => i8x16_shuffle(ctx, sink, input),
        // ----------------------------------------------------------------
        "i16x8.extract_lane_signed" => i16x8_extract_lane_s(ctx, sink, input),
        "i16x8.extract_lane_unsigned" => i16x8_extract_lane_u(ctx, sink, input),
        "i16x8.replace_lane" => i16x8_replace_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        "i32x4.extract_lane" => i32x4_extract_lane(ctx, sink, input),
        "i32x4.replace_lane" => i32x4_replace_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        "i64x2.extract_lane" => i64x2_extract_lane(ctx, sink, input),
        "i64x2.replace_lane" => i64x2_replace_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        "f32x4.extract_lane" => f32x4_extract_lane(ctx, sink, input),
        "f32x4.replace_lane" => f32x4_replace_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        "f64x2.extract_lane" => f64x2_extract_lane(ctx, sink, input),
        "f64x2.replace_lane" => f64x2_replace_lane(ctx, sink, input),
        // ----------------------------------------------------------------
        _ => None,
    }
}

fn ordering(input: &Val) -> Option<Ordering> {
    let key = key(input)?;
    match key.as_ref() {
        "acquire_release" => Some(Ordering::AcqRel),
        "sequential_consistency" => Some(Ordering::SeqCst),
        _ => None,
    }
}

mod local;

mod global;

mod number;

mod memory;

mod table;

mod struct_;

mod array;

mod reference;

mod constant;

mod zero_arg;
