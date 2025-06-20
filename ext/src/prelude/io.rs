use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::prelude::DynFn;
use airlang::prelude::FreeFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::free_impl;
use airlang::prelude::mode::FuncMode;
use airlang::prelude::mode::SymbolMode;
use airlang::prelude::mut_impl;
use airlang::semantics::val::FreeStaticPrimFuncVal;
use airlang::semantics::val::MutStaticPrimFuncVal;
use airlang::semantics::val::Val;

pub struct IoPrelude {
    pub read_line: MutStaticPrimFuncVal,
    pub print: FreeStaticPrimFuncVal,
    pub print_line: FreeStaticPrimFuncVal,
    pub flush: FreeStaticPrimFuncVal,
    pub error_print: FreeStaticPrimFuncVal,
    pub error_print_line: FreeStaticPrimFuncVal,
    pub error_flush: FreeStaticPrimFuncVal,
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

pub fn read_line() -> MutStaticPrimFuncVal {
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

pub fn print() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.print", f: free_impl(fn_print), mode: FuncMode::default() }.free_static()
}

fn fn_print(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    print!("{}", &**t);
    Val::default()
}

pub fn print_line() -> FreeStaticPrimFuncVal {
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

pub fn flush() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.flush", f: free_impl(fn_flush), mode: FuncMode::default() }.free_static()
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

pub fn error_print() -> FreeStaticPrimFuncVal {
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

pub fn error_print_line() -> FreeStaticPrimFuncVal {
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

pub fn error_flush() -> FreeStaticPrimFuncVal {
    FreeFn { id: "io.error_flush", f: free_impl(fn_error_flush), mode: FuncMode::default() }
        .free_static()
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
