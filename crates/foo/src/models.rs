use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Foo {
    pub ftype: FType,
    pub otype: OType,
    pub otype_2: OType,
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Foo2 {
    pub ftype2: FType2,
    pub otype2: OType2,
    pub otype2_2: OType2,
}
