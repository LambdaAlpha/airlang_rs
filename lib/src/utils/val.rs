use crate::MapVal;
use crate::Symbol;
use crate::Val;

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn symbol(s: &str) -> Val {
    Val::Symbol(Symbol::from_str(s))
}
