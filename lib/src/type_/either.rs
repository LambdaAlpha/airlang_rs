use derive_more::IsVariant;
use derive_more::TryUnwrap;
use derive_more::Unwrap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, IsVariant, Unwrap, TryUnwrap)]
pub enum Either<This, That> {
    This(This),
    That(That),
}
