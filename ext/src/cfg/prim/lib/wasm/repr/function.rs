use std::collections::HashMap;

use airlang::semantics::val::Val;
use wasm_encoder::Function;

use super::ParseCtx;
use super::Sections;
use super::get;
use super::instruction::LocalCtx;
use super::instruction::instruction;
use super::key;
use super::list;
use super::map;
use super::pair;
use super::register_name;
use super::resolve_type;
use super::triple;
use super::type_::TypeKind;
use super::type_::parse_val_type;

pub(super) fn collect_function(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    let pair = pair(input)?;
    register_name(&mut ctx.function, &pair.left, ctx.function_count)?;
    ctx.function_count += 1;
    Some(())
}

pub(super) fn parse_function(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    let (_name, map0, body) = triple(input)?;
    let map = map(map0)?;
    let body = list(body)?;

    let func_type = resolve_type(&ctx.type_, TypeKind::Func, get(map, "type")?)?;
    let input_length = *ctx.function_input_length.get(&func_type)?;
    let input = list(get(map, "input")?)?;
    if input.len() != input_length as usize {
        return None;
    }
    let local = list(get(map, "local")?)?;
    let mut locals = HashMap::new();
    for (i, input) in input.iter().enumerate() {
        locals.insert(key(input)?.clone(), i as u32);
    }
    let mut local_types = Vec::new();
    for (i, local) in local.iter().enumerate() {
        let pair = pair(local)?;
        locals.insert(key(&pair.left)?.clone(), input_length + i as u32);
        let type_ = parse_val_type(ctx, &pair.right)?;
        local_types.push(type_);
    }

    let mut local = LocalCtx::new(locals);
    let mut func = Function::new_with_locals_types(local_types);
    let mut sink = func.instructions();
    for item in body {
        instruction(ctx, &mut local, sections, &mut sink, item)?;
    }
    sink.end();
    sections.function.function(func_type);
    sections.code.function(&func);
    Some(())
}
