use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::map(Map::from([])),
        Val::map(Map::from([(
            Val::bytes(vec![0x01]),
            Val::bytes(vec![0x02]),
        )])),
        Val::map(Map::from([(
            Val::bytes(vec![0x01]),
            Val::bytes(vec![0x02]),
        )])),
        Val::map(Map::from([
            (Val::bytes(vec![0x01]), Val::bytes(vec![0x02])),
            (Val::bytes(vec![0x03]), Val::bytes(vec![0x04])),
        ])),
        Val::map(Map::from([
            (Val::bytes(vec![0x01]), Val::bytes(vec![0x02])),
            (Val::bytes(vec![0x03]), Val::bytes(vec![0x04])),
        ])),
        Val::map(Map::from([(
            Val::map(Map::from([])),
            Val::map(Map::from([])),
        )])),
        Val::map(Map::from([(
            Val::map(Map::from([(
                Val::bytes(vec![0x01]),
                Val::bytes(vec![0x02]),
            )])),
            Val::map(Map::from([(
                Val::bytes(vec![0x03]),
                Val::bytes(vec![0x04]),
            )])),
        )])),
    ])
}
