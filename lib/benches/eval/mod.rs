use criterion::{black_box, Criterion};

use airlang::{eval, parse};

pub(crate) fn bench_eval(c: &mut Criterion) {
    c.bench_function("eval", |b| {
        let s = include_str!("src.air");
        let v = parse(s);
        b.iter(|| {
            eval(black_box(v.clone()));
        })
    });
}
