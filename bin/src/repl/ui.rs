use std::{
    error::Error,
    fmt::Display,
    io::{
        Stderr,
        Stdin,
        Stdout,
        Write,
    },
};

pub(crate) trait Ui {
    fn print(&mut self, s: impl Display);
    fn println(&mut self, s: impl Display);
    fn eprint(&mut self, s: impl Error);
    fn eprintln(&mut self, s: impl Error);
    fn read_line(&mut self, s: &mut String);
}

pub(crate) struct StdUi {
    stdin: Stdin,
    stdout: Stdout,
    stderr: Stderr,
}

impl StdUi {
    pub(crate) fn new() -> Self {
        StdUi {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        }
    }
}

impl Ui for StdUi {
    fn print(&mut self, s: impl Display) {
        write!(&mut self.stdout, "{s}").unwrap();
        self.stdout.flush().unwrap();
    }

    fn println(&mut self, s: impl Display) {
        writeln!(&mut self.stdout, "{s}").unwrap();
    }

    fn eprint(&mut self, s: impl Error) {
        write!(&mut self.stderr, "{s}").unwrap();
        self.stderr.flush().unwrap();
    }

    fn eprintln(&mut self, s: impl Error) {
        writeln!(&mut self.stderr, "{s}").unwrap();
        self.stderr.flush().unwrap();
    }

    fn read_line(&mut self, s: &mut String) {
        self.stdin.read_line(s).unwrap();
    }
}
