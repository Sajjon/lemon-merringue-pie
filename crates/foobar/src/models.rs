use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Foobar {
    pub etype: EType,
    pub ftype: FType,
    pub otype: OType,
    pub foo: Foo,
    pub foo2: Foo2,
    pub bar: Bar,
    pub bar2: Bar2,
}
