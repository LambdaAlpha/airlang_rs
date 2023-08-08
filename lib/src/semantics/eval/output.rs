pub(crate) trait OutputBuilder<Output> {
    fn from_pair(first: Output, second: Output) -> Output;

    fn from_list<Iter>(iter: Iter) -> Output
    where
        Iter: Iterator<Item = Output>;

    fn from_map<Iter>(kv_iter: Iter) -> Output
    where
        Iter: Iterator<Item = (Output, Output)>;

    fn from_call(func: Output, input: Output) -> Output;

    fn from_reverse(func: Output, output: Output) -> Output;
}
