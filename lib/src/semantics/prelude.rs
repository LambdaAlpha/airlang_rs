use crate::{
    semantics::{
        ctx::{
            Ctx,
            NameMap,
            TaggedVal,
        },
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            FuncEval,
            FuncImpl,
            Primitive,
        },
        input_mode::InputMode,
        val::Val,
        Func,
    },
    types::{
        Reader,
        Symbol,
    },
};

thread_local! (static PRELUDE: NameMap = init_prelude());

pub(crate) fn prelude() -> NameMap {
    PRELUDE.with(Clone::clone)
}

pub(crate) fn initial_ctx() -> Ctx {
    Ctx {
        name_map: prelude(),
        super_ctx: None,
    }
}

fn init_prelude() -> NameMap {
    let mut c = NameMap::default();

    prelude_meta(&mut c);
    prelude_syntax(&mut c);
    prelude_value(&mut c);
    prelude_ctx(&mut c);
    prelude_ctrl(&mut c);
    prelude_eval(&mut c);
    prelude_logic(&mut c);
    prelude_func(&mut c);
    prelude_call(&mut c);
    prelude_prop(&mut c);
    prelude_bool(&mut c);
    prelude_int(&mut c);
    prelude_str(&mut c);
    prelude_pair(&mut c);
    prelude_list(&mut c);
    prelude_map(&mut c);

    c
}

fn prelude_meta(c: &mut NameMap) {
    put(c, names::AIR_VERSION_MAJOR, meta::version_major());
    put(c, names::AIR_VERSION_MINOR, meta::version_minor());
    put(c, names::AIR_VERSION_PATCH, meta::version_patch());
}

fn prelude_syntax(c: &mut NameMap) {
    put_primitive_func(c, syntax::parse());
    put_primitive_func(c, syntax::stringify());
}

fn prelude_value(c: &mut NameMap) {
    put_primitive_func(c, value::type_of());
    put_primitive_func(c, value::equal());
    put_primitive_func(c, value::not_equal());
}

fn prelude_ctx(c: &mut NameMap) {
    put_primitive_func(c, ctx::read());
    put_primitive_func(c, ctx::remove());
    put_primitive_func(c, ctx::assign());
    put_primitive_func(c, ctx::set_final());
    put_primitive_func(c, ctx::set_const());
    put_primitive_func(c, ctx::is_final());
    put_primitive_func(c, ctx::is_const());
    put_primitive_func(c, ctx::is_null());
    put_primitive_func(c, ctx::is_local());
    put_primitive_func(c, ctx::ctx_get_super());
    put_primitive_func(c, ctx::ctx_set_super());
    put_primitive_func(c, ctx::ctx_new());
    put_primitive_func(c, ctx::ctx_repr());
    put_primitive_func(c, ctx::ctx_prelude());
    put_primitive_func(c, ctx::ctx_current());
}

fn prelude_ctrl(c: &mut NameMap) {
    put_primitive_func(c, ctrl::sequence());
    put_primitive_func(c, ctrl::breakable_sequence());
    put_primitive_func(c, ctrl::condition());
    put_primitive_func(c, ctrl::matching());
    put_primitive_func(c, ctrl::while_loop());
}

fn prelude_eval(c: &mut NameMap) {
    put_primitive_func(c, eval::value());
    put_primitive_func(c, eval::eval());
    put_primitive_func(c, eval::quote());
    put_primitive_func(c, eval::eval_twice());
    put_primitive_func(c, eval::eval_thrice());
}

fn prelude_logic(c: &mut NameMap) {
    put_primitive_func(c, logic::theorem_new());
    put_primitive_func(c, logic::prove());
}

fn prelude_func(c: &mut NameMap) {
    put_primitive_func(c, func::func_new());
    put_primitive_func(c, func::func_repr());
    put_primitive_func(c, func::func_input_mode());
    put_primitive_func(c, func::func_access());
    put_primitive_func(c, func::func_is_primitive());
    put_primitive_func(c, func::func_id());
    put_primitive_func(c, func::func_body());
    put_primitive_func(c, func::func_context());
    put_primitive_func(c, func::func_input_name());
    put_primitive_func(c, func::func_caller_name());
}

fn prelude_call(c: &mut NameMap) {
    put_primitive_func(c, call::chain());
    put_primitive_func(c, call::call_with_ctx());
}

fn prelude_prop(c: &mut NameMap) {
    put_primitive_func(c, prop::prop_new());
    put_primitive_func(c, prop::prop_repr());
    put_primitive_func(c, prop::get_truth());
    put_primitive_func(c, prop::get_function());
    put_primitive_func(c, prop::get_input());
    put_primitive_func(c, prop::get_output());
    put_primitive_func(c, prop::get_before());
    put_primitive_func(c, prop::get_after());
}

fn prelude_bool(c: &mut NameMap) {
    put_primitive_func(c, bool::not());
    put_primitive_func(c, bool::and());
    put_primitive_func(c, bool::or());
}

fn prelude_int(c: &mut NameMap) {
    put_primitive_func(c, int::add());
    put_primitive_func(c, int::subtract());
    put_primitive_func(c, int::multiply());
    put_primitive_func(c, int::divide());
    put_primitive_func(c, int::remainder());
    put_primitive_func(c, int::divide_remainder());
    put_primitive_func(c, int::less_than());
    put_primitive_func(c, int::less_equal());
    put_primitive_func(c, int::greater_than());
    put_primitive_func(c, int::greater_equal());
    put_primitive_func(c, int::less_greater());
}

fn prelude_str(c: &mut NameMap) {
    put_primitive_func(c, str::length());
    put_primitive_func(c, str::concat());
}

fn prelude_pair(c: &mut NameMap) {
    put_primitive_func(c, pair::first());
    put_primitive_func(c, pair::first_assign());
    put_primitive_func(c, pair::second());
    put_primitive_func(c, pair::second_assign());
}

fn prelude_list(c: &mut NameMap) {
    put_primitive_func(c, list::length());
    put_primitive_func(c, list::set());
    put_primitive_func(c, list::set_many());
    put_primitive_func(c, list::get());
    put_primitive_func(c, list::insert());
    put_primitive_func(c, list::insert_many());
    put_primitive_func(c, list::remove());
    put_primitive_func(c, list::push());
    put_primitive_func(c, list::push_many());
    put_primitive_func(c, list::pop());
    put_primitive_func(c, list::clear());
}

fn prelude_map(c: &mut NameMap) {
    put_primitive_func(c, map::length());
    put_primitive_func(c, map::keys());
    put_primitive_func(c, map::into_keys());
    put_primitive_func(c, map::values());
    put_primitive_func(c, map::into_values());
    put_primitive_func(c, map::contains());
    put_primitive_func(c, map::contains_many());
    put_primitive_func(c, map::set());
    put_primitive_func(c, map::set_many());
    put_primitive_func(c, map::get());
    put_primitive_func(c, map::get_many());
    put_primitive_func(c, map::remove());
    put_primitive_func(c, map::remove_many());
    put_primitive_func(c, map::clear());
}

fn put(c: &mut NameMap, key: &str, val: Val) {
    c.insert(Symbol::from_str(key), TaggedVal::new_const(val));
}

fn put_primitive_func<F>(c: &mut NameMap, primitive: PrimitiveFunc<F>)
where
    PrimitiveFunc<F>: Into<Func>,
{
    let id = primitive.evaluator.get_id().clone();
    let func = primitive.into();
    let val = Val::Func(Reader::new(func).into());
    c.insert(id, TaggedVal::new_const(val));
}

pub(crate) struct PrimitiveFunc<F> {
    input_mode: InputMode,
    evaluator: Primitive<F>,
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxFreeFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Free(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxConstFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Const(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Func> for PrimitiveFunc<CtxMutableFn> {
    fn into(self) -> Func {
        let evaluator = FuncEval::Mutable(FuncImpl::Primitive(self.evaluator));
        Func::new(self.input_mode, evaluator)
    }
}

impl<F> PrimitiveFunc<F> {
    fn new(input_mode: InputMode, primitive: Primitive<F>) -> Self {
        PrimitiveFunc {
            input_mode,
            evaluator: primitive,
        }
    }
}

pub(crate) mod names {
    pub(crate) const AIR_VERSION_MAJOR: &str = "air_version_major";
    pub(crate) const AIR_VERSION_MINOR: &str = "air_version_minor";
    pub(crate) const AIR_VERSION_PATCH: &str = "air_version_patch";

    pub(crate) const PARSE: &str = "parse";
    pub(crate) const STRINGIFY: &str = "stringify";

    pub(crate) const TYPE_OF: &str = "type_of";
    pub(crate) const EQUAL: &str = "==";
    pub(crate) const NOT_EQUAL: &str = "=/=";

    pub(crate) const READ: &str = "read";
    pub(crate) const MOVE: &str = "move";
    pub(crate) const ASSIGN: &str = "=";
    pub(crate) const SET_FINAL: &str = "set_final";
    pub(crate) const SET_CONST: &str = "set_constant";
    pub(crate) const IS_FINAL: &str = "is_final";
    pub(crate) const IS_CONST: &str = "is_constant";
    pub(crate) const IS_NULL: &str = "is_null";
    pub(crate) const IS_LOCAL: &str = "is_local";
    pub(crate) const CTX_GET_SUPER: &str = "get_super";
    pub(crate) const CTX_SET_SUPER: &str = "set_super";
    pub(crate) const CTX_NEW: &str = "context";
    pub(crate) const CTX_REPR: &str = "context_represent";
    pub(crate) const CTX_PRELUDE: &str = "prelude";
    pub(crate) const CTX_CURRENT: &str = "this";

    pub(crate) const SEQUENCE: &str = ";";
    pub(crate) const BREAKABLE_SEQUENCE: &str = ";return";
    pub(crate) const IF: &str = "if";
    pub(crate) const MATCH: &str = "match";
    pub(crate) const WHILE: &str = "while";

    pub(crate) const VALUE: &str = "'";
    pub(crate) const EVAL: &str = "`";
    pub(crate) const QUOTE: &str = "\"";
    pub(crate) const EVAL_TWICE: &str = "`2";
    pub(crate) const EVAL_THRICE: &str = "`3";

    pub(crate) const LOGIC_THEOREM_NEW: &str = "theorem";
    pub(crate) const LOGIC_PROVE: &str = "prove";

    pub(crate) const FUNC_NEW: &str = "function";
    pub(crate) const FUNC_REPR: &str = "function_represent";
    pub(crate) const FUNC_ACCESS: &str = "function_caller_access";
    pub(crate) const FUNC_INPUT_MODE: &str = "function_input_mode";
    pub(crate) const FUNC_IS_PRIMITIVE: &str = "function_is_primitive";
    pub(crate) const FUNC_ID: &str = "function_id";
    pub(crate) const FUNC_BODY: &str = "function_body";
    pub(crate) const FUNC_CTX: &str = "function_context";
    pub(crate) const FUNC_INPUT_NAME: &str = "function_input_name";
    pub(crate) const FUNC_CALLER_NAME: &str = "function_caller_name";

    pub(crate) const CHAIN: &str = ".";
    pub(crate) const CALL_WITH_CTX: &str = "..";

    pub(crate) const PROP_NEW: &str = "proposition";
    pub(crate) const PROP_REPR: &str = "proposition_represent";
    pub(crate) const PROP_TRUTH: &str = "proposition_truth";
    pub(crate) const PROP_FUNCTION: &str = "proposition_function";
    pub(crate) const PROP_INPUT: &str = "proposition_input";
    pub(crate) const PROP_OUTPUT: &str = "proposition_output";
    pub(crate) const PROP_CTX_BEFORE: &str = "proposition_before";
    pub(crate) const PROP_CTX_AFTER: &str = "proposition_after";

    pub(crate) const NOT: &str = "not";
    pub(crate) const AND: &str = "and";
    pub(crate) const OR: &str = "or";

    pub(crate) const INT_ADD: &str = "+";
    pub(crate) const INT_SUBTRACT: &str = "-";
    pub(crate) const INT_MULTIPLY: &str = "*";
    pub(crate) const INT_DIVIDE: &str = "/";
    pub(crate) const INT_REMAINDER: &str = "%";
    pub(crate) const INT_DIVIDE_REMAINDER: &str = "/%";
    pub(crate) const INT_LESS_THAN: &str = "<";
    pub(crate) const INT_LESS_EQUAL: &str = "<=";
    pub(crate) const INT_GREATER_THAN: &str = ">";
    pub(crate) const INT_GREATER_EQUAL: &str = ">=";
    pub(crate) const INT_LESS_GREATER: &str = "<>";

    pub(crate) const STR_LENGTH: &str = "string_length";
    pub(crate) const STR_CONCAT: &str = "string_concat";

    pub(crate) const PAIR_FIRST: &str = "get_1";
    pub(crate) const PAIR_FIRST_ASSIGN: &str = "set_1";
    pub(crate) const PAIR_SECOND: &str = "get_2";
    pub(crate) const PAIR_SECOND_ASSIGN: &str = "set_2";

    pub(crate) const LIST_LENGTH: &str = "list_length";
    pub(crate) const LIST_SET: &str = "list_set";
    pub(crate) const LIST_SET_MANY: &str = "list_set_many";
    pub(crate) const LIST_GET: &str = "list_get";
    pub(crate) const LIST_INSERT: &str = "list_insert";
    pub(crate) const LIST_INSERT_MANY: &str = "list_insert_many";
    pub(crate) const LIST_REMOVE: &str = "list_remove";
    pub(crate) const LIST_PUSH: &str = "list_push";
    pub(crate) const LIST_PUSH_MANY: &str = "list_push_many";
    pub(crate) const LIST_POP: &str = "list_pop";
    pub(crate) const LIST_CLEAR: &str = "list_clear";

    pub(crate) const MAP_LENGTH: &str = "map_length";
    pub(crate) const MAP_KEYS: &str = "map_keys";
    pub(crate) const MAP_INTO_KEYS: &str = "map_into_keys";
    pub(crate) const MAP_VALUES: &str = "map_values";
    pub(crate) const MAP_INTO_VALUES: &str = "map_into_values";
    pub(crate) const MAP_CONTAINS: &str = "map_contains";
    pub(crate) const MAP_CONTAINS_MANY: &str = "map_contains_many";
    pub(crate) const MAP_SET: &str = "map_set";
    pub(crate) const MAP_SET_MANY: &str = "map_set_many";
    pub(crate) const MAP_GET: &str = "map_get";
    pub(crate) const MAP_GET_MANY: &str = "map_get_many";
    pub(crate) const MAP_REMOVE: &str = "map_remove";
    pub(crate) const MAP_REMOVE_MANY: &str = "map_remove_many";
    pub(crate) const MAP_CLEAR: &str = "map_clear";
}

mod meta;

mod syntax;

mod value;

mod ctx;

mod ctrl;

mod eval;

mod logic;

mod func;

mod call;

mod reverse;

mod prop;

mod symbol;

mod unit;

mod bool;

mod int;

mod float;

mod bytes;

mod str;

mod pair;

mod list;

mod map;

mod utils;
