use airlang::cfg::prelude::Prelude;
use airlang::cfg::prelude::memo_put_func;
use airlang::semantics::memo::Contract;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::TextVal;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;
use airlang_ext::cfg::prelude::StdPrelude;

use crate::cfg::lib::BinLib;

#[derive(Clone)]
pub struct BinPrelude {
    pub std: StdPrelude,
    pub help: TextVal,
    pub call: FreePrimFuncVal,
}

impl BinPrelude {
    pub fn new(lib: &BinLib) -> Self {
        Self {
            std: StdPrelude::new(&lib.std),
            help: lib.repl.help.clone(),
            call: lib.cmd.call.clone(),
        }
    }
}

impl Prelude for BinPrelude {
    fn extend(&self, memo: &mut Memo) {
        self.std.extend(memo);
        let _ = memo.put(
            Symbol::from_str_unchecked("help"),
            Val::Text(self.help.clone()),
            Contract::default(),
        );
        memo_put_func(memo, ";", &self.call);
    }
}
