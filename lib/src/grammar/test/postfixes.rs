use crate::val::{map, List, Val};

pub(crate) fn expected() -> Val {
    Val::from(vec![
        Val::ltree(Val::letter("a".to_owned()), vec![]),
        Val::ltree(
            Val::letter("a".to_owned()),
            vec![Val::ltree(Val::letter("a".to_owned()), vec![])],
        ),
        Val::ltree(Val::ltree(Val::letter("a".to_owned()), vec![]), vec![]),
        Val::ltree(Val::from(vec![] as List), vec![]),
        Val::mtree(Val::letter("a".to_owned()), map::from([])),
        Val::mtree(
            Val::letter("a".to_owned()),
            map::from([(
                Val::mtree(Val::letter("b".to_owned()), map::from([])),
                Val::mtree(Val::letter("c".to_owned()), map::from([])),
            )]),
        ),
        Val::mtree(
            Val::mtree(Val::letter("a".to_owned()), map::from([])),
            map::from([]),
        ),
        Val::mtree(Val::from(map::from([])), map::from([])),
        Val::mtree(
            Val::ltree(Val::letter("a".to_owned()), vec![]),
            map::from([]),
        ),
        Val::ltree(
            Val::mtree(Val::letter("a".to_owned()), map::from([])),
            vec![],
        ),
    ])
}
