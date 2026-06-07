use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::GlobalType;

use super::ParseCtx;
use super::Sections;
use super::contains;
use super::get;
use super::instruction::const_expr;
use super::map;
use super::pair;
use super::register_name;
use super::type_::parse_val_type;

pub(super) fn collect_global(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.global, &pair.left, ctx.global_count)?;
    ctx.global_count += 1;
    Some(())
}

pub(super) fn parse_global(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let map = map(&pair(input)?.right)?;
    let type_ = global_type(ctx, map)?;
    let const_expr = const_expr(ctx, get(map, "init")?)?;
    sections.global.global(type_, &const_expr);
    Some(())
}

pub(super) fn global_type(ctx: &ParseCtx, map: &Map<Key, Val>) -> Option<GlobalType> {
    let val_type = parse_val_type(ctx, get(map, "type")?)?;
    let mutable = contains(map, "mutable")?;
    let shared = contains(map, "shared")?;
    Some(GlobalType { val_type, mutable, shared })
}
