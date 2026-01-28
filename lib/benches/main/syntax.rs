use std::hint::black_box;

use airlang::syntax::repr::Repr;
use criterion::Criterion;

pub fn bench_syntax(c: &mut Criterion) {
    bench_parse(c);
    bench_generate(c);
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("repr-parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| black_box(s).parse::<Repr>());
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("repr-generate", |b| {
        let s = include_str!("generate.air");
        let repr: Repr = s.parse().expect("parse failed");
        b.iter(|| format!("{}", black_box(&repr)));
    });
}
