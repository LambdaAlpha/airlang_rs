use {
    crate::repl::{
        cmd::repl::title,
        eval::{
            ConstCtx,
            DynCtx,
            Output,
        },
        ui::Ui,
    },
    airlang::semantics::{
        parse,
        Interpreter,
        Val,
    },
};

pub(crate) fn repl(ui: &mut impl Ui) {
    let const_ctx = cmd::const_ctx();
    let mut dyn_ctx = dyn_ctx();
    let mut input_buffer = String::new();

    print_title(ui, &const_ctx, &mut dyn_ctx);

    loop {
        ui.print(PROMPT_PREFIX);

        if !input_buffer.is_empty() {
            input_buffer.push('\n');
        }

        let len = input_buffer.len();
        ui.read_line(&mut input_buffer);
        input_buffer.truncate(input_buffer.trim_end().len());

        let input = input_buffer.trim();
        if input.is_empty() {
            input_buffer.clear();
            continue;
        }

        if input == "((" {
            dyn_ctx.multiline_mode = true;
        } else {
            let newline = input_buffer[len..].trim();
            if newline == "))" {
                dyn_ctx.multiline_mode = false;
            }
        }
        if dyn_ctx.multiline_mode {
            continue;
        }

        match eval(&const_ctx, &mut dyn_ctx, input) {
            Output::Ok(output) => {
                ui.println(&output);
            }
            Output::Err(error) => {
                ui.eprintln(&*error);
            }
            Output::Break => break,
        }
        input_buffer.clear();
    }
}

pub(crate) fn dyn_ctx() -> DynCtx {
    DynCtx {
        interpreter: Interpreter::new(),
        meta_interpreter: Interpreter::new(),
        multiline_mode: false,
    }
}

fn eval(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, input: &str) -> Output {
    match parse(input) {
        Ok(input) => const_ctx.eval(dyn_ctx, input),
        Err(err) => Output::Err(Box::new(err)),
    }
}

fn print_title(ui: &mut impl Ui, const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx) {
    match title(const_ctx, dyn_ctx, Val::default()) {
        Output::Ok(output) => {
            ui.println(&output);
        }
        Output::Err(error) => {
            ui.eprintln(&*error);
        }
        _ => {}
    }
}

const PROMPT_PREFIX: &str = "> ";

pub(crate) mod cmd;

pub(crate) mod eval;

pub(crate) mod ui;
