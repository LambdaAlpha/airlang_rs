use crate::{
    semantics::{
        eval::{
            Name,
            NameMap,
        },
        val::Val,
    },
    types::ImRef,
};

pub(crate) fn prelude() -> ImRef<NameMap> {
    let mut c = NameMap::default();

    put(&mut c, names::AIR_VERSION_CODE, meta::version_code());
    put(&mut c, names::AIR_VERSION_NAME, meta::version_name());

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

    put(&mut c, names::NEW_BOX, refer::new_box());
    put(&mut c, names::NEW_IM, refer::new_im());
    put(&mut c, names::NEW_MUT, refer::new_mut());
    put(&mut c, names::REF_BOX, refer::ref_box());
    put(&mut c, names::REF_IM, refer::ref_im());
    put(&mut c, names::REF_MUT, refer::ref_mut());
    put(&mut c, names::DEREF_IM, refer::deref_im());
    put(&mut c, names::DEREF_MUT, refer::deref_mut());

    ImRef::new(c)
}

fn put(constants: &mut NameMap, key: &str, val: Val) {
    constants.insert(Name::from(key), val);
}

pub(crate) mod names {
    pub(crate) const AIR_VERSION_CODE: &str = "air_version_code";
    pub(crate) const AIR_VERSION_NAME: &str = "air_version_name";

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

    pub(crate) const NEW_BOX: &str = "box";
    pub(crate) const NEW_IM: &str = "im";
    pub(crate) const NEW_MUT: &str = "mut";
    pub(crate) const REF_BOX: &str = "&box";
    pub(crate) const REF_IM: &str = "&im";
    pub(crate) const REF_MUT: &str = "&mut";
    pub(crate) const DEREF_IM: &str = "@";
    pub(crate) const DEREF_MUT: &str = "=mut";
}

mod meta;

mod eval;

mod ctx;

mod ctrl;

mod bool;

mod refer;
