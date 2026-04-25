use std::hint::black_box;

use airlang::cfg::comp::BaseCompCfg;
use airlang::cfg::prelude;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang_dev::bench::BatchSize;
use airlang_dev::bench::Criterion;
use airlang_dev::bench::criterion_group;
use airlang_dev::bench::criterion_main;
use airlang_dev::log::init_logger;

criterion_group!(benches, bench_all);
criterion_main!(benches);

pub fn bench_all(c: &mut Criterion) {
    init_logger();
    bench_interpret(c);
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let mut cfg = BaseCompCfg::generate();
        let mut ctx = prelude(&mut cfg);
        let s = include_str!("interpret.air");
        let src_val: Val = s.parse().expect("parse failed");
        b.iter_batched(
            || src_val.clone(),
            |val| Eval.call(&mut cfg, &mut ctx, black_box(val)),
            BatchSize::SmallInput,
        );
    });
}
