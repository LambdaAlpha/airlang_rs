mod eval;
mod interpret;
mod parse;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    interpret::bench_interpret,
    parse::bench_parse,
    eval::bench_eval,
);
criterion_main!(benches);
