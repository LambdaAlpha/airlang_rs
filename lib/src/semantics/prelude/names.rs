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
pub(crate) const HAS_META: &str = "has_meta";
pub(crate) const SET_META: &str = "set_meta";
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
