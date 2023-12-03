use {
    crate::eval::Cmd,
    airlang::semantics::Interpreter,
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

pub(crate) struct ConstCtx {
    pub(crate) cmd_map: HashMap<String, Rc<dyn Cmd>>,
}

pub(crate) struct DynCtx {
    pub(crate) interpreter: Interpreter,
    pub(crate) meta_interpreter: Interpreter,

    pub(crate) multiline_mode: bool,
}
