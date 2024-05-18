use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
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

impl Default for Foobar {
    fn default() -> Self {
        Self {
            etype: EType { val: true },
            ftype: FType { val: true },
            otype: OType { val: true },
            foo: Foo {
                ftype: FType { val: true },
                otype: OType { val: true },
                otype_2: OType { val: true },
            },
            foo2: Foo2 {
                ftype2: FType2 { val: true },
                otype2: OType2 { val: true },
                otype2_2: OType2 { val: true },
            },
            bar: Bar { bar: true },
            bar2: Bar2 { bar: true },
        }
    }
}

#[uniffi::export]
impl Foobar {
    #[uniffi::constructor]
    pub fn new() -> Foobar {
        Foobar::default()
    }
}
