use const_format::concatcp;
use log::error;

use super::ConstImpl;
use super::FreeImpl;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::KEY;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct KeyLib {
    pub from_text: FreePrimFuncVal,
    pub into_text: FreePrimFuncVal,
    pub get_length: ConstPrimFuncVal,
    pub join: FreePrimFuncVal,
}

pub const FROM_TEXT: &str = concatcp!(PREFIX_ID, KEY, ".from_text");
pub const INTO_TEXT: &str = concatcp!(PREFIX_ID, KEY, ".into_text");
pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, KEY, ".get_length");
pub const JOIN: &str = concatcp!(PREFIX_ID, KEY, ".join");

impl Default for KeyLib {
    fn default() -> Self {
        KeyLib {
            from_text: from_text(),
            into_text: into_text(),
            get_length: get_length(),
            join: join(),
        }
    }
}

impl CfgMod for KeyLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, FROM_TEXT, self.from_text);
        extend_func(cfg, INTO_TEXT, self.into_text);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, JOIN, self.join);
    }
}

pub fn from_text() -> FreePrimFuncVal {
    FreeImpl { free: fn_from_text }.build()
}

fn fn_from_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    let is_key = t.chars().all(Key::is_key);
    if !is_key {
        error!("every character of input {t:?} text should be a key");
        return illegal_input(cfg);
    }
    let key = Key::from_string_unchecked(t.to_string());
    Val::Key(key)
}

pub fn into_text() -> FreePrimFuncVal {
    FreeImpl { free: fn_into_text }.build()
}

fn fn_into_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(key) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    Val::Text(Text::from(String::from(key)).into())
}

pub fn get_length() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_LENGTH), const_: fn_get_length }.build()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Key(key) = &*ctx else {
        error!("ctx {ctx:?} should be a key");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let len: Int = key.len().into();
    Val::Int(len.into())
}

// todo design
pub fn join() -> FreePrimFuncVal {
    FreeImpl { free: fn_join }.build()
}

fn fn_join(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let Val::Key(separator) = &pair.left else {
        error!("separator {:?} should be a key", pair.left);
        return illegal_input(cfg);
    };
    let Val::List(keys) = &pair.right else {
        error!("input.right {:?} should be a list", pair.right);
        return illegal_input(cfg);
    };
    let keys: Option<Vec<&str>> = keys
        .iter()
        .map(|v| {
            let Val::Key(s) = v else {
                error!("item {v:?} should be a key");
                return None;
            };
            let key: &str = s;
            Some(key)
        })
        .collect();
    let Some(keys) = keys else {
        return illegal_input(cfg);
    };
    let key = keys.join(separator);
    Val::Key(Key::from_string_unchecked(key))
}
