use crate::val::{Map, Val};

pub fn expected() -> Val {
    Val::list(vec![
        Val::ltree1(Val::letter("a".to_owned()), vec![]),
        Val::ltree1(
            Val::letter("a".to_owned()),
            vec![Val::ltree1(Val::letter("a".to_owned()), vec![])],
        ),
        Val::ltree1(Val::ltree1(Val::letter("a".to_owned()), vec![]), vec![]),
        Val::ltree1(Val::list(vec![]), vec![]),
        Val::mtree1(Val::letter("a".to_owned()), Map::from([])),
        Val::mtree1(
            Val::letter("a".to_owned()),
            Map::from([(
                Val::mtree1(Val::letter("b".to_owned()), Map::from([])),
                Val::mtree1(Val::letter("c".to_owned()), Map::from([])),
            )]),
        ),
        Val::mtree1(
            Val::mtree1(Val::letter("a".to_owned()), Map::from([])),
            Map::from([]),
        ),
        Val::mtree1(Val::map(Map::from([])), Map::from([])),
        Val::mtree1(
            Val::ltree1(Val::letter("a".to_owned()), vec![]),
            Map::from([]),
        ),
        Val::ltree1(
            Val::mtree1(Val::letter("a".to_owned()), Map::from([])),
            vec![],
        ),
    ])
}
