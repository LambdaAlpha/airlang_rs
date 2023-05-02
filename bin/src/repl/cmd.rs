use {
    crate::repl::eval::{
        Cmd,
        ConstCtx,
    },
    std::collections::HashMap,
};

pub(crate) fn const_ctx() -> ConstCtx {
    let mut m: HashMap<String, Box<dyn Cmd>> = HashMap::new();

    put(&mut m, names::REPL_TITLE, repl::title);
    put(&mut m, names::REPL_QUIT, repl::quit);
    put(&mut m, names::REPL_EXIT, repl::exit);

    put(&mut m, names::RESET, interpreter::reset);

    put(&mut m, names::IMPORT, file::import);

    ConstCtx { cmd_map: m }
}

fn put(map: &mut HashMap<String, Box<dyn Cmd>>, name: &str, cmd: impl Cmd + 'static) {
    map.insert(name.to_owned(), Box::new(cmd));
}

pub(crate) mod names {
    pub(crate) const REPL_TITLE: &str = "repl_title";
    pub(crate) const REPL_EXIT: &str = "exit";
    pub(crate) const REPL_QUIT: &str = "quit";

    pub(crate) const RESET: &str = "reset";

    pub(crate) const IMPORT: &str = "import";
}

pub(crate) mod repl;

pub(crate) mod interpreter;

pub(crate) mod file;
