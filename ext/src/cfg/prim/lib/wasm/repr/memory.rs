use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::MemoryType;

use super::ParseCtx;
use super::Sections;
use super::contains;
use super::get;
use super::instruction::const_expr;
use super::map;
use super::pair;
use super::register_name;
use super::resolve_name;
use super::u64;

pub(super) fn collect_memory(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.memory, &pair.left, ctx.memory_count)?;
    ctx.memory_count += 1;
    Some(())
}

pub(super) fn parse_memory(sections: &mut Sections, input: &Val) -> Option<()> {
    let memory = memory_type(map(&pair(input)?.right)?)?;
    sections.memory.memory(memory);
    Some(())
}

pub(super) fn memory_type(map: &Map<Key, Val>) -> Option<MemoryType> {
    let minimum = u64(get(map, "min")?)?;
    let maximum = if let Some(max) = get(map, "max") { Some(u64(max)?) } else { None };
    let shared = contains(map, "shared")?;
    let memory64 = contains(map, "64")?;
    // todo impl support page_size_log2
    Some(MemoryType { minimum, maximum, memory64, shared, page_size_log2: None })
}

pub(super) fn collect_data(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.data, &pair.left, ctx.data_count)?;
    ctx.data_count += 1;
    Some(())
}

pub(super) fn parse_data(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let map = map(&pair(input)?.right)?;
    let init = parse_data_bytes(get(map, "init")?)?;
    if let Some(offset) = get(map, "offset") {
        let memory = resolve_name(&ctx.memory, get(map, "memory")?)?;
        let offset = const_expr(ctx, offset)?;
        sections.data.active(memory, &offset, init);
    } else {
        sections.data.passive(init);
    }
    Some(())
}

fn parse_data_bytes(value: &Val) -> Option<Vec<u8>> {
    match value {
        Val::Text(t) => Some(t.as_bytes().to_vec()),
        Val::Byte(b) => Some(b.to_vec()),
        _ => None,
    }
}
