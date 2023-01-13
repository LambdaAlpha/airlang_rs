use {
    airlang::interpret,
    criterion::{
        black_box,
        Criterion,
    },
};

pub(crate) fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let s = include_str!("./interpret/src.air");
        b.iter(|| {
            interpret(black_box(s));
        })
    });
}
