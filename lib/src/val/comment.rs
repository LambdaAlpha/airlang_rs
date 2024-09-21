use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    comment::Comment,
    syntax::repr::comment::CommentRepr,
    ReprError,
    Val,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CommentVal(Box<Comment<Val, Val>>);

impl CommentVal {
    #[allow(unused)]
    pub(crate) fn new(comment: Box<Comment<Val, Val>>) -> Self {
        Self(comment)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Comment<Val, Val>> {
        self.0
    }
}

impl From<Comment<Val, Val>> for CommentVal {
    fn from(value: Comment<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<CommentVal> for Comment<Val, Val> {
    fn from(value: CommentVal) -> Self {
        *value.0
    }
}

impl From<&CommentRepr> for CommentVal {
    fn from(value: &CommentRepr) -> Self {
        let comment = Comment::new(Val::from(&value.meta), Val::from(&value.value));
        Self(Box::new(comment))
    }
}

impl From<CommentRepr> for CommentVal {
    fn from(value: CommentRepr) -> Self {
        let comment = Comment::new(Val::from(value.meta), Val::from(value.value));
        Self(Box::new(comment))
    }
}

impl TryInto<CommentRepr> for &CommentVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CommentRepr, Self::Error> {
        Ok(CommentRepr::new(
            (&self.meta).try_into()?,
            (&self.value).try_into()?,
        ))
    }
}

impl TryInto<CommentRepr> for CommentVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CommentRepr, Self::Error> {
        let comment = CommentRepr::new(self.0.meta.try_into()?, self.0.value.try_into()?);
        Ok(comment)
    }
}

impl Debug for CommentVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Comment::fmt(self, f)
    }
}

impl Deref for CommentVal {
    type Target = Comment<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommentVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
