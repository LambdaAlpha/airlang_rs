use crate::{
    semantics::{
        eval::{
            ctx::{
                NameMap,
                TaggedVal,
            },
            Name,
        },
        val::Val,
        Func,
    },
    types::Reader,
};

pub(crate) fn prelude() -> NameMap {
    let mut c = NameMap::default();

    put(&mut c, names::AIR_VERSION_MAJOR, meta::version_major());
    put(&mut c, names::AIR_VERSION_MINOR, meta::version_minor());
    put(&mut c, names::AIR_VERSION_PATCH, meta::version_patch());

    put(&mut c, names::READ, ctx::read());
    put(&mut c, names::IS_NULL, ctx::is_null());
    put(&mut c, names::MOVE, ctx::remove());
    put(&mut c, names::ASSIGN_LOCAL, ctx::assign_local());
    put(&mut c, names::ASSIGN, ctx::assign());
    put(&mut c, names::ASSIGN_FINAL, ctx::assign_final());
    put(&mut c, names::ASSIGN_CONST, ctx::assign_const());
    put(&mut c, names::FINAL, ctx::set_final());
    put(&mut c, names::CONST, ctx::set_const());
    put(&mut c, names::IS_FINAL, ctx::is_final());
    put(&mut c, names::IS_CONST, ctx::is_const());
    put(&mut c, names::REF, ctx::new_ref());
    put(&mut c, names::NULL_REF, ctx::null_ref());
    put(&mut c, names::FINAL_REF, ctx::final_ref());
    put(&mut c, names::CONST_REF, ctx::const_ref());
    put(&mut c, names::CTX_NEW, ctx::ctx_new());
    put(&mut c, names::CTX_SET_SUPER, ctx::ctx_set_super());

    put(&mut c, names::SEQUENCE, ctrl::sequence());
    put(&mut c, names::IF, ctrl::condition());
    put(&mut c, names::WHILE, ctrl::while_loop());

    put(&mut c, names::NOT, bool::not());
    put(&mut c, names::AND, bool::and());
    put(&mut c, names::OR, bool::or());
    put(&mut c, names::EQUAL, bool::equal());
    put(&mut c, names::NOT_EQUAL, bool::not_equal());

    put(&mut c, names::VALUE, eval::value());
    put(&mut c, names::EVAL, eval::eval());
    put(&mut c, names::EVAL_INTERPOLATE, eval::eval_interpolate());
    put(&mut c, names::EVAL_INLINE, eval::eval_inline());
    put(&mut c, names::EVAL_TWICE, eval::eval_twice());
    put(&mut c, names::EVAL_THRICE, eval::eval_thrice());
    put(&mut c, names::EVAL_FREE, eval::eval_free());
    put(&mut c, names::EVAL_IN_CTX, eval::eval_in_ctx());
    put(&mut c, names::EVAL_IN_CTX_CONST, eval::eval_in_ctx_const());
    put(&mut c, names::PARSE, eval::parse());
    put(&mut c, names::STRINGIFY, eval::stringify());
    put(&mut c, names::FUNC, eval::func());
    put(&mut c, names::CHAIN, eval::chain());

    put(&mut c, names::INT_ADD, int::add());
    put(&mut c, names::INT_SUBTRACT, int::subtract());
    put(&mut c, names::INT_MULTIPLY, int::multiply());
    put(&mut c, names::INT_DIVIDE, int::divide());
    put(&mut c, names::INT_REMAINDER, int::remainder());
    put(&mut c, names::INT_DIVIDE_REMAINDER, int::divide_remainder());
    put(&mut c, names::INT_LESS_THAN, int::less_than());
    put(&mut c, names::INT_LESS_EQUAL, int::less_equal());
    put(&mut c, names::INT_GREATER_THAN, int::greater_than());
    put(&mut c, names::INT_GREATER_EQUAL, int::greater_equal());
    put(&mut c, names::INT_LESS_GREATER, int::less_greater());

    put(&mut c, names::STR_LENGTH, str::length());
    put(&mut c, names::STR_CONCAT, str::concat());

    put(&mut c, names::PAIR_FIRST, pair::first());
    put(&mut c, names::PAIR_FIRST_ASSIGN, pair::first_assign());
    put(&mut c, names::PAIR_SECOND, pair::second());
    put(&mut c, names::PAIR_SECOND_ASSIGN, pair::second_assign());

    put(&mut c, names::LIST_LENGTH, list::length());
    put(&mut c, names::LIST_SET, list::set());
    put(&mut c, names::LIST_SET_MANY, list::set_many());
    put(&mut c, names::LIST_GET, list::get());
    put(&mut c, names::LIST_INSERT, list::insert());
    put(&mut c, names::LIST_INSERT_MANY, list::insert_many());
    put(&mut c, names::LIST_REMOVE, list::remove());
    put(&mut c, names::LIST_PUSH, list::push());
    put(&mut c, names::LIST_PUSH_MANY, list::push_many());
    put(&mut c, names::LIST_POP, list::pop());
    put(&mut c, names::LIST_CLEAR, list::clear());

    put(&mut c, names::MAP_LENGTH, map::length());
    put(&mut c, names::MAP_KEYS, map::keys());
    put(&mut c, names::MAP_VALUES, map::values());
    put(&mut c, names::MAP_CONTAINS, map::contains());
    put(&mut c, names::MAP_CONTAINS_MANY, map::contains_many());
    put(&mut c, names::MAP_SET, map::set());
    put(&mut c, names::MAP_SET_MANY, map::set_many());
    put(&mut c, names::MAP_GET, map::get());
    put(&mut c, names::MAP_GET_MANY, map::get_many());
    put(&mut c, names::MAP_REMOVE, map::remove());
    put(&mut c, names::MAP_REMOVE_MANY, map::remove_many());
    put(&mut c, names::MAP_CLEAR, map::clear());

    c
}

fn put(constants: &mut NameMap, key: &str, val: Val) {
    constants.insert(Name::from(key), TaggedVal::new_const(val));
}

fn prelude_func(func: Func) -> Val {
    Val::Func(Reader::new(func).into())
}

pub(crate) mod names {
    pub(crate) const AIR_VERSION_MAJOR: &str = "air_version_major";
    pub(crate) const AIR_VERSION_MINOR: &str = "air_version_minor";
    pub(crate) const AIR_VERSION_PATCH: &str = "air_version_patch";

    pub(crate) const READ: &str = "read";
    pub(crate) const IS_NULL: &str = "is_null";
    pub(crate) const MOVE: &str = "move";
    pub(crate) const ASSIGN_LOCAL: &str = "=local";
    pub(crate) const ASSIGN: &str = "=";
    pub(crate) const ASSIGN_FINAL: &str = "=final";
    pub(crate) const ASSIGN_CONST: &str = "=const";
    pub(crate) const FINAL: &str = "final";
    pub(crate) const CONST: &str = "const";
    pub(crate) const IS_FINAL: &str = "is_final";
    pub(crate) const IS_CONST: &str = "is_const";
    pub(crate) const REF: &str = "ref";
    pub(crate) const NULL_REF: &str = "null_ref";
    pub(crate) const FINAL_REF: &str = "final_ref";
    pub(crate) const CONST_REF: &str = "const_ref";
    pub(crate) const CTX_NEW: &str = "context";
    pub(crate) const CTX_SET_SUPER: &str = "set_super";

    pub(crate) const SEQUENCE: &str = ";";
    pub(crate) const IF: &str = "if";
    pub(crate) const WHILE: &str = "while";

    pub(crate) const VALUE: &str = "value";
    pub(crate) const EVAL: &str = "eval";
    pub(crate) const EVAL_INTERPOLATE: &str = "interpolate";
    pub(crate) const EVAL_INLINE: &str = "inline";
    pub(crate) const EVAL_TWICE: &str = "eval2";
    pub(crate) const EVAL_THRICE: &str = "eval3";
    pub(crate) const EVAL_FREE: &str = "eval_free";
    pub(crate) const EVAL_IN_CTX: &str = "..";
    pub(crate) const EVAL_IN_CTX_CONST: &str = "..const";
    pub(crate) const PARSE: &str = "parse";
    pub(crate) const STRINGIFY: &str = "stringify";
    pub(crate) const FUNC: &str = "function";
    pub(crate) const CHAIN: &str = ".";

    pub(crate) const NOT: &str = "not";
    pub(crate) const AND: &str = "and";
    pub(crate) const OR: &str = "or";
    pub(crate) const EQUAL: &str = "==";
    pub(crate) const NOT_EQUAL: &str = "!=";

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

    pub(crate) const STR_LENGTH: &str = "str_length";
    pub(crate) const STR_CONCAT: &str = "str_concat";

    pub(crate) const PAIR_FIRST: &str = "get1";
    pub(crate) const PAIR_FIRST_ASSIGN: &str = "set1";
    pub(crate) const PAIR_SECOND: &str = "get2";
    pub(crate) const PAIR_SECOND_ASSIGN: &str = "set2";

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
    pub(crate) const MAP_VALUES: &str = "map_values";
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

mod ctx;

mod ctrl;

mod eval;

mod bool;

mod int;

mod str;

mod pair;

mod list;

mod map;
