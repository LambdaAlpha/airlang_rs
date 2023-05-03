use crate::{
    semantics::{
        eval::{
            Name,
            NameMap,
        },
        val::Val,
    },
    types::Reader,
};

pub(crate) fn prelude() -> Reader<NameMap> {
    let mut c = NameMap::default();

    put(&mut c, names::AIR_VERSION, meta::version());
    put(&mut c, names::AIR_VERSION_MAJOR, meta::version_major());
    put(&mut c, names::AIR_VERSION_MINOR, meta::version_minor());
    put(&mut c, names::AIR_VERSION_PATCH, meta::version_patch());

    put(&mut c, names::ASSIGN, ctx::assign());
    put(&mut c, names::MOVE, ctx::remove());

    put(&mut c, names::SEQUENCE, ctrl::sequence());
    put(&mut c, names::IF, ctrl::condition());
    put(&mut c, names::WHILE, ctrl::while_loop());

    put(&mut c, names::NOT, bool::not());
    put(&mut c, names::AND, bool::and());
    put(&mut c, names::OR, bool::or());
    put(&mut c, names::EQUAL, bool::equal());
    put(&mut c, names::NOT_EQUAL, bool::not_equal());

    put(&mut c, names::EVAL, eval::eval());
    put(&mut c, names::VAL, eval::val());
    put(&mut c, names::PARSE, eval::parse());
    put(&mut c, names::STRINGIFY, eval::stringify());
    put(&mut c, names::FUNC, eval::func());
    put(&mut c, names::CHAIN, eval::chain());

    put(&mut c, names::INTO_KEEPER, refer::into_keeper());
    put(&mut c, names::INTO_READER, refer::into_reader());
    put(&mut c, names::INTO_OWNER, refer::into_owner());
    put(&mut c, names::SHARE_KEEPER, refer::share_keeper());
    put(&mut c, names::SHARE_READER, refer::share_reader());
    put(&mut c, names::SHARE_OWNER, refer::share_owner());
    put(&mut c, names::FROM_READER, refer::from_reader());
    put(&mut c, names::ASSIGN_OWNER, refer::assign_owner());
    put(&mut c, names::FROM_OWNER, refer::from_owner());

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

    Reader::new(c)
}

fn put(constants: &mut NameMap, key: &str, val: Val) {
    constants.insert(Name::from(key), val);
}

pub(crate) mod names {
    pub(crate) const AIR_VERSION: &str = "air_version";
    pub(crate) const AIR_VERSION_MAJOR: &str = "air_version_major";
    pub(crate) const AIR_VERSION_MINOR: &str = "air_version_minor";
    pub(crate) const AIR_VERSION_PATCH: &str = "air_version_patch";

    pub(crate) const ASSIGN: &str = "=";
    pub(crate) const MOVE: &str = "move";

    pub(crate) const SEQUENCE: &str = ";";
    pub(crate) const IF: &str = "if";
    pub(crate) const WHILE: &str = "while";

    pub(crate) const NOT: &str = "not";
    pub(crate) const AND: &str = "and";
    pub(crate) const OR: &str = "or";
    pub(crate) const EQUAL: &str = "==";
    pub(crate) const NOT_EQUAL: &str = "!=";

    pub(crate) const EVAL: &str = "eval";
    pub(crate) const VAL: &str = "val";
    pub(crate) const PARSE: &str = "parse";
    pub(crate) const STRINGIFY: &str = "stringify";
    pub(crate) const FUNC: &str = "func";
    pub(crate) const CHAIN: &str = ".";

    pub(crate) const INTO_KEEPER: &str = ">k";
    pub(crate) const INTO_READER: &str = ">r";
    pub(crate) const INTO_OWNER: &str = ">o";
    pub(crate) const SHARE_KEEPER: &str = "^k";
    pub(crate) const SHARE_READER: &str = "^r";
    pub(crate) const SHARE_OWNER: &str = "^o";
    pub(crate) const FROM_READER: &str = "<r";
    pub(crate) const FROM_OWNER: &str = "<o";
    pub(crate) const ASSIGN_OWNER: &str = "=o";

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
}

mod meta;

mod eval;

mod ctx;

mod ctrl;

mod bool;

mod refer;

mod int;
