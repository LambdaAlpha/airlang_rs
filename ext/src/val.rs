use {
    crate::ExtFunc,
    airlang::{
        AsFuncExt,
        FuncExt,
    },
    std::rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtVal {
    Func(Rc<ExtFunc>),
}

impl AsFuncExt for ExtVal {
    fn as_func(&self) -> Option<&dyn FuncExt> {
        match self {
            ExtVal::Func(func) => Some(&**func),
        }
    }
}
