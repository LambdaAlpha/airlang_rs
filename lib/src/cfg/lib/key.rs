use std::ops::Deref;

use const_format::concatcp;

use super::ConstImpl;
use super::FreeImpl;
use super::abort_free;
use crate::bug;
use crate::cfg::CfgMod;
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
        return bug!(cfg, "{FROM_TEXT}: expected input to be a text, but got {input:?}");
    };
    let is_key = t.chars().all(Key::is_key);
    if !is_key {
        return bug!(
            cfg,
            "{FROM_TEXT}: expected every character of input text should be a key, but got {t:?}"
        );
    }
    let key = Key::from_string_unchecked(t.to_string());
    Val::Key(key)
}

pub fn into_text() -> FreePrimFuncVal {
    FreeImpl { free: fn_into_text }.build()
}

fn fn_into_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(key) = input else {
        return bug!(cfg, "{INTO_TEXT}: expected input to be a key, but got {input:?}");
    };
    Val::Text(Text::from(String::from(key)).into())
}

pub fn get_length() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_LENGTH), const_: fn_get_length }.build()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Key(key) = &*ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a key, but got {:?}", ctx.deref());
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_LENGTH}: expected input to be a unit, but got {input:?}");
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
        return bug!(cfg, "{JOIN}: expected input to be a pair, but got {input:?}");
    };
    let Val::Key(separator) = &pair.left else {
        return bug!(cfg, "{JOIN}: expected input.left to be a key, but got {:?}", pair.left);
    };
    let Val::List(keys) = &pair.right else {
        return bug!(cfg, "{JOIN}: expected input.right to be a list, but got {:?}", pair.right);
    };
    let mut to_join: Vec<&str> = Vec::with_capacity(keys.len());
    for key in keys.iter() {
        let Val::Key(s) = key else {
            return bug!(cfg, "{JOIN}: expected input.right.item to be a key, but got {key:?}");
        };
        to_join.push(s);
    }
    let key = to_join.join(separator);
    Val::Key(Key::from_string_unchecked(key))
}
