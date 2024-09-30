use airlang::syntax::{
    generate_compact,
    parse,
};
use criterion::{
    Criterion,
    black_box,
};

pub fn bench_syntax(c: &mut Criterion) {
    bench_parse(c);
    bench_generate(c);
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("repr-parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| parse(black_box(s)));
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("repr-generate", |b| {
        let s = include_str!("generate.air");
        let repr = parse(s).expect("parse failed");
        b.iter(|| generate_compact(black_box(&repr)));
    });
}
