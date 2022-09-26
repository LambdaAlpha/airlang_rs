use crate::val::{List, Map, Val};

pub(crate) fn expected() -> Val {
    Val::from(vec![
        Val::ltree(Val::letter("a".to_owned()), vec![]),
        Val::ltree(
            Val::letter("a".to_owned()),
            vec![Val::ltree(Val::letter("a".to_owned()), vec![])],
        ),
        Val::ltree(Val::ltree(Val::letter("a".to_owned()), vec![]), vec![]),
        Val::ltree(Val::from(vec![] as List), vec![]),
        Val::mtree(Val::letter("a".to_owned()), Map::from([])),
        Val::mtree(
            Val::letter("a".to_owned()),
            Map::from([(
                Val::mtree(Val::letter("b".to_owned()), Map::from([])),
                Val::mtree(Val::letter("c".to_owned()), Map::from([])),
            )]),
        ),
        Val::mtree(
            Val::mtree(Val::letter("a".to_owned()), Map::from([])),
            Map::from([]),
        ),
        Val::mtree(Val::from(Map::from([])), Map::from([])),
        Val::mtree(
            Val::ltree(Val::letter("a".to_owned()), vec![]),
            Map::from([]),
        ),
        Val::ltree(
            Val::mtree(Val::letter("a".to_owned()), Map::from([])),
            vec![],
        ),
    ])
}
