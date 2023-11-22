use {
    airlang::syntax::{
        generate,
        parse,
    },
    criterion::{
        black_box,
        Criterion,
    },
};

pub fn bench_syntax(c: &mut Criterion) {
    bench_parse(c);
    bench_generate(c);
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| parse(black_box(s)))
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        let s = include_str!("generate.air");
        let repr = parse(s).expect("parse failed");
        b.iter(|| generate(black_box(&repr)))
    });
}
