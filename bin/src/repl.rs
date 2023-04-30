use {
    crate::repl::{
        ctrl::{
            Command,
            CtrlEval,
        },
        ui::Ui,
    },
    airlang::{
        self,
        repr::Repr,
        semantics::Interpreter,
        syntax::parse,
    },
    std::{
        fmt::Display,
        ops::ControlFlow,
    },
};

pub(crate) fn repl(ui: &mut impl Ui) {
    let mut eval = Eval::new();

    ui.print(TITLE_PREFIX);
    let version = include_str!("./air/version.air");
    println(ui, eval.eval_input(version, true));

    loop {
        ui.print(PROMPT_PREFIX);

        let mut input = String::new();
        ui.read_line(&mut input);
        let input = input.trim();

        if input == "" {
            continue;
        }

        match eval.eval(input) {
            ControlFlow::Continue(result) => println(ui, result),
            ControlFlow::Break(_) => break,
        }
    }
}

const PROMPT_PREFIX: &str = "> ";
const TITLE_PREFIX: &str = "🜁 Air ";

fn println(ui: &mut impl Ui, result: Result<String, String>) {
    match result {
        Ok(output) => {
            ui.println(&output);
        }
        Err(error) => {
            ui.eprintln(&error);
        }
    }
}

pub(crate) struct Eval {
    interpreter: Interpreter,
    ctrl_eval: CtrlEval,
}

impl Eval {
    pub(crate) fn new() -> Self {
        Eval {
            interpreter: Interpreter::new(),
            ctrl_eval: CtrlEval,
        }
    }

    pub(crate) fn eval(&mut self, input: &str) -> ControlFlow<(), Result<String, String>> {
        if input.starts_with(COMMAND_PREFIX) {
            match parse(&input[1..]) {
                Ok(control) => {
                    let command = self.ctrl_eval.eval(control)?;
                    ControlFlow::Continue(self.execute_command(command))
                }
                Err(e) => ControlFlow::Continue(self.error(PARSE_COMMAND_ERROR, e)),
            }
        } else {
            ControlFlow::Continue(self.eval_input(&input, false))
        }
    }

    fn eval_input(&mut self, input: &str, unquote_string: bool) -> Result<String, String> {
        match parse(&input) {
            Ok(input) => match self.interpreter.interpret(&input) {
                Ok(output) => match output {
                    Repr::String(ref s) => {
                        if unquote_string {
                            self.output(&**s)
                        } else {
                            self.output(output)
                        }
                    }
                    output => self.output(output),
                },
                Err(e) => self.error(REPRESENT_ERROR, e),
            },
            Err(e) => self.error(PARSE_INPUT_ERROR, e),
        }
    }

    fn execute_command(&mut self, command: Command) -> Result<String, String> {
        match command {
            Command::Reset => {
                self.interpreter.reset();
                Ok("".to_owned())
            }
            Command::Unknown => self.error(UNKNOWN_COMMAND_ERROR, ""),
        }
    }

    fn output(&mut self, s: impl Display) -> Result<String, String> {
        Ok(s.to_string())
    }

    fn error(&mut self, prefix: &str, err: impl Display) -> Result<String, String> {
        Err(format!("{}: {}", prefix, err))
    }
}

const COMMAND_PREFIX: &str = ":";

const PARSE_COMMAND_ERROR: &str = "syntax error";
const UNKNOWN_COMMAND_ERROR: &str = "unknown command";
const PARSE_INPUT_ERROR: &str = "syntax error";
const REPRESENT_ERROR: &str = "output is not representable";

pub(crate) mod ctrl;

pub(crate) mod ui;
