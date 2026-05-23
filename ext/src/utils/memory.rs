pub(crate) fn leak_const<T>(t: T) -> &'static T {
    Box::leak(Box::new(t)) as &'static T
}
