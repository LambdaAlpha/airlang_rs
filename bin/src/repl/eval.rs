use {
    airlang::semantics::{
        generate,
        Interpreter,
        Val,
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

    pub(crate) multiline_mode: bool,
}

pub(crate) trait Cmd = Fn(&ConstCtx, &mut DynCtx, Val) -> Output;

pub(crate) enum Output {
    Break,
    Ok(Box<dyn Display>),
    Err(Box<dyn Error>),
}

impl ConstCtx {
    pub(crate) fn eval(&self, dyn_ctx: &mut DynCtx, input: Val) -> Output {
        let Val::Call(call) = input else {
            return self.eval_air(dyn_ctx, input);
        };
        let Val::Symbol(func) = &call.func else {
            return self.eval_air(dyn_ctx, Val::Call(call));
        };
        match &**func {
            CMD => self.eval_cmd(dyn_ctx, call.input),
            AIR => self.eval_air(dyn_ctx, call.input),
            _ => self.eval_air(dyn_ctx, Val::Call(call)),
        }
    }

    pub(crate) fn eval_cmd(&self, dyn_ctx: &mut DynCtx, input: Val) -> Output {
        match input {
            Val::Symbol(ref s) => {
                let Some(f) = self.cmd_map.get(&**s) else {
                    return self.eval_meta(dyn_ctx, input);
                };
                f(self, dyn_ctx, Val::default())
            }
            Val::Call(call) => {
                let Val::Symbol(func) = &call.func else {
                    return self.eval_meta(dyn_ctx, Val::Call(call));
                };
                if &**func == AIR {
                    return self.eval_meta(dyn_ctx, call.input);
                }
                let Some(f) = self.cmd_map.get(&**func) else {
                    return self.eval_meta(dyn_ctx, Val::Call(call));
                };
                f(self, dyn_ctx, call.input)
            }
            input => self.eval_meta(dyn_ctx, input),
        }
    }

    pub(crate) fn eval_air(&self, dyn_ctx: &mut DynCtx, val: Val) -> Output {
        Self::eval_interpret(&mut dyn_ctx.interpreter, val)
    }

    pub(crate) fn eval_meta(&self, dyn_ctx: &mut DynCtx, val: Val) -> Output {
        Self::eval_interpret(&mut dyn_ctx.meta_interpreter, val)
    }

    fn eval_interpret(interpreter: &mut Interpreter, val: Val) -> Output {
        let output = interpreter.interpret(val);
        match generate(&output) {
            Ok(output) => Output::Ok(Box::new(output)),
            Err(err) => Output::Err(Box::new(err)),
        }
    }
}

const CMD: &str = ":cmd";
const AIR: &str = ":air";
