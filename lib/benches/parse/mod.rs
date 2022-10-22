use criterion::{black_box, Criterion};

use airlang::parse;

pub(crate) fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let s = include_str!("src.air");
        b.iter(|| {
            parse(black_box(s));
        })
    });
}
