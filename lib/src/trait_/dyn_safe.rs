macro_rules! dyn_any_fmt_clone_eq {
    ($visibility:vis $trait_name:ident : $super_trait:path) => {
        $visibility trait $trait_name: ::std::any::Any + ::std::fmt::Debug + ::std::fmt::Display + $super_trait {
            fn dyn_eq(&self, other: &dyn $trait_name) -> bool;
            fn dyn_clone(&self) -> ::std::boxed::Box<dyn $trait_name>;
        }

        impl<T> $trait_name for T
        where
            T: $super_trait
                + ::std::any::Any
                + ::std::cmp::Eq
                + ::std::clone::Clone
                + ::std::fmt::Debug
                + ::std::fmt::Display,
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
    };
}

pub(crate) use dyn_any_fmt_clone_eq;
