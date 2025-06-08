use super::val::MapVal;
use super::val::Val;
use crate::type_::Symbol;

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str_unchecked(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn symbol(s: &str) -> Val {
    Val::Symbol(Symbol::from_str_unchecked(s))
}
