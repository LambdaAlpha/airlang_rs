use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::FreeStaticPrimFuncVal;
use airlang::FuncMode;
use airlang::MutStaticPrimFuncVal;
use airlang::PreludeCtx;
use airlang::SymbolMode;
use airlang::Val;

use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;

pub(crate) struct IoPrelude {
    pub(crate) read_line: MutStaticPrimFuncVal,
    pub(crate) print: FreeStaticPrimFuncVal,
    pub(crate) print_line: FreeStaticPrimFuncVal,
    pub(crate) flush: FreeStaticPrimFuncVal,
    pub(crate) error_print: FreeStaticPrimFuncVal,
    pub(crate) error_print_line: FreeStaticPrimFuncVal,
    pub(crate) error_flush: FreeStaticPrimFuncVal,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.read_line.put(ctx);
        self.print.put(ctx);
        self.print_line.put(ctx);
        self.flush.put(ctx);
        self.error_print.put(ctx);
        self.error_print_line.put(ctx);
        self.error_flush.put(ctx);
    }
}

fn read_line() -> MutStaticPrimFuncVal {
    let forward =
        FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode());
    DynFn {
        id: "io.read_line",
        f: mut_impl(fn_read_line),
        mode: FuncMode { forward, reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_read_line(ctx: &mut Val, _input: Val) -> Val {
    let Val::Text(t) = ctx else {
        return Val::default();
    };
    let _ = stdin().read_line(t);
    Val::default()
}

fn print() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.print", f: free_impl(fn_print), mode: FuncMode::default() }.free_static()
}

fn fn_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    print!("{}", &**t);
    Val::default()
}

fn print_line() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.print_line", f: free_impl(fn_print_line), mode: FuncMode::default() }
        .free_static()
}

fn fn_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    println!("{}", &**t);
    Val::default()
}

fn flush() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.flush", f: free_impl(fn_flush), mode: FuncMode::default() }.free_static()
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

fn error_print() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.error_print", f: free_impl(fn_error_print), mode: FuncMode::default() }
        .free_static()
}

fn fn_error_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprint!("{}", &**t);
    Val::default()
}

fn error_print_line() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "io.error_print_line",
        f: free_impl(fn_error_print_line),
        mode: FuncMode::default(),
    }
    .free_static()
}

fn fn_error_print_line(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    eprintln!("{}", &**t);
    Val::default()
}

fn error_flush() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.error_flush", f: free_impl(fn_error_flush), mode: FuncMode::default() }
        .free_static()
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
