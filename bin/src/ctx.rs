use {
    crate::eval::Cmd,
    airlang::Ctx,
    std::{
        collections::HashMap,
        rc::Rc,
    },
};

pub(crate) struct ConstCtx {
    pub(crate) cmd_map: HashMap<String, Rc<dyn Cmd>>,
}

pub(crate) struct DynCtx {
    pub(crate) ctx: Ctx,
    pub(crate) meta_ctx: Ctx,

    pub(crate) multiline_mode: bool,
}
