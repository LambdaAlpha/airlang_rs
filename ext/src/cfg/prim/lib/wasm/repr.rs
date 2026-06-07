use std::collections::HashMap;
use std::collections::hash_map::Entry;

use airlang::semantics::val::Val;
use airlang::type_::Call;
use airlang::type_::Decimal;
use airlang::type_::Int;
use airlang::type_::Key;
use airlang::type_::List;
use airlang::type_::Map;
use airlang::type_::Pair;
use airlang::type_::Text;
use num_traits::ToPrimitive;
use wasm_encoder::CodeSection;
use wasm_encoder::DataCountSection;
use wasm_encoder::DataSection;
use wasm_encoder::ElementSection;
use wasm_encoder::ExportSection;
use wasm_encoder::FunctionSection;
use wasm_encoder::GlobalSection;
use wasm_encoder::ImportSection;
use wasm_encoder::MemorySection;
use wasm_encoder::Module;
use wasm_encoder::StartSection;
use wasm_encoder::TableSection;
use wasm_encoder::TagSection;
use wasm_encoder::TypeSection;

use self::export::collect_export;
use self::export::parse_export;
use self::function::collect_function;
use self::function::parse_function;
use self::global::collect_global;
use self::global::parse_global;
use self::import::collect_import;
use self::import::parse_import;
use self::memory::collect_data;
use self::memory::collect_memory;
use self::memory::parse_data;
use self::memory::parse_memory;
use self::table::collect_element;
use self::table::collect_table;
use self::table::parse_element;
use self::table::parse_table;
use self::tag::collect_tag;
use self::tag::parse_tag;
use self::type_::TypeKind;
use self::type_::collect_type;
use self::type_::parse_type;

// Two-pass compilation:
//   1. collect — register names, compute index space counts (imports included)
//   2. parse — build sections using resolved indices
#[derive(Default)]
struct ParseCtx {
    function: HashMap<Key, u32>,
    global_count: u32,
    global: HashMap<Key, u32>,
    memory_count: u32,
    memory: HashMap<Key, u32>,
    data_count: u32,
    // resolve name in data section should set need_data_count to true in Sections
    data: HashMap<Key, u32>,
    table_count: u32,
    table: HashMap<Key, u32>,
    element_count: u32,
    element: HashMap<Key, u32>,
    tag_count: u32,
    tag: HashMap<Key, u32>,
    type_count: u32,
    type_: HashMap<Key, (TypeKind, u32)>,
    // (struct_type_index, field_name) -> index
    field: HashMap<(u32, Key), u32>,
    // func_type_index -> func_input_length
    function_input_length: HashMap<u32, u32>,
    function_count: u32,
}

#[derive(Default)]
struct Sections {
    import: ImportSection,
    export: ExportSection,
    function: FunctionSection,
    code: CodeSection,
    global: GlobalSection,
    memory: MemorySection,
    data: DataSection,
    need_data_count: bool,
    table: TableSection,
    element: ElementSection,
    tag: TagSection,
    type_: TypeSection,
    start: Option<StartSection>,
}

pub(super) fn parse_module(input: &Val) -> Option<Module> {
    let call = key_call(input)?;
    exact_key("module", call.func)?;
    let pair = pair(call.input)?;
    let map = map(&pair.left)?;
    let list = list(&pair.right)?;

    let mut ctx = ParseCtx::default();
    for entity in list {
        collect_entity(&mut ctx, entity)?;
    }
    let mut sections = Sections::default();
    for entity in list {
        parse_entity(&ctx, entity, &mut sections)?;
    }
    if let Some(start) = get(map, "start") {
        let function_index = resolve_name(&ctx.function, start)?;
        sections.start = Some(StartSection { function_index });
    }
    let module = encode_section(&sections);
    Some(module)
}

fn collect_entity(ctx: &mut ParseCtx, item: &Val) -> Option<()> {
    let call = key_call(item)?;
    let input = call.input;
    match call.func.as_ref() {
        "import" => collect_import(ctx, input),
        "export" => collect_export(ctx, input),
        "function" => collect_function(ctx, input),
        "global" => collect_global(ctx, input),
        "memory" => collect_memory(ctx, input),
        "data" => collect_data(ctx, input),
        "table" => collect_table(ctx, input),
        "element" => collect_element(ctx, input),
        "tag" => collect_tag(ctx, input),
        "type" => collect_type(ctx, input),
        _ => None,
    }
}

fn parse_entity(ctx: &ParseCtx, item: &Val, sections: &mut Sections) -> Option<()> {
    let call = key_call(item)?;
    let input = call.input;
    match call.func.as_ref() {
        "import" => parse_import(ctx, sections, input),
        "export" => parse_export(ctx, sections, input),
        "function" => parse_function(ctx, sections, input),
        "global" => parse_global(ctx, sections, input),
        "memory" => parse_memory(sections, input),
        "data" => parse_data(ctx, sections, input),
        "table" => parse_table(ctx, sections, input),
        "element" => parse_element(ctx, sections, input),
        "tag" => parse_tag(ctx, sections, input),
        "type" => parse_type(ctx, sections, input),
        _ => None,
    }
}

// Section order per binary spec
// https://webassembly.github.io/exception-handling/core/binary/modules.html#binary-module
fn encode_section(sections: &Sections) -> Module {
    let mut module = Module::new();
    if !sections.type_.is_empty() {
        module.section(&sections.type_);
    }
    if !sections.import.is_empty() {
        module.section(&sections.import);
    }
    if !sections.function.is_empty() {
        module.section(&sections.function);
    }
    if !sections.table.is_empty() {
        module.section(&sections.table);
    }
    if !sections.memory.is_empty() {
        module.section(&sections.memory);
    }
    if !sections.tag.is_empty() {
        module.section(&sections.tag);
    }
    if !sections.global.is_empty() {
        module.section(&sections.global);
    }
    if !sections.export.is_empty() {
        module.section(&sections.export);
    }
    if let Some(start) = sections.start {
        module.section(&start);
    }
    if !sections.element.is_empty() {
        module.section(&sections.element);
    }
    if sections.need_data_count {
        let count = DataCountSection { count: sections.data.len() };
        module.section(&count);
    }
    if !sections.code.is_empty() {
        module.section(&sections.code);
    }
    if !sections.data.is_empty() {
        module.section(&sections.data);
    }
    module
}

fn register_name<T: Copy>(names: &mut HashMap<Key, T>, name: &Val, idx: T) -> Option<()> {
    let Val::Key(name) = name else {
        return None;
    };
    match names.entry(name.clone()) {
        Entry::Occupied(_) => None,
        Entry::Vacant(entry) => {
            entry.insert(idx);
            Some(())
        },
    }
}

fn resolve_type(
    map: &HashMap<Key, (TypeKind, u32)>, required: TypeKind, input: &Val,
) -> Option<u32> {
    let (kind, index) = resolve_name(map, input)?;
    if kind != required {
        return None;
    }
    Some(index)
}

fn resolve_name<T: Copy>(map: &HashMap<Key, T>, name: &Val) -> Option<T> {
    let Val::Key(name) = name else {
        return None;
    };
    Some(*map.get(name)?)
}

fn key(val: &Val) -> Option<&Key> {
    let Val::Key(key) = val else {
        return None;
    };
    Some(key)
}

fn exact_key(target: &str, key: &str) -> Option<()> {
    if key != target {
        return None;
    }
    Some(())
}

fn text(val: &Val) -> Option<&Text> {
    let Val::Text(text) = val else {
        return None;
    };
    Some(text)
}

fn int(val: &Val) -> Option<&Int> {
    let Val::Int(int) = val else {
        return None;
    };
    Some(int)
}

fn u32(val: &Val) -> Option<u32> {
    int(val)?.to_u32()
}

fn u64(val: &Val) -> Option<u64> {
    int(val)?.to_u64()
}

fn decimal(val: &Val) -> Option<&Decimal> {
    let Val::Decimal(decimal) = val else {
        return None;
    };
    Some(decimal)
}

fn pair(val: &Val) -> Option<&Pair<Val, Val>> {
    let Val::Pair(pair) = val else {
        return None;
    };
    Some(pair)
}

fn triple(val: &Val) -> Option<(&Val, &Val, &Val)> {
    let Val::Pair(pair) = val else {
        return None;
    };
    let Val::Pair(right) = &pair.right else {
        return None;
    };
    Some((&pair.left, &right.left, &right.right))
}

fn list(val: &Val) -> Option<&List<Val>> {
    let Val::List(list) = val else {
        return None;
    };
    Some(list)
}

fn map(val: &Val) -> Option<&Map<Key, Val>> {
    let Val::Map(map) = val else {
        return None;
    };
    Some(map)
}

fn contains(map: &Map<Key, Val>, key: &str) -> Option<bool> {
    match map.get(&Key::from_str_unchecked(key)) {
        None => Some(false),
        Some(Val::Unit(_)) => Some(true),
        Some(_) => None,
    }
}

fn get<'a>(map: &'a Map<Key, Val>, key: &str) -> Option<&'a Val> {
    map.get(&Key::from_str_unchecked(key))
}

fn key_call(val: &Val) -> Option<Call<&Key, &Val>> {
    let Val::Call(call) = val else {
        return None;
    };
    let Val::Key(key) = &call.func else {
        return None;
    };
    Some(Call::new(key, &call.input))
}

mod import;

mod export;

mod function;

mod global;

mod memory;

mod table;

mod tag;

mod instruction;

mod type_;
