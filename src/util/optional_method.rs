//! An implementation of optional trait methods, sort of.

pub trait OptionalMethod {
    const IS_IMPLEMENTED: bool;
}

pub struct Implemented;

pub struct Unimplemented;

impl OptionalMethod for Implemented {
    const IS_IMPLEMENTED: bool = true;
}

impl OptionalMethod for Unimplemented {
    const IS_IMPLEMENTED: bool = false;
}

macro_rules! declare_optional_method {
    ( $method:ident<$lv:lifetime>($arg:ty) )
        =>
    {
        pub trait $method<$lv, M>: $crate::prelude::OptionalMethod {
            fn build(src: $arg) -> M;
        }

        impl<$lv, M> $method<$lv, M> for $crate::prelude::Unimplemented {
            fn build(_: $arg) -> M {
                unimplemented!(stringify!($method))
            }
        }
    };
}
