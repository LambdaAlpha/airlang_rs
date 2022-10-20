use crate::grammar::repr::{List, map, Repr};

pub(crate) fn expected() -> Repr {
    Repr::from(vec![
        Repr::ltree(Repr::letter("a".to_owned()), vec![]),
        Repr::ltree(
            Repr::letter("a".to_owned()),
            vec![Repr::ltree(Repr::letter("a".to_owned()), vec![])],
        ),
        Repr::ltree(Repr::ltree(Repr::letter("a".to_owned()), vec![]), vec![]),
        Repr::ltree(Repr::from(vec![] as List), vec![]),
        Repr::mtree(Repr::letter("a".to_owned()), map::from([])),
        Repr::mtree(
            Repr::letter("a".to_owned()),
            map::from([(
                Repr::mtree(Repr::letter("b".to_owned()), map::from([])),
                Repr::mtree(Repr::letter("c".to_owned()), map::from([])),
            )]),
        ),
        Repr::mtree(
            Repr::mtree(Repr::letter("a".to_owned()), map::from([])),
            map::from([]),
        ),
        Repr::mtree(Repr::from(map::from([])), map::from([])),
        Repr::mtree(
            Repr::ltree(Repr::letter("a".to_owned()), vec![]),
            map::from([]),
        ),
        Repr::ltree(
            Repr::mtree(Repr::letter("a".to_owned()), map::from([])),
            vec![],
        ),
    ])
}
