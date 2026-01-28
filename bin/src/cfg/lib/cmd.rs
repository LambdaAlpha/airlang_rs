use std::process::Command;

use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::cfg::lib::FreeImpl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::List;
use airlang::type_::Pair;
use airlang::type_::Text;
use const_format::concatcp;

#[derive(Clone)]
pub struct CmdLib {
    pub call: FreePrimFuncVal,
}

const COMMAND: &str = "command";

pub const CALL: &str = concatcp!(PREFIX_ID, COMMAND, ".call");

impl Default for CmdLib {
    fn default() -> Self {
        Self { call: call() }
    }
}

impl CfgMod for CmdLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, CALL, self.call);
    }
}

// todo rename
// todo design
// todo impl
pub fn call() -> FreePrimFuncVal {
    FreeImpl { free: fn_call }.build()
}

fn fn_call(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{CALL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let program: &str = match &pair.left {
        Val::Text(program) => program,
        Val::Key(key) => key,
        v => {
            return bug!(cfg, "{CALL}: expected input.left to be a text or a key, but got {v}");
        }
    };
    let Val::List(arguments) = pair.right else {
        return bug!(cfg, "{CALL}: expected input.right to be a list, but got {}", pair.right);
    };
    let mut args = Vec::with_capacity(arguments.len());
    for arg in List::from(arguments) {
        let Val::Text(arg) = arg else {
            return bug!(cfg, "{CALL}: expected argument to be a text, but got {arg}");
        };
        let arg = Text::from(arg);
        args.push(String::from(arg));
    }

    let child = Command::new(program).args(args).spawn();
    let Ok(mut child) = child else {
        eprintln!("failed to execute program");
        return Val::default();
    };
    let Ok(status) = child.wait() else {
        return Val::default();
    };

    if let Some(status) = status.code()
        && status != 0
    {
        println!("program exit with code: {status}");
    }
    Val::default()
}
