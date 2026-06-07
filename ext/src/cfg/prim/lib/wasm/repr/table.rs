use std::borrow::Cow;

use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::ConstExpr;
use wasm_encoder::Elements;
use wasm_encoder::TableType;

use super::ParseCtx;
use super::Sections;
use super::contains;
use super::exact_key;
use super::get;
use super::instruction::const_expr;
use super::list;
use super::map;
use super::pair;
use super::register_name;
use super::resolve_name;
use super::type_::ref_type;
use super::u64;

pub(super) fn collect_table(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.table, &pair.left, ctx.table_count)?;
    ctx.table_count += 1;
    Some(())
}

pub(super) fn parse_table(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let map = map(&pair(input)?.right)?;
    let table = table_type(ctx, map)?;
    if let Some(init) = get(map, "init") {
        let init = const_expr(ctx, init)?;
        sections.table.table_with_init(table, &init);
    } else {
        sections.table.table(table);
    }
    Some(())
}

pub(super) fn table_type(ctx: &ParseCtx, map: &Map<Key, Val>) -> Option<TableType> {
    let minimum = u64(get(map, "min")?)?;
    let maximum = if let Some(max) = get(map, "max") { Some(u64(max)?) } else { None };
    let element_type = ref_type(ctx, get(map, "type")?)?;
    let shared = contains(map, "shared")?;
    let table64 = contains(map, "64")?;
    Some(TableType { element_type, minimum, maximum, table64, shared })
}

pub(super) fn collect_element(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.element, &pair.left, ctx.element_count)?;
    ctx.element_count += 1;
    Some(())
}

pub(super) fn parse_element(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let map = map(&pair(input)?.right)?;
    let type_ = get(map, "type")?;
    let init = list(get(map, "init")?)?;
    let elements = element_init(ctx, type_, init)?;
    let declare = contains(map, "declare")?;
    if declare {
        sections.element.declared(elements);
        return Some(());
    }
    if let Some(offset) = get(map, "offset") {
        let table = Some(resolve_name(&ctx.table, get(map, "table")?)?);
        let offset = const_expr(ctx, offset)?;
        sections.element.active(table, &offset, elements);
    } else {
        sections.element.passive(elements);
    }
    Some(())
}

fn element_init(ctx: &ParseCtx, type_: &Val, list: &[Val]) -> Option<Elements<'static>> {
    if let Val::Key(type_) = type_ {
        exact_key("function", type_)?;
        let functions: Option<Vec<u32>> =
            list.iter().map(|v| resolve_name(&ctx.function, v)).collect();
        Some(Elements::Functions(Cow::Owned(functions?)))
    } else {
        let ref_type = ref_type(ctx, type_)?;
        let expressions: Option<Vec<ConstExpr>> = list.iter().map(|v| const_expr(ctx, v)).collect();
        Some(Elements::Expressions(ref_type, Cow::Owned(expressions?)))
    }
}
