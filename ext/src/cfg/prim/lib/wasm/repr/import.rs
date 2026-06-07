use std::borrow::Cow;

use airlang::semantics::val::Val;
use wasm_encoder::EntityType;
use wasm_encoder::ImportCompact;
use wasm_encoder::Imports;

use super::ParseCtx;
use super::Sections;
use super::contains;
use super::get;
use super::global::global_type;
use super::key_call;
use super::map;
use super::memory::memory_type;
use super::pair;
use super::register_name;
use super::resolve_type;
use super::table::table_type;
use super::tag::tag_type;
use super::text;
use crate::cfg::prim::lib::wasm::repr::type_::TypeKind;

// todo design support compact2
pub(super) fn collect_import(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    match &pair.right {
        Val::List(list) => {
            for item in list.iter() {
                collect_import_item(ctx, item)?;
            }
            Some(())
        },
        item => collect_import_item(ctx, item),
    }
}

fn collect_import_item(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let call = key_call(&pair(input)?.right)?;
    let name = &pair(call.input)?.left;
    match call.func.as_ref() {
        "function" => {
            register_name(&mut ctx.function, name, ctx.function_count)?;
            ctx.function_count += 1;
        },
        "global" => {
            register_name(&mut ctx.global, name, ctx.global_count)?;
            ctx.global_count += 1;
        },
        "memory" => {
            register_name(&mut ctx.memory, name, ctx.memory_count)?;
            ctx.memory_count += 1;
        },
        "table" => {
            register_name(&mut ctx.table, name, ctx.table_count)?;
            ctx.table_count += 1;
        },
        "tag" => {
            register_name(&mut ctx.tag, name, ctx.tag_count)?;
            ctx.tag_count += 1;
        },
        _ => return None,
    }
    Some(())
}

pub(super) fn parse_import(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    let module = text(&pair.left)?;
    match &pair.right {
        Val::List(list) => {
            let mut items = Vec::new();
            for item in list.iter() {
                items.push(parse_import_item(ctx, item)?);
            }
            sections.import.imports(Imports::Compact1 { module, items: Cow::Owned(items) });
        },
        item => {
            let item = parse_import_item(ctx, item)?;
            sections.import.import(module, item.name, item.ty);
        },
    }
    Some(())
}

fn parse_import_item<'a>(ctx: &ParseCtx, input: &'a Val) -> Option<ImportCompact<'a>> {
    let pair0 = pair(input)?;
    let name = text(&pair0.left)?;
    let call = key_call(&pair0.right)?;
    let map = map(&pair(call.input)?.right)?;
    match call.func.as_ref() {
        "function" => {
            let type_ = resolve_type(&ctx.type_, TypeKind::Func, get(map, "type")?)?;
            let exact = contains(map, "exact")?;
            let ty =
                if exact { EntityType::FunctionExact(type_) } else { EntityType::Function(type_) };
            Some(ImportCompact { name, ty })
        },
        "memory" => {
            let memory = memory_type(map)?;
            Some(ImportCompact { name, ty: EntityType::Memory(memory) })
        },
        "global" => {
            let global = global_type(ctx, map)?;
            Some(ImportCompact { name, ty: EntityType::Global(global) })
        },
        "table" => {
            let table = table_type(ctx, map)?;
            Some(ImportCompact { name, ty: EntityType::Table(table) })
        },
        "tag" => {
            let tag = tag_type(ctx, map)?;
            Some(ImportCompact { name, ty: EntityType::Tag(tag) })
        },
        _ => None,
    }
}
