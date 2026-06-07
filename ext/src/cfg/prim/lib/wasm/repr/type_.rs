use std::collections::hash_map::Entry;

use airlang::semantics::val::Val;
use airlang::type_::Call;
use airlang::type_::Key;
use airlang::type_::List;
use airlang::type_::Map;
use wasm_encoder::AbstractHeapType;
use wasm_encoder::ArrayType;
use wasm_encoder::CompositeInnerType;
use wasm_encoder::CompositeType;
use wasm_encoder::ContType;
use wasm_encoder::FieldType;
use wasm_encoder::FuncType;
use wasm_encoder::HeapType;
use wasm_encoder::RefType;
use wasm_encoder::StorageType;
use wasm_encoder::StructType;
use wasm_encoder::SubType;
use wasm_encoder::ValType;

use super::ParseCtx;
use super::Sections;
use super::contains;
use super::exact_key;
use super::get;
use super::key;
use super::key_call;
use super::list;
use super::map;
use super::pair;
use super::register_name;
use super::resolve_name;
use super::resolve_type;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum TypeKind {
    Func,
    Struct,
    Array,
    Cont,
}

pub(super) fn collect_type(ctx: &mut ParseCtx, input: &Val) -> Option<()> {
    match input {
        Val::List(list) => {
            for item in list.iter() {
                collect_composite_type(ctx, key_call(item)?)?;
            }
            Some(())
        },
        Val::Call(call) => {
            let call = Call::new(key(&call.func)?, &call.input);
            collect_composite_type(ctx, call)
        },
        _ => None,
    }
}

fn collect_composite_type(ctx: &mut ParseCtx, call: Call<&Key, &Val>) -> Option<()> {
    let pair0 = pair(call.input)?;
    let name = &pair0.left;
    let kind = match call.func.as_ref() {
        "function" => {
            let input = list(get(map(&pair0.right)?, "input")?)?;
            ctx.function_input_length.insert(ctx.type_count, input.len() as u32);
            TypeKind::Func
        },
        "struct" => {
            let list = list(get(map(&pair0.right)?, "fields")?)?;
            collect_fields(ctx, ctx.type_count, list)?;
            TypeKind::Struct
        },
        "array" => TypeKind::Array,
        "continuation" => TypeKind::Cont,
        _ => return None,
    };
    register_name(&mut ctx.type_, name, (kind, ctx.type_count))?;
    ctx.type_count += 1;
    Some(())
}

fn collect_fields(ctx: &mut ParseCtx, struct_: u32, list: &List<Val>) -> Option<()> {
    for (index, field) in list.iter().enumerate() {
        match ctx.field.entry((struct_, key_call(field)?.func.clone())) {
            Entry::Occupied(_) => return None,
            Entry::Vacant(entry) => {
                entry.insert(index as u32);
            },
        }
    }
    Some(())
}

pub(super) fn parse_type(ctx: &ParseCtx, sections: &mut Sections, input: &Val) -> Option<()> {
    match input {
        Val::List(list) => {
            let mut subtypes = Vec::with_capacity(list.len());
            for item in list.iter() {
                subtypes.push(parse_composite_type(ctx, false, key_call(item)?)?);
            }
            sections.type_.ty().rec(subtypes);
            Some(())
        },
        Val::Call(call) => {
            let call = Call::new(key(&call.func)?, &call.input);
            let sub_type = parse_composite_type(ctx, true, call)?;
            sections.type_.ty().subtype(&sub_type);
            Some(())
        },
        _ => None,
    }
}

fn parse_composite_type(ctx: &ParseCtx, single: bool, call: Call<&Key, &Val>) -> Option<SubType> {
    let map = map(&pair(call.input)?.right)?;
    match call.func.as_ref() {
        "function" => {
            let input = parse_val_types(ctx, list(get(map, "input")?)?)?;
            let output = parse_val_types(ctx, list(get(map, "output")?)?)?;
            let inner = CompositeInnerType::Func(FuncType::new(input, output));
            subtype_info(ctx, single, map, inner)
        },
        "struct" => {
            let list = list(get(map, "fields")?)?;
            let fields: Option<Vec<FieldType>> = list.iter().map(|v| field_type(ctx, v)).collect();
            let fields = fields?.into_boxed_slice();
            let inner = CompositeInnerType::Struct(StructType { fields });
            subtype_info(ctx, single, map, inner)
        },
        "array" => {
            let element_type = storage_type(ctx, get(map, "item")?)?;
            let mutable = contains(map, "mutable")?;
            let inner = CompositeInnerType::Array(ArrayType(FieldType { mutable, element_type }));
            subtype_info(ctx, single, map, inner)
        },
        "continuation" => {
            let type_ = resolve_type(&ctx.type_, TypeKind::Func, get(map, "function")?)?;
            let inner = CompositeInnerType::Cont(ContType(type_));
            subtype_info(ctx, single, map, inner)
        },
        _ => None,
    }
}

fn subtype_info(
    ctx: &ParseCtx, single: bool, map: &Map<Key, Val>, inner: CompositeInnerType,
) -> Option<SubType> {
    let shared = contains(map, "shared")?;
    if single {
        let composite_type = CompositeType { inner, shared, descriptor: None, describes: None };
        return Some(SubType { is_final: true, supertype_idx: None, composite_type });
    }
    let is_final = contains(map, "final")?;
    let supertype_idx = match get(map, "parent") {
        None => None,
        Some(v) => Some(resolve_name(&ctx.type_, v)?.1),
    };
    let descriptor = match get(map, "descriptor") {
        None => None,
        Some(v) => Some(resolve_name(&ctx.type_, v)?.1),
    };
    let describes = match get(map, "describes") {
        None => None,
        Some(v) => Some(resolve_name(&ctx.type_, v)?.1),
    };
    let composite_type = CompositeType { inner, shared, descriptor, describes };
    Some(SubType { is_final, supertype_idx, composite_type })
}

fn field_type(ctx: &ParseCtx, input: &Val) -> Option<FieldType> {
    let pair = pair(key_call(input)?.input)?;
    let map = map(&pair.left)?;
    let mutable = contains(map, "mutable")?;
    let element_type = storage_type(ctx, &pair.right)?;
    Some(FieldType { element_type, mutable })
}

pub(super) fn parse_val_types(ctx: &ParseCtx, input: &List<Val>) -> Option<Box<[ValType]>> {
    let types: Option<Vec<ValType>> = input.iter().map(|v| parse_val_type(ctx, v)).collect();
    Some(types?.into_boxed_slice())
}

pub(super) fn parse_val_type(ctx: &ParseCtx, value: &Val) -> Option<ValType> {
    if let StorageType::Val(val_type) = storage_type(ctx, value)? { Some(val_type) } else { None }
}

fn storage_type(ctx: &ParseCtx, value: &Val) -> Option<StorageType> {
    let type_ = match value {
        Val::Key(k) => match k.as_ref() {
            "i8" => StorageType::I8,
            "i16" => StorageType::I16,
            "i32" => StorageType::Val(ValType::I32),
            "i64" => StorageType::Val(ValType::I64),
            "f32" => StorageType::Val(ValType::F32),
            "f64" => StorageType::Val(ValType::F64),
            "v128" => StorageType::Val(ValType::V128),
            _ => return None,
        },
        Val::Call(_) => StorageType::Val(ValType::Ref(ref_type(ctx, value)?)),
        _ => return None,
    };
    Some(type_)
}

pub(super) fn ref_type(ctx: &ParseCtx, input: &Val) -> Option<RefType> {
    let call = key_call(input)?;
    exact_key("reference", call.func)?;
    let pair = pair(call.input)?;
    let nullable = contains(map(&pair.left)?, "nullable")?;
    let heap_type = heap_type(ctx, &pair.right)?;
    Some(RefType { nullable, heap_type })
}

pub(super) fn heap_type(ctx: &ParseCtx, input: &Val) -> Option<HeapType> {
    let call = key_call(input)?;
    exact_key("heap", call.func)?;
    let pair = pair(call.input)?;
    let map = map(&pair.left)?;
    let abstract_ = contains(map, "abstract")?;
    if abstract_ {
        let shared = contains(map, "shared")?;
        let ty = abstract_heap_type(key(&pair.right)?)?;
        Some(HeapType::Abstract { shared, ty })
    } else {
        let exact = contains(map, "exact")?;
        let type_ = resolve_name(&ctx.type_, &pair.right)?.1;
        let heap_type = if exact { HeapType::Exact(type_) } else { HeapType::Concrete(type_) };
        Some(heap_type)
    }
}

fn abstract_heap_type(type_: &str) -> Option<AbstractHeapType> {
    match type_ {
        "function" => Some(AbstractHeapType::Func),
        "no_function" => Some(AbstractHeapType::NoFunc),
        "extern" => Some(AbstractHeapType::Extern),
        "no_extern" => Some(AbstractHeapType::NoExtern),
        "continuation" => Some(AbstractHeapType::Cont),
        "no_continuation" => Some(AbstractHeapType::NoCont),
        "exception" => Some(AbstractHeapType::Exn),
        "no_exception" => Some(AbstractHeapType::NoExn),
        "any" => Some(AbstractHeapType::Any),
        "none" => Some(AbstractHeapType::None),
        "struct" => Some(AbstractHeapType::Struct),
        "array" => Some(AbstractHeapType::Array),
        "i31" => Some(AbstractHeapType::I31),
        "equal" => Some(AbstractHeapType::Eq),
        _ => None,
    }
}
