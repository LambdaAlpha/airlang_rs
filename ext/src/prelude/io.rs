use std::io::{
    stderr,
    stdin,
    stdout,
    Write,
};

use airlang::{
    FuncVal,
    Mode,
    MutCtx,
    MutFnCtx,
    Val,
};

use crate::prelude::{
    named_mut_fn,
    named_static_fn,
    Named,
    Prelude,
};

pub(crate) struct IoPrelude {
    pub(crate) read_line: Named<FuncVal>,
    pub(crate) print: Named<FuncVal>,
    pub(crate) print_line: Named<FuncVal>,
    pub(crate) flush: Named<FuncVal>,
    pub(crate) error_print: Named<FuncVal>,
    pub(crate) error_print_line: Named<FuncVal>,
    pub(crate) error_flush: Named<FuncVal>,
}

impl Default for IoPrelude {
    fn default() -> Self {
        Self {
            read_line: read_line(),
            print: print(),
            print_line: print_line(),
            flush: flush(),
            error_print: error_print(),
            error_print_line: error_print_line(),
            error_flush: error_flush(),
        }
    }
}

impl Prelude for IoPrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.read_line.put(ctx.reborrow());
        self.print.put(ctx.reborrow());
        self.print_line.put(ctx.reborrow());
        self.flush.put(ctx.reborrow());
        self.error_print.put(ctx.reborrow());
        self.error_print_line.put(ctx.reborrow());
        self.error_flush.put(ctx.reborrow());
    }
}

fn read_line() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("io.read_line", input_mode, output_mode, false, fn_read_line)
}

fn fn_read_line(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(str) = ctx.get_ref_mut(s) else {
        return Val::default();
    };
    let Val::Text(t) = str else {
        return Val::default();
    };
    let _ = stdin().read_line(t);
    Val::default()
}

fn print() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn("io.print", input_mode, output_mode, false, fn_print)
}

fn fn_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    print!("{}", &**t);
    Val::default()
}

fn print_line() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn(
        "io.print_line",
        input_mode,
        output_mode,
        false,
        fn_print_line,
    )
}

fn fn_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    println!("{}", &**t);
    Val::default()
}

fn flush() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn("io.flush", input_mode, output_mode, false, fn_flush)
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

fn error_print() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn(
        "io.error_print",
        input_mode,
        output_mode,
        false,
        fn_error_print,
    )
}

fn fn_error_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprint!("{}", &**t);
    Val::default()
}

fn error_print_line() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn(
        "io.error_print_line",
        input_mode,
        output_mode,
        false,
        fn_error_print_line,
    )
}

fn fn_error_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprintln!("{}", &**t);
    Val::default()
}

fn error_flush() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_static_fn(
        "io.error_flush",
        input_mode,
        output_mode,
        false,
        fn_error_flush,
    )
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
