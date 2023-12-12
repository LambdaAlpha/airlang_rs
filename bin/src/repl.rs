use {
    crate::{
        ctx::{
            ConstCtx,
            DynCtx,
        },
        eval::{
            eval,
            Output,
        },
        prelude::initial_const_ctx,
        ui::Ui,
    },
    airlang::{
        initial_ctx,
        interpret_mutable,
        parse,
        Ctx,
        MutableCtx,
        Val,
    },
    std::error::Error,
};

pub(crate) fn repl(ui: &mut impl Ui) -> std::io::Result<()> {
    let const_ctx = initial_const_ctx();
    let mut dyn_ctx = dyn_ctx();
    let mut input_buffer = String::new();

    print_title(ui, &mut dyn_ctx.ctx)?;

    loop {
        ui.print(PROMPT_PREFIX)?;

        let len = input_buffer.len();
        ui.read_line(&mut input_buffer)?;

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

        match repl_eval(&const_ctx, &mut dyn_ctx, input) {
            Output::Print(output) => {
                ui.println(&output)?;
            }
            Output::Eprint(error) => {
                ui.eprintln(&*error)?;
            }
            Output::Break => break,
        }
        input_buffer.clear();
    }
    Ok(())
}

pub(crate) fn dyn_ctx() -> DynCtx {
    DynCtx {
        ctx: initial_ctx(),
        meta_ctx: initial_ctx(),
        multiline_mode: false,
    }
}

fn repl_eval(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, input: &str) -> Output {
    match parse(input) {
        Ok(input) => eval(const_ctx, dyn_ctx, input),
        Err(err) => Output::Eprint(Box::new(err)),
    }
}

const TITLE_PREFIX: &str = "🜁 Air ";

fn print_title(ui: &mut impl Ui, ctx: &mut Ctx) -> std::io::Result<()> {
    match parse(include_str!("air/version.air")) {
        Ok(repr) => {
            let mutable_ctx = MutableCtx::new(ctx);
            match interpret_mutable(mutable_ctx, repr) {
                Val::String(s) => ui.println(format!("{}{}", TITLE_PREFIX, &*s)),
                _ => {
                    let msg = format!("{} unknown version", TITLE_PREFIX);
                    ui.eprintln(&*Box::<dyn Error>::from(msg))
                }
            }
        }
        Err(err) => ui.eprintln(err),
    }
}

const PROMPT_PREFIX: &str = "> ";
