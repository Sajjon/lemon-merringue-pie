mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use bar::prelude::*;
    pub(crate) use e::prelude::*;
    pub(crate) use f::prelude::*;
    pub(crate) use foo::prelude::*;
    pub(crate) use o::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("foobar");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(
            Foobar {
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
                bar2: Bar2 { bar: true }
            },
            Foobar {
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
                bar2: Bar2 { bar: true }
            }
        );
    }
}
