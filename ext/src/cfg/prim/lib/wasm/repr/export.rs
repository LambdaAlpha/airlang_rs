use airlang::semantics::val::Val;
use wasm_encoder::ExportKind;

use super::ParseCtx;
use super::Sections;
use super::key_call;
use super::pair;
use super::resolve_name;
use super::text;

pub(super) fn collect_export(_ctx: &mut ParseCtx, _input: &Val) -> Option<()> {
    Some(())
}

pub(super) fn parse_export(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    let id = text(&pair.left)?;
    let call = key_call(&pair.right)?;
    let input = &call.input;
    match call.func.as_ref() {
        "function" => {
            let index = resolve_name(&ctx.function, input)?;
            sections.export.export(id, ExportKind::Func, index);
        },
        "global" => {
            let index = resolve_name(&ctx.global, input)?;
            sections.export.export(id, ExportKind::Global, index);
        },
        "memory" => {
            let index = resolve_name(&ctx.memory, input)?;
            sections.export.export(id, ExportKind::Memory, index);
        },
        "table" => {
            let index = resolve_name(&ctx.table, input)?;
            sections.export.export(id, ExportKind::Table, index);
        },
        "tag" => {
            let index = resolve_name(&ctx.tag, input)?;
            sections.export.export(id, ExportKind::Tag, index);
        },
        _ => return None,
    }
    Some(())
}
