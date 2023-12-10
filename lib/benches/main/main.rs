use {
    crate::syntax::bench_syntax,
    airlang::{
        generate,
        parse,
        Interpreter,
    },
    criterion::{
        black_box,
        criterion_group,
        criterion_main,
        BatchSize,
        Criterion,
    },
};

criterion_group!(benches, bench_all);
criterion_main!(benches);

pub fn bench_all(c: &mut Criterion) {
    bench_interpret(c);
    bench_parse(c);
    bench_generate(c);

    bench_syntax(c);
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let mut interpreter = Interpreter::new();
        let s = include_str!("interpret.air");
        let src_val = parse(s).expect("parse failed");
        b.iter_batched(
            || src_val.clone(),
            |val| interpreter.interpret(black_box(val)),
            BatchSize::SmallInput,
        )
    });
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

mod syntax;
