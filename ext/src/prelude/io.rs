use std::io::{
    Write,
    stderr,
    stdin,
    stdout,
};

use airlang::{
    FuncMode,
    FuncVal,
    Mode,
    MutCtx,
    MutFnCtx,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    named_free_fn,
    named_mut_fn,
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
    let id = "io.read_line";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_read_line;
    named_mut_fn(id, mode, cacheable, f)
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
    let id = "io.print";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_print;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    print!("{}", &**t);
    Val::default()
}

fn print_line() -> Named<FuncVal> {
    let id = "io.print_line";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_print_line;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    println!("{}", &**t);
    Val::default()
}

fn flush() -> Named<FuncVal> {
    let id = "io.flush";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_flush;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

fn error_print() -> Named<FuncVal> {
    let id = "io.error_print";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_error_print;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_error_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprint!("{}", &**t);
    Val::default()
}

fn error_print_line() -> Named<FuncVal> {
    let id = "io.error_print_line";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_error_print_line;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_error_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprintln!("{}", &**t);
    Val::default()
}

fn error_flush() -> Named<FuncVal> {
    let id = "io.error_flush";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = false;
    let f = fn_error_flush;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
