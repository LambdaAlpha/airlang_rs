use std::hint::black_box;

use airlang::syntax::generate_compact;
use airlang::syntax::parse;
use airlang::syntax::repr::Repr;
use criterion::Criterion;

pub fn bench_syntax(c: &mut Criterion) {
    bench_parse(c);
    bench_generate(c);
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("repr-parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| parse::<Repr>(black_box(s)));
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("repr-generate", |b| {
        let s = include_str!("generate.air");
        let repr: Repr = parse(s).expect("parse failed");
        b.iter(|| generate_compact(black_box((&repr).try_into().unwrap())));
    });
}
