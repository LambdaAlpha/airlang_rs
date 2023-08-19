#[allow(clippy::wrong_self_convention)]
pub(crate) trait OutputBuilder<Output> {
    fn from_pair(&self, first: Output, second: Output) -> Output;

    fn from_list<Iter>(&self, iter: Iter) -> Output
    where
        Iter: Iterator<Item = Output>;

    fn from_map<Iter>(&self, kv_iter: Iter) -> Output
    where
        Iter: Iterator<Item = (Output, Output)>;

    fn from_call(&self, func: Output, input: Output) -> Output;

    fn from_reverse(&self, func: Output, output: Output) -> Output;
}
