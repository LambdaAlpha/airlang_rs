use std::process::Command;

use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::cfg::lib::FreeImpl;
use airlang::cfg::lib::ImplExtra;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Byte;
use airlang::type_::Cell;
use airlang::type_::Int;
use airlang::type_::Key;
use airlang::type_::List;
use airlang::type_::Map;
use airlang::type_::Text;
use const_format::concatcp;

#[derive(Clone)]
pub struct ProcessLib {
    pub call: PrimFuncVal,
}

const PROCESS: &str = "process";

pub const CALL: &str = concatcp!(PREFIX_ID, PROCESS, ".call");

impl Default for ProcessLib {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl CfgMod for ProcessLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, CALL, self.call);
    }
}

const PROGRAM: &str = "program";
const ARGUMENTS: &str = "arguments";

// todo design
// todo impl
pub fn call() -> PrimFuncVal {
    FreeImpl { fn_: fn_call }.build(ImplExtra { raw_input: false })
}

fn fn_call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return bug!(cfg, "{CALL}: expected input to be a map, but got {input}");
    };
    let Some(program) = map.remove(&Key::from_str_unchecked(PROGRAM)) else {
        return bug!(cfg, "{CALL}: {PROGRAM} not found");
    };
    let Val::Text(program) = program else {
        return bug!(cfg, "{CALL}: expected {PROGRAM} to be a text, but got {program}");
    };
    let Some(arguments) = map.remove(&Key::from_str_unchecked(ARGUMENTS)) else {
        return bug!(cfg, "{CALL}: {ARGUMENTS} not found");
    };
    let Val::List(arguments) = arguments else {
        return bug!(cfg, "{CALL}: expected {ARGUMENTS} to be a list, but got {arguments}");
    };
    let mut args = Vec::with_capacity(arguments.len());
    for arg in List::from(arguments) {
        let Val::Text(arg) = arg else {
            return bug!(cfg, "{CALL}: expected argument to be a text, but got {arg}");
        };
        let arg = Text::from(arg);
        args.push(String::from(arg));
    }

    let output = Command::new(&**program).args(args).output();
    let output = match output {
        Ok(output) => output,
        Err(_e) => return Val::default(),
    };

    let stdout = Val::Byte(Byte::from(output.stdout).into());
    let stderr = Val::Byte(Byte::from(output.stderr).into());
    let status = if let Some(status) = output.status.code() {
        Val::Int(Int::from(status).into())
    } else {
        Val::default()
    };

    let mut map = Map::default();
    map.insert(Key::from_str_unchecked("output"), stdout);
    map.insert(Key::from_str_unchecked("error"), stderr);
    map.insert(Key::from_str_unchecked("status"), status);
    Val::Cell(Cell::new(Val::Map(map.into())).into())
}
