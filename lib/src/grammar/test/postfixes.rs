use crate::grammar::repr::Repr;

pub(crate) fn expected() -> Repr {
    Repr::list(vec![
        Repr::ltree(Repr::letter("a".to_owned()), vec![]),
        Repr::ltree(
            Repr::letter("a".to_owned()),
            vec![Repr::ltree(Repr::letter("a".to_owned()), vec![])],
        ),
        Repr::ltree(Repr::ltree(Repr::letter("a".to_owned()), vec![]), vec![]),
        Repr::ltree(Repr::list(vec![]), vec![]),
        Repr::mtree(Repr::letter("a".to_owned()), vec![]),
        Repr::mtree(
            Repr::letter("a".to_owned()),
            vec![(
                Repr::mtree(Repr::letter("b".to_owned()), vec![]),
                Repr::mtree(Repr::letter("c".to_owned()), vec![]),
            )],
        ),
        Repr::mtree(
            Repr::mtree(Repr::letter("a".to_owned()), vec![]),
            vec![],
        ),
        Repr::mtree(Repr::map(vec![]), vec![]),
        Repr::mtree(
            Repr::ltree(Repr::letter("a".to_owned()), vec![]),
            vec![],
        ),
        Repr::ltree(
            Repr::mtree(Repr::letter("a".to_owned()), vec![]),
            vec![],
        ),
    ])
}
