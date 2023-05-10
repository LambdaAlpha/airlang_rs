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

    print_title(ui, &const_ctx, &mut dyn_ctx);

    loop {
        ui.print(PROMPT_PREFIX);

        let mut input = String::new();
        ui.read_line(&mut input);
        let input = input.trim();

        if input == "" {
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
    }
}

pub(crate) fn dyn_ctx() -> DynCtx {
    DynCtx {
        interpreter: Interpreter::new(),
        meta_interpreter: Interpreter::new(),
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
