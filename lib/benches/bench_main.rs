use {
    airlang::{
        interpret,
        parse,
        Repr,
    },
    criterion::{
        black_box,
        criterion_group,
        criterion_main,
        Criterion,
    },
};

criterion_group!(benches, bench_parse, bench_interpret,);
criterion_main!(benches);

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        let s = include_str!("parse.air");
        b.iter(|| {
            let _ = black_box(s).parse::<Repr>();
        })
    });
}

fn bench_interpret(c: &mut Criterion) {
    c.bench_function("interpret str to str", |b| {
        let s = include_str!("interpret.air");
        b.iter(|| {
            let _ = interpret_str(black_box(s));
        })
    });
}

fn interpret_str(src: &str) -> Option<String> {
    let src_repr = parse(src).ok()?;
    let ret_repr = interpret(&src_repr).ok()?;
    Some((&ret_repr).into())
}
