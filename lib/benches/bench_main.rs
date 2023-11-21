use {
    crate::{
        semantics::bench_semantics,
        syntax::bench_syntax,
    },
    criterion::{
        criterion_group,
        criterion_main,
        Criterion,
    },
};

pub mod semantics;
pub mod syntax;

criterion_group!(benches, bench_all);
criterion_main!(benches);

pub fn bench_all(c: &mut Criterion) {
    bench_syntax(c);
    bench_semantics(c);
}
