use {
    airlang::{
        repr::Repr,
        semantics::Interpreter,
    },
    std::{
        collections::HashMap,
        error::Error,
        fmt::Display,
    },
};

pub(crate) struct ConstCtx {
    pub(crate) cmd_map: HashMap<String, Box<dyn Cmd>>,
}

pub(crate) struct DynCtx {
    pub(crate) interpreter: Interpreter,
    pub(crate) meta_interpreter: Interpreter,
}

pub(crate) trait Cmd = Fn(&ConstCtx, &mut DynCtx, &Repr) -> Output;

pub(crate) enum Output {
    Break,
    Ok(Box<dyn Display>),
    Err(Box<dyn Error>),
}

impl ConstCtx {
    pub(crate) fn eval(&self, dyn_ctx: &mut DynCtx, input: &Repr) -> Output {
        if let Repr::Call(call) = input {
            if let Repr::Symbol(func) = &call.func {
                match &**func {
                    CMD => return self.eval_cmd(dyn_ctx, &call.input),
                    AIR => return self.eval_air(dyn_ctx, &call.input),
                    _ => {}
                }
            }
        }
        self.eval_air(dyn_ctx, input)
    }

    pub(crate) fn eval_cmd(&self, dyn_ctx: &mut DynCtx, input: &Repr) -> Output {
        match input {
            Repr::Symbol(ref s) => {
                if let Some(f) = self.cmd_map.get(&**s) {
                    return f(self, dyn_ctx, &Repr::default());
                }
            }
            Repr::Call(call) => {
                if let Repr::Symbol(func) = &call.func {
                    if &**func == AIR {
                        return self.eval_meta(dyn_ctx, &call.input);
                    } else if let Some(f) = self.cmd_map.get(&**func) {
                        return f(self, dyn_ctx, &call.input);
                    }
                }
            }
            _ => {}
        }
        self.eval_meta(dyn_ctx, input)
    }

    pub(crate) fn eval_air(&self, dyn_ctx: &mut DynCtx, repr: &Repr) -> Output {
        Self::eval_interpret(&mut dyn_ctx.interpreter, repr)
    }

    pub(crate) fn eval_meta(&self, dyn_ctx: &mut DynCtx, repr: &Repr) -> Output {
        Self::eval_interpret(&mut dyn_ctx.meta_interpreter, repr)
    }

    fn eval_interpret(interpreter: &mut Interpreter, repr: &Repr) -> Output {
        match interpreter.interpret(repr) {
            Ok(output) => Output::Ok(Box::new(output)),
            Err(err) => Output::Err(Box::new(err)),
        }
    }
}

const CMD: &str = "#cmd";
const AIR: &str = "#air";
