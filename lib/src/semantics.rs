use thiserror::Error;

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}
