#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Unit;

impl From<&()> for Unit {
    fn from(_: &()) -> Self {
        Unit
    }
}

impl From<()> for Unit {
    fn from(_: ()) -> Self {
        Unit
    }
}

impl From<Unit> for () {
    fn from(_: Unit) -> Self {}
}

impl From<&Unit> for () {
    fn from(_: &Unit) -> Self {}
}
