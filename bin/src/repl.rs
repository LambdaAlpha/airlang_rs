use {
    airlang::{
        self,
        semantics::Interpreter,
        syntax::parse,
    },
    std::{
        error::Error,
        io::{
            self,
            Write,
        },
    },
};

pub fn repl() {
    let mut interpreter = Interpreter::new();
    print_version(&mut interpreter);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut src = String::new();
        io::stdin().read_line(&mut src).unwrap();
        let src = src.trim();

        match src {
            "# quit" | "# exit" => break,
            "# reset" => {
                interpreter.reset();
                continue;
            }
            _ => {}
        }

        let result = interpret_str(&mut interpreter, &src);
        match result {
            Ok(s) => {
                println!("{s}")
            }
            Err(e) => {
                println!("# {e}")
            }
        }
    }
}

fn print_version(interpreter: &mut Interpreter) {
    let version = include_str!("./air/version.air");
    let version = interpret_str(interpreter, version);
    match version {
        Ok(s) => {
            println!("🜁 Air {s}")
        }
        Err(_) => {}
    }
}

fn interpret_str(interpreter: &mut Interpreter, src: &str) -> Result<String, Box<dyn Error>> {
    let src_repr = parse(src)?;
    let ret_repr = interpreter.interpret(&src_repr)?;
    Ok((&ret_repr).into())
}
