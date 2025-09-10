macro_rules! impl_prim_func {
    ($func:ty) => {
        impl $func {
            pub fn id(&self) -> Symbol {
                self.id.clone()
            }
        }

        impl ::std::fmt::Debug for $func {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.id.fmt(f)
            }
        }

        impl ::std::cmp::PartialEq for $func {
            fn eq(&self, other: &$func) -> bool {
                self.id == other.id
            }
        }

        impl ::std::cmp::Eq for $func {}

        impl ::std::hash::Hash for $func {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }
    };
}

pub(super) use impl_prim_func;
