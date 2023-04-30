use {
    airlang::repr::Repr,
    std::ops::ControlFlow,
};

pub(crate) const EXIT: &'static str = "exit";
pub(crate) const QUIT: &'static str = "quit";
pub(crate) const RESET: &'static str = "reset";

pub(crate) struct CtrlEval;

impl CtrlEval {
    pub(crate) fn eval(&self, ctrl: Repr) -> ControlFlow<(), Command> {
        match ctrl {
            Repr::Symbol(s) => match &*s {
                EXIT | QUIT => return ControlFlow::Break(()),
                RESET => return ControlFlow::Continue(Command::Reset),
                _ => {}
            },
            _ => {}
        }
        ControlFlow::Continue(Command::Unknown)
    }
}

pub(crate) enum Command {
    Reset,
    Unknown,
}
