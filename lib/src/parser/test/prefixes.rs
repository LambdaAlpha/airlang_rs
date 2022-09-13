use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::ltree1(Val::bytes("a".as_bytes().to_vec()), vec![]),
        Val::ltree1(
            Val::bytes("a".as_bytes().to_vec()),
            vec![Val::ltree1(Val::bytes("a".as_bytes().to_vec()), vec![])],
        ),
        Val::ltree1(
            Val::ltree1(Val::bytes("a".as_bytes().to_vec()), vec![]),
            vec![],
        ),
        Val::ltree1(Val::list(vec![]), vec![]),
        Val::mtree1(Val::bytes("a".as_bytes().to_vec()), Map::from([])),
        Val::mtree1(
            Val::bytes("a".as_bytes().to_vec()),
            Map::from([(
                Val::mtree1(Val::bytes("b".as_bytes().to_vec()), Map::from([])),
                Val::mtree1(Val::bytes("c".as_bytes().to_vec()), Map::from([])),
            )]),
        ),
        Val::mtree1(
            Val::mtree1(Val::bytes("a".as_bytes().to_vec()), Map::from([])),
            Map::from([]),
        ),
        Val::mtree1(Val::map(Map::from([])), Map::from([])),
        Val::mtree1(
            Val::ltree1(Val::bytes("a".as_bytes().to_vec()), vec![]),
            Map::from([]),
        ),
        Val::ltree1(
            Val::mtree1(Val::bytes("a".as_bytes().to_vec()), Map::from([])),
            vec![],
        ),
    ])
}
