use {
    crate::{
        ctx::ConstCtx,
        eval::Cmd,
        prelude::{
            build::BuildPrelude,
            eval::EvalPrelude,
            repl::ReplPrelude,
        },
    },
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

thread_local! (pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) repl: ReplPrelude,
    pub(crate) eval: EvalPrelude,
    pub(crate) build: BuildPrelude,
}

pub(crate) trait Prelude {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>);
}

impl Prelude for AllPrelude {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>) {
        self.repl.put(m);
        self.eval.put(m);
        self.build.put(m);
    }
}

pub(crate) fn initial_const_ctx() -> ConstCtx {
    let cmd_map = PRELUDE.with(|prelude| {
        let mut m = HashMap::default();
        prelude.put(&mut m);
        m
    });
    ConstCtx { cmd_map }
}

#[derive(Clone)]
pub(crate) struct Named<T> {
    pub(crate) name: &'static str,
    pub(crate) value: T,
}

impl Prelude for Named<Rc<dyn Cmd>> {
    fn put(&self, m: &mut HashMap<String, Rc<dyn Cmd>>) {
        let name = String::from(self.name);
        m.insert(name, self.value.clone());
    }
}

fn named_fn(name: &'static str, cmd: impl Cmd + 'static) -> Named<Rc<dyn Cmd>> {
    Named {
        name,
        value: Rc::new(cmd),
    }
}

mod repl;

mod eval;

mod build;
