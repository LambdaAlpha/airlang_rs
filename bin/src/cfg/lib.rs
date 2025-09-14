use airlang::cfg::CfgMod;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Contract;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::FuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;
use airlang_ext::cfg::lib::StdLib;

use self::cmd::CmdLib;
use self::repl::ReplLib;

#[derive(Default, Clone)]
pub struct BinLib {
    pub repl: ReplLib,
    pub cmd: CmdLib,
    pub std: StdLib,
}

impl CfgMod for BinLib {
    fn extend(self, cfg: &Cfg) {
        self.repl.extend(cfg);
        self.cmd.extend(cfg);
        self.std.extend(cfg);
    }
}

impl Library for BinLib {
    fn prelude(&self, memo: &mut Memo) {
        self.repl.prelude(memo);
        self.cmd.prelude(memo);
        self.std.prelude(memo);
    }
}

fn memo_put_func<V: Clone + Into<FuncVal>>(memo: &mut Memo, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let v = memo.put(name, Val::Func(val.clone().into()), Contract::None);
    assert!(matches!(v, Ok(None)), "names of preludes should be unique");
}

pub mod repl;

pub mod cmd;
