macro_rules! derive_display {
    ($name:ident($value:ty)) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <$value as ::std::fmt::Display>::fmt(self, f)
            }
        }
    };
}

pub(crate) use derive_display;

macro_rules! derive_debug {
    ($name:ident($value:ty)) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <$value as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    };
}

pub(crate) use derive_debug;
