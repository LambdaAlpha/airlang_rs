macro_rules! dyn_any_debug_clone_eq_hash {
    ($visibility:vis $trait_name:ident : $super_trait:path) => {
        $visibility trait $trait_name: ::std::any::Any + ::std::fmt::Debug + $super_trait {
            fn dyn_eq(&self, other: &dyn $trait_name) -> bool;
            fn dyn_clone(&self) -> ::std::boxed::Box<dyn $trait_name>;
            fn dyn_hash(&self, hasher: &mut dyn ::std::hash::Hasher);
        }

        impl<T> $trait_name for T
        where
            T: $super_trait
                + ::std::any::Any
                + ::std::cmp::Eq
                + ::std::clone::Clone
                + ::std::hash::Hash
                + ::std::fmt::Debug,
        {
            fn dyn_eq(&self, other: &dyn $trait_name) -> bool {
                if let Some(other) = <dyn ::std::any::Any>::downcast_ref(other) {
                    self == other
                } else {
                    false
                }
            }

            fn dyn_clone(&self) -> ::std::boxed::Box<dyn $trait_name> {
                ::std::boxed::Box::new(self.clone())
            }

            fn dyn_hash(&self, mut hasher: &mut dyn ::std::hash::Hasher) {
                self.hash(&mut hasher);
            }
        }

        impl ::std::clone::Clone for ::std::boxed::Box<dyn $trait_name> {
            fn clone(&self) -> Self {
                (**self).dyn_clone()
            }
        }

        impl ::std::cmp::PartialEq for dyn $trait_name {
            fn eq(&self, other: &Self) -> bool {
                self.dyn_eq(other)
            }
        }

        // https://github.com/rust-lang/rust/issues/31740
        impl ::std::cmp::PartialEq<&Self> for ::std::boxed::Box<dyn $trait_name> {
            fn eq(&self, other: &&Self) -> bool {
                <Self as ::std::cmp::PartialEq>::eq(self, *other)
            }
        }

        impl ::std::cmp::Eq for dyn $trait_name {}

        impl ::std::hash::Hash for dyn $trait_name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.dyn_hash(state);
            }
        }
    };
}

pub(crate) use dyn_any_debug_clone_eq_hash;
