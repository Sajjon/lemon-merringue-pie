mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use f::prelude::*;
    pub(crate) use o::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("foo");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(
            Foo {
                ftype: FType { val: true },
                otype: OType { val: true },
                otype_2: OType { val: false }
            },
            Foo {
                ftype: FType { val: true },
                otype: OType { val: true },
                otype_2: OType { val: false }
            }
        );
    }
}
