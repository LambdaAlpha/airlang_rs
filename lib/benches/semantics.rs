use {
    airlang::{
        semantics::interpret,
        syntax::parse,
    },
    criterion::{
        black_box,
        Criterion,
    },
};

pub fn bench_semantics(c: &mut Criterion) {
    bench_interpret(c);
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let s = include_str!("interpret.air");
        let src_repr = parse(s).unwrap();
        b.iter(|| interpret(black_box(&src_repr)))
    });
}
