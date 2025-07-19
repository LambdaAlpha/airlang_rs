use std::borrow::Borrow;

use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use derive_more::Into;

#[derive(
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    From,
    Into,
    derive_more::Debug,
    derive_more::Display,
    Deref,
    DerefMut,
)]
#[debug("{_0:?}")]
#[display("{_0}")]
#[from(&str, String)]
pub struct Text(String);

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self
    }
}
