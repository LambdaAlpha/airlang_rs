use {
    airlang::{
        eval,
        parse,
    },
    criterion::{
        black_box,
        Criterion,
    },
};

pub(crate) fn bench_eval(c: &mut Criterion) {
    c.bench_function("eval", |b| {
        let s = include_str!("./eval/src.air");
        let v = parse(s);
        b.iter(|| {
            eval(black_box(v.clone()));
        })
    });
}
