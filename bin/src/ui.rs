use std::{
    error::Error,
    fmt::Display,
    io::{
        stderr,
        stdin,
        stdout,
        Write,
    },
};

pub(crate) trait Ui {
    fn print(&mut self, s: impl Display) -> std::io::Result<()>;
    fn println(&mut self, s: impl Display) -> std::io::Result<()>;
    fn eprint(&mut self, s: impl Error) -> std::io::Result<()>;
    fn eprintln(&mut self, s: impl Error) -> std::io::Result<()>;
    fn read_line(&mut self, s: &mut String) -> std::io::Result<usize>;
}

pub(crate) struct StdUi;

impl StdUi {
    pub(crate) fn new() -> Self {
        StdUi
    }
}

impl Ui for StdUi {
    fn print(&mut self, s: impl Display) -> std::io::Result<()> {
        print!("{s}");
        stdout().flush()
    }

    fn println(&mut self, s: impl Display) -> std::io::Result<()> {
        println!("{s}");
        stdout().flush()
    }

    fn eprint(&mut self, s: impl Error) -> std::io::Result<()> {
        eprint!("{s}");
        stderr().flush()
    }

    fn eprintln(&mut self, s: impl Error) -> std::io::Result<()> {
        eprintln!("{s}");
        stderr().flush()
    }

    fn read_line(&mut self, s: &mut String) -> std::io::Result<usize> {
        stdin().read_line(s)
    }
}
