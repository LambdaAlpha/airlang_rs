use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::list(vec![]),
        Val::list(vec![Val::bytes(vec![0x02]), Val::bytes(vec![0x05])]),
        Val::map(Map::from([])),
        Val::map(Map::from([
            (Val::bytes(vec![0x02]), Val::bytes(vec![0x05])),
            (Val::bytes(vec![0x08]), Val::bytes(vec![0x09])),
        ])),
        Val::ltree1(Val::bytes("b".as_bytes().to_vec()), vec![]),
        Val::ltree1(
            Val::bytes("e".as_bytes().to_vec()),
            vec![
                Val::bytes("b".as_bytes().to_vec()),
                Val::bytes("g".as_bytes().to_vec()),
            ],
        ),
    ])
}
