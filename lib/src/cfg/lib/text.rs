use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::CtxMutInputEvalFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::TEXT;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::Int;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct TextLib {
    pub from_utf8: PrimFuncVal,
    pub into_utf8: PrimFuncVal,
    pub get_length: PrimFuncVal,
    pub push: PrimFuncVal,
    pub join: PrimFuncVal,
}

pub const FROM_UTF8: &str = concatcp!(PREFIX_ID, TEXT, ".from_utf8");
pub const INTO_UTF8: &str = concatcp!(PREFIX_ID, TEXT, ".into_utf8");
pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, TEXT, ".get_length");
pub const PUSH: &str = concatcp!(PREFIX_ID, TEXT, ".push");
pub const JOIN: &str = concatcp!(PREFIX_ID, TEXT, ".join");

impl Default for TextLib {
    fn default() -> Self {
        TextLib {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            get_length: get_length(),
            push: push(),
            join: join(),
        }
    }
}

impl CfgMod for TextLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, FROM_UTF8, self.from_utf8);
        extend_func(cfg, INTO_UTF8, self.into_utf8);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, PUSH, self.push);
        extend_func(cfg, JOIN, self.join);
    }
}

pub fn from_utf8() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_from_utf8 }.build()
}

fn fn_from_utf8(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Byte(byte) = input else {
        return bug!(cfg, "{FROM_UTF8}: expected input to be a byte, but got {input}");
    };
    let byte = Byte::from(byte);
    let Ok(str) = String::from_utf8(byte.into()) else {
        return bug!(cfg, "{FROM_UTF8}: expected input to be a utf8 text");
    };
    Val::Text(Text::from(str).into())
}

pub fn into_utf8() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_into_utf8 }.build()
}

fn fn_into_utf8(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(text) = input else {
        return bug!(cfg, "{INTO_UTF8}: expected input to be a text, but got {input}");
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

pub fn get_length() -> PrimFuncVal {
    CtxConstInputFreeFunc { fn_: fn_get_length }.build()
}

fn fn_get_length(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Text(text) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a text, but got {ctx}");
    };
    let len: Int = text.len().into();
    Val::Int(len.into())
}

pub fn push() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_push }.build()
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Text(text) = ctx else {
        return bug!(cfg, "{PUSH}: expected context to be a text, but got {ctx}");
    };
    let Val::Text(t) = input else {
        return bug!(cfg, "{PUSH}: expected input to be a text, but got {input}");
    };
    text.push_str(&t);
    Val::default()
}

// todo design
pub fn join() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_join }.build()
}

fn fn_join(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{JOIN}: expected input to be a pair, but got {input}");
    };
    let Val::Text(separator) = &pair.left else {
        return bug!(cfg, "{JOIN}: expected input.left to be a text, but got {}", pair.left);
    };
    let Val::List(texts) = &pair.right else {
        return bug!(cfg, "{JOIN}: expected input.right to be a list, but got {}", pair.right);
    };
    let mut to_join: Vec<&str> = Vec::with_capacity(texts.len());
    for text in texts.iter() {
        let Val::Text(s) = text else {
            return bug!(cfg, "{JOIN}: expected input.right.item to be a text, but got {text}");
        };
        to_join.push(s);
    }
    let text = to_join.join(separator);
    Val::Text(Text::from(text).into())
}
