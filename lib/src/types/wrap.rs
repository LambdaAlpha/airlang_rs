macro_rules! box_wrap {
    ($visibility:vis $name:ident($value:ty)) => {
        #[derive(std::clone::Clone, std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
        $visibility struct $name(::std::boxed::Box<$value>);

        impl $name {
            #[allow(unused)]
            pub(crate) fn new(value: ::std::boxed::Box<$value>) -> Self {
                Self(value)
            }

            #[allow(unused)]
            pub(crate) fn unwrap(self) -> ::std::boxed::Box<$value> {
                self.0
            }
        }

        impl ::std::convert::From<$value> for $name {
            fn from(value: $value) -> Self {
                Self(::std::boxed::Box::new(value))
            }
        }

        impl ::std::convert::From<$name> for $value {
            fn from(value: $name) -> Self {
                *value.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $value;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <$value as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    };
}

pub(crate) use box_wrap;

macro_rules! rc_wrap {
    ($visibility:vis $name:ident($value:ty)) => {
        #[derive(std::clone::Clone, std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
        $visibility struct $name(::std::rc::Rc<$value>);

        impl $name {
            #[allow(unused)]
            pub(crate) fn new(value: ::std::rc::Rc<$value>) -> Self {
                Self(value)
            }

            #[allow(unused)]
            pub(crate) fn unwrap(self) -> ::std::rc::Rc<$value> {
                self.0
            }
        }

        impl ::std::convert::From<$value> for $name {
            fn from(value: $value) -> Self {
                Self(::std::rc::Rc::new(value))
            }
        }

        impl ::std::convert::From<$name> for $value {
            fn from(value: $name) -> Self {
                ::std::rc::Rc::unwrap_or_clone(value.0)
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $value;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::std::rc::Rc::make_mut(&mut self.0)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <$value as ::std::fmt::Debug>::fmt(self, f)
            }
        }
    };
}

pub(crate) use rc_wrap;
