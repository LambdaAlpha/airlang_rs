use {
    airlang::semantics::{
        generate,
        parse,
        Interpreter,
    },
    criterion::{
        black_box,
        BatchSize,
        Criterion,
    },
};

pub fn bench_semantics(c: &mut Criterion) {
    bench_interpret(c);
    bench_parse(c);
    bench_generate(c);
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret", |b| {
        let mut interpreter = Interpreter::new();
        let s = include_str!("interpret.air");
        let src_val = parse(s).unwrap();
        b.iter_batched(
            || src_val.clone(),
            |val| interpreter.interpret(black_box(val)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("semantic parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| parse(black_box(s)))
    });
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("semantic generate", |b| {
        let s = include_str!("generate.air");
        let repr = parse(s).unwrap();
        b.iter(|| generate(black_box(&repr)))
    });
}
