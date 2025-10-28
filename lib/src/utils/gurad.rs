use std::mem::ManuallyDrop;
use std::ptr;

struct Guard<V, F, G>
where G: FnOnce(V) {
    value: ManuallyDrop<V>,
    function: ManuallyDrop<F>,
    guard: ManuallyDrop<G>,
}

impl<V, W, F, G> Guard<V, F, G>
where
    F: FnOnce(&mut V) -> W,
    G: FnOnce(V),
{
    fn run(mut self) -> W {
        // safety: it is `ManuallyDrop`, which will not be dropped by compiler
        let function = unsafe { ptr::read(&*self.function) };
        function(&mut self.value)
    }
}

impl<V, F, G> Drop for Guard<V, F, G>
where G: FnOnce(V)
{
    fn drop(&mut self) {
        // safety: it is `ManuallyDrop`, which will not be dropped by compiler
        let value = unsafe { ptr::read(&*self.value) };
        // safety: it is `ManuallyDrop`, which will not be dropped by compiler
        let guard = unsafe { ptr::read(&*self.guard) };
        guard(value);
    }
}

pub(crate) fn guard<V, W, F, G>(value: V, function: F, guard: G) -> W
where
    F: FnOnce(&mut V) -> W,
    G: FnOnce(V), {
    Guard {
        value: ManuallyDrop::new(value),
        function: ManuallyDrop::new(function),
        guard: ManuallyDrop::new(guard),
    }
    .run()
}
