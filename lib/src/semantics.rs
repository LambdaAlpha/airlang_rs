use {
    crate::{
        repr::Repr,
        semantics::val::Val,
    },
    thiserror::Error,
};

#[cfg(test)]
mod test;
pub(crate) mod val;

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}

pub fn interpret(src: &Repr) -> Result<Repr, ReprError> {
    let input = Val::from(src);
    let output = eval(input);
    output.try_into()
}

// todo eval impl
pub(crate) fn eval(src: Val) -> Val {
    src
}
