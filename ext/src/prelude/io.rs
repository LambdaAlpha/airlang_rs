use std::{
    io::{
        stderr,
        stdin,
        stdout,
        Write,
    },
    rc::Rc,
};

use airlang::{
    CtxForMutableFn,
    EvalMode,
    IoMode,
    MutableCtx,
    Symbol,
    Val,
};

use crate::{
    prelude::{
        default_mode,
        put_func,
        symbol_value_mode,
        ExtFunc,
        Prelude,
    },
    ExtFn,
};

pub(crate) struct IoPrelude {
    pub(crate) read_line: Rc<ExtFunc>,
    pub(crate) print: Rc<ExtFunc>,
    pub(crate) print_line: Rc<ExtFunc>,
    pub(crate) flush: Rc<ExtFunc>,
    pub(crate) error_print: Rc<ExtFunc>,
    pub(crate) error_print_line: Rc<ExtFunc>,
    pub(crate) error_flush: Rc<ExtFunc>,
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
    fn put(&self, mut ctx: MutableCtx) {
        put_func(&self.read_line, ctx.reborrow());
        put_func(&self.print, ctx.reborrow());
        put_func(&self.print_line, ctx.reborrow());
        put_func(&self.flush, ctx.reborrow());
        put_func(&self.error_print, ctx.reborrow());
        put_func(&self.error_print_line, ctx.reborrow());
        put_func(&self.error_flush, ctx.reborrow());
    }
}

fn read_line() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.read_line") };
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    let ext_fn = ExtFn::new_mutable(fn_read_line);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
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

fn print() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.print") };
    let input_mode = default_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_print);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_print(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    print!("{s}");
    Val::default()
}

fn print_line() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.print_line") };
    let input_mode = default_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_print_line);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_print_line(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    println!("{s}");
    Val::default()
}

fn flush() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.flush") };
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = IoMode::Eval(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_flush);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_flush(_input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

fn error_print() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.error_print") };
    let input_mode = default_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_error_print);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_error_print(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    eprint!("{s}");
    Val::default()
}

fn error_print_line() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.error_print_line") };
    let input_mode = default_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
    let ext_fn = ExtFn::new_free(fn_error_print_line);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_error_print_line(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    eprintln!("{s}");
    Val::default()
}

fn error_flush() -> Rc<ExtFunc> {
    let id = unsafe { Symbol::from_str_unchecked("io.error_flush") };
    let ext_fn = ExtFn::new_free(fn_error_flush);
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = IoMode::Eval(EvalMode::Value);
    Rc::new(ExtFunc::new(id, input_mode, output_mode, ext_fn))
}

fn fn_error_flush(_input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
