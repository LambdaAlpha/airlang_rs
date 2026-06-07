use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
use wasm_encoder::TagKind;
use wasm_encoder::TagType;

use super::ParseCtx;
use super::Sections;
use super::exact_key;
use super::get;
use super::key;
use super::map;
use super::pair;
use super::register_name;
use super::resolve_type;
use super::type_::TypeKind;

pub(super) fn collect_tag(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.tag, &pair.left, ctx.tag_count)?;
    ctx.tag_count += 1;
    Some(())
}

pub(super) fn parse_tag(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let tag_type = tag_type(ctx, map(&pair(input)?.right)?)?;
    sections.tag.tag(tag_type);
    Some(())
}

pub(super) fn tag_type(ctx: &ParseCtx, map: &Map<Key, Val>) -> Option<TagType> {
    exact_key("exception", key(get(map, "attribute")?)?)?;
    let kind = TagKind::Exception;
    let func_type_idx = resolve_type(&ctx.type_, TypeKind::Func, get(map, "type")?)?;
    Some(TagType { kind, func_type_idx })
}
