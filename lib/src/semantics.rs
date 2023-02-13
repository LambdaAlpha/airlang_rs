use {
    crate::semantics::val::Val,
    thiserror::Error,
};

#[cfg(test)]
mod test;
pub(crate) mod val;

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}

// todo eval impl
pub(crate) fn eval(src: Val) -> Val {
    src
}
