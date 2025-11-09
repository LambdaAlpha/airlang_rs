fn main() -> std::io::Result<()> {
    cmd::main()
}

mod repl;

mod cmd;

mod cfg2;

mod cfg;
