use std::cell::Cell;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Default)]
pub struct Link<T> {
    // todo impl fix memory leak
    target: Rc<Cell<T>>,
}

impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self { target: Rc::new(Cell::new(value)) }
    }

    pub fn set(&mut self, value: T) -> T {
        self.target.replace(value)
    }
}

impl<T: Copy> Link<T> {
    pub fn get(&self) -> T {
        self.target.get()
    }
}

impl<T: Clone + Default> Link<T> {
    pub fn get_clone(&self) -> T {
        let t = self.target.take();
        let clone = t.clone();
        self.target.set(t);
        clone
    }
}

impl<T: Default> Link<T> {
    pub fn take(&mut self) -> T {
        self.target.take()
    }
}

impl<T> PartialEq for Link<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl<T> Eq for Link<T> {}

impl<T> Hash for Link<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.target).hash(state);
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Self { target: Rc::clone(&self.target) }
    }
}

impl<T> Debug for Link<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Link").field(&Rc::as_ptr(&self.target)).finish()
    }
}
