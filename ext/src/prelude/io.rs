use {
    crate::{
        prelude::{
            NamedExtFunc,
            Prelude,
        },
        ExtFn,
        ExtFunc,
    },
    airlang::{
        CtxForMutableFn,
        EvalMode,
        IoMode,
        Symbol,
        Val,
    },
    std::{
        collections::HashMap,
        io::{
            stderr,
            stdin,
            stdout,
            Write,
        },
    },
};

#[derive(Clone)]
pub(crate) struct IoPrelude {
    pub(crate) read_line: NamedExtFunc,
    pub(crate) print: NamedExtFunc,
    pub(crate) print_line: NamedExtFunc,
    pub(crate) flush: NamedExtFunc,
    pub(crate) error_print: NamedExtFunc,
    pub(crate) error_print_line: NamedExtFunc,
    pub(crate) error_flush: NamedExtFunc,
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
    fn put(&self, m: &mut HashMap<Symbol, ExtFunc>) {
        self.read_line.put(m);
        self.print.put(m);
        self.print_line.put(m);
        self.flush.put(m);
        self.error_print.put(m);
        self.error_print_line.put(m);
        self.error_flush.put(m);
    }
}

fn read_line() -> NamedExtFunc {
    let ext_fn = ExtFn::new_mutable(fn_read_line);
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.read_line", func)
}

fn fn_read_line(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(str) = ctx.get_mut(&s) else {
        return Val::default();
    };
    let Val::String(str) = str else {
        return Val::default();
    };
    let _ = stdin().read_line(str);
    Val::default()
}

fn print() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_print);
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.print", func)
}

fn fn_print(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    print!("{s}");
    Val::default()
}

fn print_line() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_print_line);
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.print_line", func)
}

fn fn_print_line(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    println!("{s}");
    Val::default()
}

fn flush() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_flush);
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.flush", func)
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

fn error_print() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_error_print);
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.error_print", func)
}

fn fn_error_print(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    eprint!("{s}");
    Val::default()
}

fn error_print_line() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_error_print_line);
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.error_print_line", func)
}

fn fn_error_print_line(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    eprintln!("{s}");
    Val::default()
}

fn error_flush() -> NamedExtFunc {
    let ext_fn = ExtFn::new_free(fn_error_flush);
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    let func = ExtFunc::new(input_mode, output_mode, ext_fn);
    NamedExtFunc::new("io.error_flush", func)
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
