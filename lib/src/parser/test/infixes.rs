use crate::val::Val;

pub fn expected() -> Val {
    Val::list(vec![
        Val::ltree1(
            Val::bytes("&".as_bytes().to_vec()),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
        ),
        Val::ltree1(
            Val::bytes("*".as_bytes().to_vec()),
            vec![
                Val::ltree1(
                    Val::bytes("&".as_bytes().to_vec()),
                    vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
                ),
                Val::bytes(vec![0x03]),
            ],
        ),
        Val::ltree1(
            Val::bytes("&".as_bytes().to_vec()),
            vec![Val::bytes(vec![0x01]), Val::list(vec![])],
        ),
        Val::ltree1(
            Val::bytes("&".as_bytes().to_vec()),
            vec![Val::list(vec![]), Val::bytes(vec![0x01])],
        ),
        Val::ltree1(
            Val::bytes("&".as_bytes().to_vec()),
            vec![
                Val::ltree1(Val::bytes(vec![0x01]), vec![]),
                Val::ltree1(Val::bytes(vec![0x02]), vec![]),
            ],
        ),
        Val::ltree1(
            Val::bytes("a".as_bytes().to_vec()),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
        ),
        Val::ltree1(
            Val::bytes("b".as_bytes().to_vec()),
            vec![
                Val::ltree1(
                    Val::bytes("a".as_bytes().to_vec()),
                    vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
                ),
                Val::bytes(vec![0x03]),
            ],
        ),
        Val::ltree1(
            Val::bytes("a".as_bytes().to_vec()),
            vec![Val::bytes(vec![0x01]), Val::list(vec![])],
        ),
        Val::ltree1(
            Val::bytes("a".as_bytes().to_vec()),
            vec![Val::list(vec![]), Val::bytes(vec![0x01])],
        ),
        Val::ltree1(
            Val::bytes("a".as_bytes().to_vec()),
            vec![
                Val::ltree1(Val::bytes(vec![0x01]), vec![]),
                Val::ltree1(Val::bytes(vec![0x02]), vec![]),
            ],
        ),
        Val::ltree1(
            Val::list(vec![]),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
        ),
        Val::ltree1(
            Val::bytes(vec![0x02]),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x03])],
        ),
        Val::ltree1(
            Val::ltree1(
                Val::bytes(vec![0x03]),
                vec![Val::bytes(vec![0x02]), Val::bytes(vec![0x04])],
            ),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x05])],
        ),
        Val::ltree1(
            Val::bytes("%".as_bytes().to_vec()),
            vec![
                Val::ltree1(
                    Val::bytes("&".as_bytes().to_vec()),
                    vec![
                        Val::bytes(vec![0x01]),
                        Val::ltree1(
                            Val::bytes("*".as_bytes().to_vec()),
                            vec![Val::bytes(vec![0x02]), Val::bytes(vec![0x03])],
                        ),
                    ],
                ),
                Val::bytes(vec![0x04]),
            ],
        ),
        Val::ltree1(
            Val::bytes(vec![0x02]),
            vec![
                Val::ltree1(
                    Val::bytes(vec![0x01]),
                    vec![
                        Val::bytes("+".as_bytes().to_vec()),
                        Val::bytes("-".as_bytes().to_vec()),
                    ],
                ),
                Val::bytes("*".as_bytes().to_vec()),
            ],
        ),
        Val::bytes("a".as_bytes().to_vec()),
        Val::bytes("&".as_bytes().to_vec()),
        Val::ltree1(
            Val::ltree1(
                Val::bytes("&".as_bytes().to_vec()),
                vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
            ),
            vec![],
        ),
        Val::ltree1(Val::bytes("a".as_bytes().to_vec()), vec![]),
        Val::list(vec![]),
        Val::ltree1(
            Val::bytes("&".as_bytes().to_vec()),
            vec![Val::bytes(vec![0x01]), Val::bytes(vec![0x02])],
        ),
    ])
}
