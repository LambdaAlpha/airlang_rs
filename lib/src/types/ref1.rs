use std::ops::Deref;

use crate::either::Either;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConstRef<'a, T>(&'a mut T);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DynRef<'a, T> {
    ref1: &'a mut T,
    const1: bool,
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
        self.ref1
    }
}

impl<'a, T> ConstRef<'a, T> {
    pub fn new(ref1: &'a mut T) -> Self {
        ConstRef(ref1)
    }

    pub(crate) fn unwrap(self) -> &'a mut T {
        self.0
    }

    pub fn reborrow(&mut self) -> ConstRef<T> {
        ConstRef(self.0)
    }

    pub fn opt_reborrow<'b>(this: &'b mut Option<ConstRef<'a, T>>) -> Option<ConstRef<'b, T>> {
        this.as_mut().map(Self::reborrow)
    }

    pub fn into_dyn(self) -> DynRef<'a, T> {
        DynRef { ref1: self.0, const1: true }
    }
}

impl<'a, T> DynRef<'a, T> {
    pub fn new(ref1: &'a mut T, const1: bool) -> Self {
        DynRef { ref1, const1 }
    }

    #[expect(dead_code)]
    pub(crate) fn unwrap(self) -> &'a mut T {
        self.ref1
    }

    pub fn reborrow(&mut self) -> DynRef<T> {
        DynRef { ref1: self.ref1, const1: self.const1 }
    }

    pub fn opt_reborrow<'b>(this: &'b mut Option<DynRef<'a, T>>) -> Option<DynRef<'b, T>> {
        this.as_mut().map(Self::reborrow)
    }

    pub fn is_const(&self) -> bool {
        self.const1
    }

    pub fn or_const(&mut self, const1: bool) {
        self.const1 |= const1;
    }

    pub fn into_const(self) -> ConstRef<'a, T> {
        ConstRef(self.ref1)
    }

    pub fn into_either(self) -> Either<ConstRef<'a, T>, &'a mut T> {
        if self.const1 { Either::This(ConstRef(self.ref1)) } else { Either::That(self.ref1) }
    }

    pub fn map<S, F>(&mut self, f: F) -> DynRef<S>
    where F: FnOnce(&mut T) -> DynRef<S> {
        let mut ref1 = f(self.ref1);
        ref1.or_const(self.const1);
        ref1
    }

    pub fn map_opt<S, F>(&mut self, f: F) -> Option<DynRef<S>>
    where F: FnOnce(&mut T) -> Option<DynRef<S>> {
        match f(self.ref1) {
            Some(mut ref1) => {
                ref1.or_const(self.const1);
                Some(ref1)
            }
            None => None,
        }
    }

    pub fn map_res<S, F, E>(&mut self, f: F) -> Result<DynRef<S>, E>
    where F: FnOnce(&mut T) -> Result<DynRef<S>, E> {
        match f(self.ref1) {
            Ok(mut ref1) => {
                ref1.or_const(self.const1);
                Ok(ref1)
            }
            Err(e) => Err(e),
        }
    }
}
