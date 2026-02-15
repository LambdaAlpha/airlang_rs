use std::hint::black_box;

use airlang::cfg::CoreCfg;
use airlang::cfg2::CoreCfg2;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::syntax::parse;
use criterion::BatchSize;
use criterion::Criterion;

pub fn bench_semantics(c: &mut Criterion) {
    bench_interpret(c);
    bench_parse(c);
    bench_generate(c);
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let mut cfg = CoreCfg2::generate();
        let mut ctx = CoreCfg::prelude(&mut cfg, "bench_interpret").unwrap();
        let s = include_str!("interpret.air");
        let src_val: Val = parse(s).expect("parse failed");
        b.iter_batched(
            || src_val.clone(),
            |val| Eval.call(&mut cfg, &mut ctx, black_box(val)),
            BatchSize::SmallInput,
        );
    });
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| parse::<Val>(black_box(s)));
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        let s = include_str!("generate.air");
        let repr: Val = parse(s).expect("parse failed");
        b.iter(|| {
            let _ = format!("{:#}", black_box(&repr));
        });
    });
}
