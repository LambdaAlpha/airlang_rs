use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::Iter;
use std::slice::IterMut;
use std::vec::IntoIter;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct List<T>(Vec<T>);

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    #[expect(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    #[expect(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        List(Vec::from_iter(iter))
    }
}

impl<T> Deref for List<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(value: Vec<T>) -> Self {
        List(value)
    }
}

impl<T> From<List<T>> for Vec<T> {
    fn from(value: List<T>) -> Self {
        value.0
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List(Default::default())
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}
