use std::ops::Deref;

use crate::type_::either::Either;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConstRef<'a, T>(&'a mut T);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DynRef<'a, T> {
    ref_: &'a mut T,
    const_: bool,
}

impl<'a, T> Deref for ConstRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0
    }
}

impl<'a, T> Deref for DynRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.ref_
    }
}

impl<'a, T> ConstRef<'a, T> {
    pub fn new(ref_: &'a mut T) -> Self {
        ConstRef(ref_)
    }

    pub(crate) fn unwrap(self) -> &'a mut T {
        self.0
    }

    pub fn reborrow(&mut self) -> ConstRef<'_, T> {
        ConstRef(self.0)
    }

    pub fn opt_reborrow<'b>(this: &'b mut Option<ConstRef<'a, T>>) -> Option<ConstRef<'b, T>> {
        this.as_mut().map(Self::reborrow)
    }

    pub fn into_dyn(self) -> DynRef<'a, T> {
        DynRef { ref_: self.0, const_: true }
    }
}

impl<'a, T> DynRef<'a, T> {
    pub fn new(ref_: &'a mut T, const_: bool) -> Self {
        DynRef { ref_, const_ }
    }

    pub fn new_mut(ref_: &'a mut T) -> Self {
        DynRef { ref_, const_: false }
    }

    pub fn new_const(ref_: &'a mut T) -> Self {
        DynRef { ref_, const_: true }
    }

    pub(crate) fn unwrap(self) -> &'a mut T {
        self.ref_
    }

    pub fn reborrow(&mut self) -> DynRef<'_, T> {
        DynRef { ref_: self.ref_, const_: self.const_ }
    }

    pub fn opt_reborrow<'b>(this: &'b mut Option<DynRef<'a, T>>) -> Option<DynRef<'b, T>> {
        this.as_mut().map(Self::reborrow)
    }

    pub fn is_const(&self) -> bool {
        self.const_
    }

    pub fn or_const(&mut self, const_: bool) {
        self.const_ |= const_;
    }

    pub fn into_const(self) -> ConstRef<'a, T> {
        ConstRef(self.ref_)
    }

    pub fn into_either(self) -> Either<ConstRef<'a, T>, &'a mut T> {
        if self.const_ { Either::This(ConstRef(self.ref_)) } else { Either::That(self.ref_) }
    }

    pub fn map<S, F>(&mut self, f: F) -> DynRef<'_, S>
    where F: FnOnce(&mut T) -> DynRef<S> {
        let mut ref_ = f(self.ref_);
        ref_.or_const(self.const_);
        ref_
    }

    pub fn map_opt<S, F>(&mut self, f: F) -> Option<DynRef<'_, S>>
    where F: FnOnce(&mut T) -> Option<DynRef<S>> {
        match f(self.ref_) {
            Some(mut ref_) => {
                ref_.or_const(self.const_);
                Some(ref_)
            }
            None => None,
        }
    }

    pub fn map_res<S, F, E>(&mut self, f: F) -> Result<DynRef<'_, S>, E>
    where F: FnOnce(&mut T) -> Result<DynRef<S>, E> {
        match f(self.ref_) {
            Ok(mut ref_) => {
                ref_.or_const(self.const_);
                Ok(ref_)
            }
            Err(e) => Err(e),
        }
    }
}
