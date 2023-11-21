use {
    airlang::syntax::{
        generate,
        parse,
    },
    criterion::{
        black_box,
        Criterion,
    },
};

pub fn bench_syntax(c: &mut Criterion) {
    bench_parse(c);
    bench_generate(c);
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

#[cfg(test)]
pub mod test {
    pub use {
        airlang::syntax::{
            generate,
            parse,
        },
        std::error::Error,
    };

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
        let str = generate(&repr);
        let new_repr = parse(&str)?;
        assert_eq!(repr, new_repr);
        Ok(())
    }
}
