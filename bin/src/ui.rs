use std::{
    error::Error,
    fmt::Display,
    io::{
        BufRead,
        StderrLock,
        StdinLock,
        StdoutLock,
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

pub(crate) struct StdUi {
    stdin: StdinLock<'static>,
    stdout: StdoutLock<'static>,
    stderr: StderrLock<'static>,
}

impl StdUi {
    pub(crate) fn new() -> Self {
        StdUi {
            stdin: std::io::stdin().lock(),
            stdout: std::io::stdout().lock(),
            stderr: std::io::stderr().lock(),
        }
    }
}

impl Ui for StdUi {
    fn print(&mut self, s: impl Display) -> std::io::Result<()> {
        write!(&mut self.stdout, "{s}")?;
        self.stdout.flush()
    }

    fn println(&mut self, s: impl Display) -> std::io::Result<()> {
        writeln!(&mut self.stdout, "{s}")?;
        self.stdout.flush()
    }

    fn eprint(&mut self, s: impl Error) -> std::io::Result<()> {
        write!(&mut self.stderr, "{s}")?;
        self.stderr.flush()
    }

    fn eprintln(&mut self, s: impl Error) -> std::io::Result<()> {
        writeln!(&mut self.stderr, "{s}")?;
        self.stderr.flush()
    }

    fn read_line(&mut self, s: &mut String) -> std::io::Result<usize> {
        self.stdin.read_line(s)
    }
}
