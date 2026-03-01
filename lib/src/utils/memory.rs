pub(crate) fn leak_const<T>(func: T) -> &'static T {
    Box::leak(Box::new(func)) as &'static T
}
