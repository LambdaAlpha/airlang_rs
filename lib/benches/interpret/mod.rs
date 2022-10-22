use criterion::{black_box, Criterion};

use airlang::interpret;

pub(crate) fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let s = include_str!("src.air");
        b.iter(|| {
            interpret(black_box(s));
        })
    });
}
