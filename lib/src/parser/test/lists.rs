use crate::val::Val;

pub fn expected() -> Val {
    Val::list(vec![
        Val::list(vec![]),
        Val::list(vec![Val::bytes(vec![0x01])]),
        Val::list(vec![Val::bytes(vec![0x01])]),
        Val::list(vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])]),
        Val::list(vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])]),
        Val::list(vec![Val::list(vec![])]),
        Val::list(vec![Val::list(vec![]), Val::list(vec![])]),
    ])
}
