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
        let src_val = parse(s).expect("parse failed");
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
        let repr = parse(s).expect("parse failed");
        b.iter(|| generate(black_box(&repr)))
    });
}

#[cfg(test)]
pub mod test {
    pub use {
        airlang::{
            semantics::{
                generate,
                parse,
                Interpreter,
                Val,
            },
            types::Int,
        },
        std::error::Error,
    };

    #[test]
    pub fn test_interpret() -> Result<(), Box<dyn Error>> {
        let mut interpreter = Interpreter::new();
        let s = include_str!("interpret.air");
        let src_val = parse(s)?;
        let output = interpreter.interpret(src_val);
        let expected = Val::Int(Int::from(6));
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    pub fn test_parse() -> Result<(), Box<dyn Error>> {
        let s = include_str!("parse.air");
        parse(s)?;
        Ok(())
    }

    #[test]
    pub fn test_generate() -> Result<(), Box<dyn Error>> {
        let s = include_str!("generate.air");
        let repr = parse(s)?;
        let str = generate(&repr)?;
        let new_repr = parse(&str)?;
        assert_eq!(repr, new_repr);
        Ok(())
    }
}
