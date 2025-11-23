use crate::semantics::val::MapVal;
use crate::semantics::val::Val;
use crate::type_::Key;

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Key(Key::from_str_unchecked(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn key(s: &str) -> Val {
    Val::Key(Key::from_str_unchecked(s))
}
