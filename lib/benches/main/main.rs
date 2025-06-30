use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use crate::semantics::bench_semantics;
use crate::syntax::bench_syntax;

criterion_group!(benches, bench_all);
criterion_main!(benches);

pub fn bench_all(c: &mut Criterion) {
    bench_syntax(c);
    bench_semantics(c);
}

mod syntax;

mod semantics;
