use {
    airlang::parse,
    criterion::{
        black_box,
        Criterion,
    },
};

pub(crate) fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let s = include_str!("./parse/src.air");
        b.iter(|| {
            parse(black_box(s));
        })
    });
}
