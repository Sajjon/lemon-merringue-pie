use crate::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Debug, Eq)]
pub struct Foobar {
    pub etype: EType,
    pub ftype: FType,
    pub otype: OType,
    pub foo: Foo,
    pub foo2: Foo2,
    pub bar: Bar,
    pub bar2: Bar2,
}

#[uniffi::export]
impl Foobar {
    #[uniffi::constructor]
    pub fn new() -> Foobar {
        Foobar::default()
    }
}
