#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Unit;

impl From<()> for Unit {
    fn from(_unit: ()) -> Self {
        Unit
    }
}

impl From<Unit> for () {
    fn from(_unit: Unit) -> Self {}
}
