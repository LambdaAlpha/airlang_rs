use {
    airlang::{
        self,
        interpret,
        parse,
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
    print_version();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut src = String::new();
        io::stdin().read_line(&mut src).unwrap();
        let src = src.trim();

        if matches!(src, "quit" | "exit") {
            break;
        }

        let result = interpret_str(&src);
        match result {
            Ok(s) => {
                println!("{s}")
            }
            Err(_) => {}
        }
    }
}

fn print_version() {
    // todo get air version
    let version = include_str!("./air/version.air");
    let version = interpret_str(version);
    match version {
        Ok(s) => {
            println!("🜁 air {s}")
        }
        Err(_) => {}
    }
}

fn interpret_str(src: &str) -> Result<String, Box<dyn Error>> {
    let src_repr = parse(src)?;
    let ret_repr = interpret(&src_repr)?;
    Ok((&ret_repr).into())
}
