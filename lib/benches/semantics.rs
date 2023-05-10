use {
    airlang::semantics::{
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
}

fn bench_interpret(c: &mut Criterion) {
    let mut interpreter = Interpreter::new();
    c.bench_function("interpret", |b| {
        let s = include_str!("interpret.air");
        let src_val = parse(s).unwrap();
        b.iter_batched(
            || src_val.clone(),
            |val| interpreter.interpret(black_box(val)),
            BatchSize::SmallInput,
        )
    });
}
