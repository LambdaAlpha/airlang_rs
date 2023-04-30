use std::io::{
    self,
    Write,
};

pub(crate) trait Ui {
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str);
    fn eprint(&mut self, s: &str);
    fn eprintln(&mut self, s: &str);
    fn read_line(&mut self, s: &mut String);
}

pub(crate) struct StdUi;

impl Ui for StdUi {
    fn print(&mut self, s: &str) {
        print!("{s}");
        io::stdout().flush().unwrap();
    }

    fn println(&mut self, s: &str) {
        println!("{s}");
    }

    fn eprint(&mut self, s: &str) {
        eprint!("{s}");
        io::stderr().flush().unwrap();
    }

    fn eprintln(&mut self, s: &str) {
        eprintln!("{s}");
    }

    fn read_line(&mut self, s: &mut String) {
        io::stdin().read_line(s).unwrap();
    }
}
