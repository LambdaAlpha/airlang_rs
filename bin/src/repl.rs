use {
    airlang::{
        generate,
        initial_ctx,
        interpret_mutable,
        parse,
        MutableCtx,
        Val,
    },
    std::io::{
        stdin,
        stdout,
        Result,
        Write,
    },
};

pub(crate) fn repl() -> Result<()> {
    let mut ctx = initial_ctx();
    let mut mutable_ctx = MutableCtx::new(&mut ctx);
    let mut multiline_mode = false;
    let mut input_buffer = String::new();

    print_title(mutable_ctx.reborrow());

    loop {
        print!("{}", PROMPT_PREFIX);
        stdout().flush()?;

        let len = input_buffer.len();
        stdin().read_line(&mut input_buffer)?;

        let input = input_buffer.trim();
        if input.is_empty() {
            input_buffer.clear();
            continue;
        }

        if input == "((" {
            multiline_mode = true;
        } else {
            let newline = input_buffer[len..].trim();
            if newline == "))" {
                multiline_mode = false;
            }
        }
        if multiline_mode {
            continue;
        }

        eval_print(mutable_ctx.reborrow(), input);

        input_buffer.clear();
    }
}

const TITLE_PREFIX: &str = "🜁 Air ";

fn print_title(ctx: MutableCtx) {
    match parse(include_str!("air/version.air")) {
        Ok(repr) => match interpret_mutable(ctx, repr) {
            Val::String(s) => println!("{}{}", TITLE_PREFIX, &*s),
            _ => {
                let msg = format!("{} unknown version", TITLE_PREFIX);
                eprintln!("{}", msg)
            }
        },
        Err(err) => eprintln!("{}", err),
    }
}

const PROMPT_PREFIX: &str = "> ";

fn eval_print(ctx: MutableCtx, input: &str) {
    match parse(input) {
        Ok(input) => {
            let output = interpret_mutable(ctx, input);
            match generate(&output) {
                Ok(output) => {
                    println!("{}", output);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
