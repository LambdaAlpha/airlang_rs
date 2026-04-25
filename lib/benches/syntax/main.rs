use std::hint::black_box;

use airlang::semantics::val::Val;
use airlang::syntax::repr::Repr;
use airlang_dev::bench::Criterion;
use airlang_dev::bench::criterion_group;
use airlang_dev::bench::criterion_main;
use airlang_dev::log::init_logger;

criterion_group!(benches, bench_all);
criterion_main!(benches);

pub fn bench_all(c: &mut Criterion) {
    init_logger();
    bench_parse_repr(c);
    bench_parse_val(c);
    bench_generate_repr(c);
    bench_generate_val(c);
}

fn bench_parse_repr(c: &mut Criterion) {
    c.bench_function("parse-repr", |b| {
        let s = include_str!("parse.air");
        b.iter(|| black_box(s).parse::<Repr>());
    });
}

fn bench_parse_val(c: &mut Criterion) {
    c.bench_function("parse-val", |b| {
        let s = include_str!("parse.air");
        b.iter(|| black_box(s).parse::<Val>());
    });
}

fn bench_generate_repr(c: &mut Criterion) {
    c.bench_function("generate-repr", |b| {
        let s = include_str!("generate.air");
        let repr: Repr = s.parse().expect("parse failed");
        b.iter(|| format!("{}", black_box(&repr)));
    });
}

fn bench_generate_val(c: &mut Criterion) {
    c.bench_function("generate-val", |b| {
        let s = include_str!("generate.air");
        let repr: Val = s.parse().expect("parse failed");
        b.iter(|| {
            let _ = format!("{:#}", black_box(&repr));
        });
    });
}
