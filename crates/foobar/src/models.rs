use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Foobar {
    pub foo: Foo,
    pub foo2: Foo2,
    pub bar: Bar,
    pub bar2: Bar2,
}
